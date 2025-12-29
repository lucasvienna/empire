//! Request handlers for the units API endpoints.
//!
//! Provides handlers for unit training operations including listing available units,
//! starting training, viewing the queue, cancelling, and checking inventory.

use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json, debug_handler};
use bigdecimal::ToPrimitive;
use chrono::{TimeDelta, Utc};
use tracing::{debug, info, instrument, trace};

use crate::Result;
use crate::controllers::game::units::models::*;
use crate::db::extractor::DatabaseConnection;
use crate::db::{player_units, resources, training_queue, unit_costs, units};
use crate::domain::app_state::{AppQueue, AppState};
use crate::domain::auth::AuthenticatedUser;
use crate::domain::modifier::ModifierTarget;
use crate::domain::unit::training::TrainingQueueKey;
use crate::game::modifiers::modifier_operations;
use crate::game::units::training_operations;

/// GET /game/units/available?building_id={uuid}
///
/// Returns all units that can be trained at the specified building, enriched with
/// cost information, faction-modified training times, and affordability calculations.
#[instrument(skip(conn, player))]
#[debug_handler(state = AppState)]
pub async fn get_available_units(
	DatabaseConnection(mut conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
	Query(query): Query<AvailableUnitsQuery>,
) -> Result<impl IntoResponse> {
	let player_id = player.id;
	debug!(
		"Getting available units for building {} player {}",
		query.building_id, player_id
	);

	// Get available units for this building (validates ownership)
	let available_units = training_operations::get_available_units_for_building(
		&mut conn,
		&player_id,
		&query.building_id,
	)?;

	// Get player's current resources for affordability calculation
	let player_res = resources::get_by_player_id(&mut conn, &player_id)?;

	// Get training speed modifier for this player
	// AIDEV-NOTE: Modifier < 1.0 means faster training (e.g., Goblin 0.8 = 20% faster)
	let training_modifier =
		modifier_operations::calc_multiplier(&mut conn, &player_id, ModifierTarget::Training, None)
			.map(|m| m.to_f64().unwrap_or(1.0))
			.unwrap_or(1.0);

	// Batch fetch all unit costs to avoid N+1 query problem
	let unit_ids: Vec<_> = available_units.iter().map(|u| u.id).collect();
	let all_costs = unit_costs::get_all_by_unit(&mut conn, &unit_ids)?;
	let costs_map: HashMap<_, Vec<_>> =
		all_costs.into_iter().fold(HashMap::new(), |mut map, cost| {
			map.entry(cost.unit_id).or_default().push(cost);
			map
		});

	// Enrich each unit with costs and affordability
	let mut unit_dtos = Vec::with_capacity(available_units.len());
	for unit in &available_units {
		let costs = costs_map.get(&unit.id).map(|v| v.as_slice()).unwrap_or(&[]);

		// Convert costs to our DTO format
		let mut cost_dto = UnitCostDto::default();
		for cost in costs {
			match cost.resource.as_str() {
				"food" => cost_dto.food = cost.amount,
				"wood" => cost_dto.wood = cost.amount,
				"stone" => cost_dto.stone = cost.amount,
				"gold" => cost_dto.gold = cost.amount,
				_ => {}
			}
		}

		// Calculate affordability
		let can_afford = player_res.food >= cost_dto.food
			&& player_res.wood >= cost_dto.wood
			&& player_res.stone >= cost_dto.stone
			&& player_res.gold >= cost_dto.gold;

		// Calculate max affordable quantity
		let max_affordable = if !can_afford {
			0
		} else {
			let mut max = i64::MAX;
			if cost_dto.food > 0 {
				max = max.min(player_res.food / cost_dto.food);
			}
			if cost_dto.wood > 0 {
				max = max.min(player_res.wood / cost_dto.wood);
			}
			if cost_dto.stone > 0 {
				max = max.min(player_res.stone / cost_dto.stone);
			}
			if cost_dto.gold > 0 {
				max = max.min(player_res.gold / cost_dto.gold);
			}
			max
		};

		// Calculate modified training time
		let modified_training_seconds =
			(unit.base_training_seconds as f64 * training_modifier).round() as i32;

		let dto = AvailableUnitDto {
			id: unit.id,
			name: unit.name.clone(),
			unit_type: unit.unit_type,
			base_atk: unit.base_atk,
			base_def: unit.base_def,
			description: unit.description.clone(),
			base_training_seconds: unit.base_training_seconds,
			modified_training_seconds,
			training_modifier,
			cost: cost_dto,
			can_afford,
			max_affordable,
		};
		unit_dtos.push(dto);
	}

	trace!("Found {} available units", unit_dtos.len());
	info!(
		"Retrieved {} available units for building {} player {}",
		unit_dtos.len(),
		query.building_id,
		player_id
	);

	Ok(Json(AvailableUnitsResponse {
		building_id: query.building_id,
		units: unit_dtos,
	}))
}

/// POST /game/units/train
///
/// Starts training units at a building. Validates resources, queue capacity,
/// and building ownership before creating the training entry.
#[instrument(skip(conn, job_queue, player))]
#[debug_handler(state = AppState)]
pub async fn train_units(
	DatabaseConnection(mut conn): DatabaseConnection,
	State(job_queue): State<AppQueue>,
	player: Extension<AuthenticatedUser>,
	Json(request): Json<TrainUnitsRequest>,
) -> Result<impl IntoResponse> {
	let player_id = player.id;
	debug!(
		"Starting training for player {}: unit {} x {} at building {}",
		player_id, request.unit_id, request.quantity, request.building_id
	);

	// Get the unit details for the response
	let unit = units::get_by_id(&mut conn, &request.unit_id)?;

	// Get costs for the response
	let costs = unit_costs::get_by_unit(&mut conn, &request.unit_id).unwrap_or_default();
	let mut cost_dto = UnitCostDto::default();
	for cost in &costs {
		match cost.resource.as_str() {
			"food" => cost_dto.food = cost.amount * request.quantity,
			"wood" => cost_dto.wood = cost.amount * request.quantity,
			"stone" => cost_dto.stone = cost.amount * request.quantity,
			"gold" => cost_dto.gold = cost.amount * request.quantity,
			_ => {}
		}
	}

	// Start training via service layer (errors have proper status codes via IntoResponse)
	// AIDEV-NOTE: completion_time is returned from start_training to ensure consistency
	// between API response and actual job scheduling (avoids rounding discrepancies)
	let (entry, completion_time) = training_operations::start_training(
		&mut conn,
		&job_queue,
		&player_id,
		&request.building_id,
		&request.unit_id,
		request.quantity,
	)?;

	// Calculate total_seconds from the authoritative completion_time
	let total_seconds = (completion_time - entry.started_at).num_seconds();

	info!(
		"Started training for player {}: {} x {} units, completes at {}",
		player_id, request.quantity, unit.name, completion_time
	);

	Ok((
		StatusCode::CREATED,
		Json(TrainUnitsResponse {
			training_id: entry.id,
			unit_id: entry.unit_id,
			unit_name: unit.name,
			quantity: entry.quantity,
			started_at: entry.started_at,
			completion_time,
			total_training_seconds: total_seconds,
			resources_spent: cost_dto,
		}),
	))
}

/// GET /game/units/queue
///
/// Returns the player's active training queue with progress calculations
/// for each entry. Entries are sorted by start time.
#[instrument(skip(conn, player))]
#[debug_handler(state = AppState)]
pub async fn get_training_queue(
	DatabaseConnection(mut conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	let player_id = player.id;
	debug!("Getting training queue for player {}", player_id);

	// Get active training entries
	let entries = training_queue::get_active_for_player(&mut conn, &player_id)?;

	// Batch fetch all units to avoid N+1 query problem
	let unit_ids: Vec<_> = entries.iter().map(|e| e.unit_id).collect();
	let units_list = units::get_all_by_id(&mut conn, &unit_ids)?;
	let units_map: HashMap<_, _> = units_list.into_iter().map(|u| (u.id, u)).collect();

	// Get training modifier for progress calculations
	// AIDEV-NOTE: Progress uses faction-modified duration, not base duration
	let training_modifier =
		modifier_operations::calc_multiplier(&mut conn, &player_id, ModifierTarget::Training, None)
			.map(|m| m.to_f64().unwrap_or(1.0))
			.unwrap_or(1.0);

	let now = Utc::now();
	let mut entry_dtos = Vec::with_capacity(entries.len());

	for entry in &entries {
		// Get unit details from pre-fetched map
		let unit = match units_map.get(&entry.unit_id) {
			Some(u) => u,
			None => continue, // Skip entries with missing units
		};

		// Calculate total training time with modifier
		let total_seconds =
			(unit.base_training_seconds as f64 * training_modifier * entry.quantity as f64) as i64;
		let estimated_completion = entry.started_at + TimeDelta::seconds(total_seconds);

		// Calculate progress
		let elapsed_seconds = (now - entry.started_at).num_seconds().max(0);
		let progress_percent = if total_seconds > 0 {
			((elapsed_seconds as f64 / total_seconds as f64) * 100.0).min(100.0)
		} else {
			100.0
		};
		let seconds_remaining = (total_seconds - elapsed_seconds).max(0);

		let dto = TrainingQueueEntryDto {
			id: entry.id,
			building_id: entry.building_id,
			unit_id: entry.unit_id,
			unit_name: unit.name.clone(),
			unit_type: unit.unit_type,
			quantity: entry.quantity,
			started_at: entry.started_at,
			status: entry.status,
			estimated_completion,
			progress_percent,
			seconds_remaining,
		};
		entry_dtos.push(dto);
	}

	// Sort by started_at for consistent ordering
	entry_dtos.sort_by(|a, b| a.started_at.cmp(&b.started_at));

	let total = entry_dtos.len();
	trace!("Found {} training queue entries", total);
	info!(
		"Retrieved {} training queue entries for player {}",
		total, player_id
	);

	Ok(Json(TrainingQueueResponse {
		entries: entry_dtos,
		total_entries: total,
	}))
}

/// DELETE /game/units/queue/{training_id}
///
/// Cancels an in-progress or pending training entry and refunds a portion
/// of the resources based on remaining time.
#[instrument(skip(conn, job_queue, player))]
#[debug_handler(state = AppState)]
pub async fn cancel_training(
	DatabaseConnection(mut conn): DatabaseConnection,
	State(job_queue): State<AppQueue>,
	player: Extension<AuthenticatedUser>,
	Path(training_id): Path<TrainingQueueKey>,
) -> Result<impl IntoResponse> {
	let player_id = player.id;
	debug!(
		"Cancelling training {} for player {}",
		training_id, player_id
	);

	// Cancel training via service layer (returns entry and refund tuple)
	let (cancelled_entry, refund) =
		training_operations::cancel_training(&mut conn, &job_queue, &player_id, &training_id)?;

	info!(
		"Cancelled training {} for player {}, refunded {:?}",
		training_id, player_id, refund
	);

	Ok(Json(CancelTrainingResponse {
		training_id: cancelled_entry.id,
		status: cancelled_entry.status,
		refunded: UnitCostDto::from_tuple(refund),
	}))
}

/// GET /game/units/inventory
///
/// Returns all units owned by the player with their quantities.
#[instrument(skip(conn, player))]
#[debug_handler(state = AppState)]
pub async fn get_player_inventory(
	DatabaseConnection(mut conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	let player_id = player.id;
	debug!("Getting unit inventory for player {}", player_id);

	// Get all player units
	let player_units_list = player_units::get_for_player(&mut conn, &player_id)?;

	// Batch fetch all units to avoid N+1 query problem
	let unit_ids: Vec<_> = player_units_list.iter().map(|pu| pu.unit_id).collect();
	let units_list = units::get_all_by_id(&mut conn, &unit_ids)?;
	let units_map: HashMap<_, _> = units_list.into_iter().map(|u| (u.id, u)).collect();

	let mut unit_dtos = Vec::with_capacity(player_units_list.len());
	let mut total_units: i64 = 0;

	for pu in &player_units_list {
		// Get unit details from pre-fetched map
		let unit = match units_map.get(&pu.unit_id) {
			Some(u) => u,
			None => continue, // Skip entries with missing units
		};

		total_units += pu.quantity;

		let dto = PlayerUnitDto {
			unit_id: pu.unit_id,
			unit_name: unit.name.clone(),
			unit_type: unit.unit_type,
			quantity: pu.quantity,
		};
		unit_dtos.push(dto);
	}

	trace!("Found {} unit types in inventory", unit_dtos.len());
	info!(
		"Retrieved {} unit types ({} total units) for player {}",
		unit_dtos.len(),
		total_units,
		player_id
	);

	Ok(Json(PlayerUnitsResponse {
		units: unit_dtos,
		total_units,
	}))
}
