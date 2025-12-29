//! Unit training operations for the Empire game.
//!
//! This module provides core functionality for managing unit training, including
//! queue management, resource validation, faction bonus application, and job scheduling.
//! It follows the functional programming approach with direct function calls rather than
//! service structs, enabling better performance through single-connection-per-request optimization.
//!
//! All operations maintain transactional integrity and provide comprehensive validation
//! of resource requirements, building constraints, and queue capacity limits.

use std::ops::Add;

use bigdecimal::ToPrimitive;
use chrono::{TimeDelta, Utc};
use diesel::Connection;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument, trace, warn};
use uuid::Uuid;

use crate::db::{
	DbConn, building_unit_types, player_buildings, player_units, resources, training_queue,
	unit_costs, units,
};
use crate::domain::error::{Error, ErrorKind, Result};
use crate::domain::jobs::{JobKey, JobType};
use crate::domain::modifier::ModifierTarget;
use crate::domain::player::PlayerKey;
use crate::domain::player::buildings::PlayerBuildingKey;
use crate::domain::unit::training::{
	NewTrainingQueueEntry, TrainingQueueEntry, TrainingQueueKey, TrainingStatus,
};
use crate::domain::unit::{Unit, UnitKey};
use crate::game::modifiers::modifier_operations;
use crate::job_queue::{JobPriority, JobQueue};

/// Maximum number of concurrent training entries per building
pub const MAX_QUEUE_PER_BUILDING: i64 = 5;

/// Refund percentage when cancelling training (80% = 0.80)
pub const CANCEL_REFUND_RATE: f64 = 0.80;

/// Job payload for training completion jobs.
///
/// This is serialized to JSON and stored in the job table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingJobPayload {
	pub training_queue_entry_id: Uuid,
	pub player_id: Uuid,
	pub unit_id: Uuid,
	pub quantity: i32,
}

/// Starts training units at a specified building.
///
/// # Validation
/// - Building must be owned by the player
/// - Building must be capable of training the specified unit type
/// - Player must have sufficient resources
/// - Quantity must be positive
/// - Building's training queue must not be full (max 5)
///
/// # Returns
/// The created TrainingQueueEntry with associated job scheduled
#[instrument(skip(conn, job_queue))]
pub fn start_training(
	conn: &mut DbConn,
	job_queue: &JobQueue,
	player_id: &PlayerKey,
	building_id: &PlayerBuildingKey,
	unit_id: &UnitKey,
	quantity: i32,
) -> Result<TrainingQueueEntry> {
	debug!(
		"Starting training for player {} at building {}: unit {} x {}",
		player_id, building_id, unit_id, quantity
	);

	// Validate quantity
	if quantity <= 0 {
		return Err(Error::from((
			ErrorKind::InvalidQuantityError,
			"Quantity must be positive",
		)));
	}

	// Validate building ownership
	let player_bld = validate_building_ownership(conn, player_id, building_id)?;
	trace!("Building ownership validated: {:?}", player_bld);

	// Get unit details
	let unit = units::get_by_id(conn, unit_id)?;
	trace!("Unit details: {:?}", unit);

	// Validate building can train this unit type
	validate_building_unit_match(conn, &player_bld.building_id, &unit)?;
	trace!("Building-unit type match validated");

	// Check resources (before transaction to fail fast)
	if !has_enough_resources(conn, player_id, unit_id, quantity)? {
		return Err(Error::from((
			ErrorKind::InsufficientResourcesError,
			"Not enough resources",
		)));
	}
	trace!("Resource check passed");

	// Calculate training duration with faction bonuses
	let duration = calculate_training_duration(conn, player_id, &unit, quantity)?;
	let completion_time = Utc::now().add(duration);
	trace!(
		"Training duration: {:?}, completion at: {}",
		duration, completion_time
	);

	// Execute transaction: check queue capacity (with lock), deduct resources, create entry
	let res: Result<TrainingQueueEntry> = conn.transaction(|connection| {
		info!("Initiating training transaction");

		// Check queue capacity with row-level locking to prevent race conditions
		// AIDEV-NOTE: FOR UPDATE lock serializes concurrent requests for the same building
		let active_count =
			training_queue::get_active_count_for_building_locked(connection, building_id)?;
		if active_count >= MAX_QUEUE_PER_BUILDING {
			return Err(Error::from((
				ErrorKind::TrainingQueueFullError,
				"Training queue is full for this building",
			)));
		}
		trace!(
			"Queue capacity check passed: {}/{}",
			active_count, MAX_QUEUE_PER_BUILDING
		);

		// Deduct resources
		let costs = get_total_cost(connection, unit_id, quantity)?;
		resources::deduct(connection, player_id, &costs)?;
		trace!("Resources deducted: {:?}", costs);

		// Create training queue entry
		let new_entry = NewTrainingQueueEntry {
			player_id: *player_id,
			building_id: *building_id,
			unit_id: *unit_id,
			quantity,
			status: Some(TrainingStatus::InProgress),
			job_id: None, // Will be set after job is scheduled
		};
		let entry = training_queue::create(connection, new_entry)?;
		trace!("Training queue entry created: {:?}", entry);

		Ok(entry)
	});

	let mut entry = res.map_err(|e| {
		warn!("Failed to start training: {}", e);
		Error::from((
			ErrorKind::StartTrainingError,
			"Failed to start training",
			format!("{:?}", e),
		))
	})?;

	// Schedule completion job (outside transaction to avoid holding locks)
	let payload = TrainingJobPayload {
		training_queue_entry_id: entry.id,
		player_id: *player_id,
		unit_id: *unit_id,
		quantity,
	};
	let job_id = job_queue.enqueue(
		JobType::Training,
		payload,
		JobPriority::Normal,
		completion_time,
	)?;
	trace!("Scheduled training job: {}", job_id);

	// Link job to training entry
	entry = training_queue::set_job_id(conn, &entry.id, &job_id)?;

	info!(
		"Successfully started training for player {}: {} x {} units",
		player_id, quantity, unit.name
	);
	Ok(entry)
}

/// Cancels an in-progress or pending training entry.
///
/// # Refund Calculation
/// - Refunds 80% of resources based on remaining time
/// - If training has not started (Pending): full 80% refund
/// - If 50% complete: 40% refund (80% * 50%)
///
/// # Returns
/// The updated TrainingQueueEntry with Cancelled status
#[instrument(skip(conn, job_queue))]
pub fn cancel_training(
	conn: &mut DbConn,
	job_queue: &JobQueue,
	player_id: &PlayerKey,
	entry_id: &TrainingQueueKey,
) -> Result<TrainingQueueEntry> {
	debug!("Cancelling training {} for player {}", entry_id, player_id);

	// Get training entry
	let entry = training_queue::get_by_id(conn, entry_id)?;

	// Validate ownership
	if &entry.player_id != player_id {
		return Err(Error::from((
			ErrorKind::CancelTrainingError,
			"Training entry not found",
		)));
	}

	// Check if cancellable
	if entry.status == TrainingStatus::Completed || entry.status == TrainingStatus::Cancelled {
		return Err(Error::from((
			ErrorKind::CancelTrainingError,
			"Training cannot be cancelled",
		)));
	}

	// Calculate refund
	let refund = calculate_refund(conn, &entry)?;
	trace!("Calculated refund: {:?}", refund);

	// Execute transaction
	let res: Result<TrainingQueueEntry> = conn.transaction(|connection| {
		// Refund resources
		if refund.0 > 0 || refund.1 > 0 || refund.2 > 0 || refund.3 > 0 {
			resources::add(connection, player_id, &refund)?;
			trace!("Refunded resources");
		}

		// Cancel entry
		training_queue::cancel(connection, entry_id)?;
		let cancelled = training_queue::get_by_id(connection, entry_id)?;
		trace!("Training entry cancelled: {:?}", cancelled);

		Ok(cancelled)
	});

	let cancelled_entry = res.map_err(|e| {
		warn!("Failed to cancel training {}: {}", entry_id, e);
		Error::from((
			ErrorKind::CancelTrainingError,
			"Failed to cancel training",
			format!("{:?}", e),
		))
	})?;

	// Cancel the associated job if one exists
	if let Some(job_id) = entry.job_id {
		match job_queue.cancel_job(&job_id) {
			Ok(true) => trace!("Cancelled job {}", job_id),
			Ok(false) => trace!("Job {} was not pending, may have already started", job_id),
			Err(e) => warn!("Failed to cancel job {}: {}", job_id, e),
		}
	}

	Ok(cancelled_entry)
}

/// Completes a training entry and adds units to player inventory.
///
/// Called by the job processor when training time has elapsed.
/// This function is idempotent - calling it multiple times is safe.
///
/// # Returns
/// The updated TrainingQueueEntry with Completed status
#[instrument(skip(conn))]
pub fn complete_training(conn: &mut DbConn, job_id: &JobKey) -> Result<TrainingQueueEntry> {
	debug!("Completing training for job {}", job_id);

	// Get training entry by job ID
	let entry = training_queue::get_by_job_id(conn, job_id)?;

	// Check if already completed (idempotent)
	if entry.status == TrainingStatus::Completed {
		debug!("Training {} already completed, skipping", entry.id);
		return Ok(entry);
	}

	// Check if cancelled
	if entry.status == TrainingStatus::Cancelled {
		debug!("Training {} was cancelled, skipping", entry.id);
		return Ok(entry);
	}

	// Execute transaction
	let res: Result<TrainingQueueEntry> = conn.transaction(|connection| {
		// Add units to player inventory
		player_units::add_units(connection, &entry.player_id, &entry.unit_id, entry.quantity)?;
		trace!(
			"Added {} units to player {} inventory",
			entry.quantity, entry.player_id
		);

		// Mark training as completed
		let completed = training_queue::complete(connection, &entry.id)?;
		trace!("Training entry completed: {:?}", completed);

		Ok(completed)
	});

	res.map_err(|e| {
		warn!("Failed to complete training for job {}: {}", job_id, e);
		Error::from((
			ErrorKind::CompleteTrainingError,
			"Failed to complete training",
			format!("{:?}", e),
		))
	})
}

/// Gets all active (Pending/InProgress) training entries for a player.
#[instrument(skip(conn))]
pub fn get_training_queue(
	conn: &mut DbConn,
	player_id: &PlayerKey,
) -> Result<Vec<TrainingQueueEntry>> {
	let entries = training_queue::get_active_for_player(conn, player_id)?;
	trace!(
		"Found {} active training entries for player {}",
		entries.len(),
		player_id
	);
	Ok(entries)
}

/// Gets all units that can be trained at a specific building.
///
/// Validates building ownership and returns available unit types.
#[instrument(skip(conn))]
pub fn get_available_units_for_building(
	conn: &mut DbConn,
	player_id: &PlayerKey,
	building_id: &PlayerBuildingKey,
) -> Result<Vec<Unit>> {
	debug!(
		"Getting available units for building {} player {}",
		building_id, player_id
	);

	// Validate building ownership
	let player_bld = validate_building_ownership(conn, player_id, building_id)?;

	// Get unit types trainable at this building
	let unit_types =
		building_unit_types::get_unit_types_for_building(conn, &player_bld.building_id)?;
	trace!(
		"Building {} can train unit types: {:?}",
		player_bld.building_id, unit_types
	);

	// Get all units of those types
	let mut available_units = Vec::new();
	for utype in unit_types {
		let type_units = units::get_by_type(conn, &utype)?;
		available_units.extend(type_units);
	}

	trace!("Found {} available units", available_units.len());
	Ok(available_units)
}

// === Internal Helper Functions ===

/// Validates building ownership by player.
fn validate_building_ownership(
	conn: &mut DbConn,
	player_id: &PlayerKey,
	building_id: &PlayerBuildingKey,
) -> Result<crate::domain::player::buildings::PlayerBuilding> {
	let player_bld = player_buildings::get_by_id(conn, building_id)?;

	if &player_bld.player_id != player_id {
		return Err(Error::from((
			ErrorKind::StartTrainingError,
			"Building not found",
		)));
	}

	Ok(player_bld)
}

/// Validates that a building can train the specified unit type.
fn validate_building_unit_match(
	conn: &mut DbConn,
	building_id: &crate::domain::building::BuildingKey,
	unit: &Unit,
) -> Result<()> {
	let can_train = building_unit_types::can_train_unit(conn, building_id, &unit.unit_type)?;

	if !can_train {
		return Err(Error::from((
			ErrorKind::InvalidBuildingTypeError,
			"Building cannot train this unit type",
		)));
	}

	Ok(())
}

/// Checks if player has enough resources for training.
fn has_enough_resources(
	conn: &mut DbConn,
	player_id: &PlayerKey,
	unit_id: &UnitKey,
	quantity: i32,
) -> Result<bool> {
	let costs = get_total_cost(conn, unit_id, quantity)?;
	let player_res = resources::get_by_player_id(conn, player_id)?;

	let has_food = player_res.food >= costs.0;
	let has_wood = player_res.wood >= costs.1;
	let has_stone = player_res.stone >= costs.2;
	let has_gold = player_res.gold >= costs.3;

	Ok(has_food && has_wood && has_stone && has_gold)
}

/// Gets the total resource cost for training units.
fn get_total_cost(
	conn: &mut DbConn,
	unit_id: &UnitKey,
	quantity: i32,
) -> Result<(i64, i64, i64, i64)> {
	let costs = unit_costs::get_by_unit(conn, unit_id)?;

	let mut food: i64 = 0;
	let mut wood: i64 = 0;
	let mut stone: i64 = 0;
	let mut gold: i64 = 0;

	for cost in costs {
		match cost.resource.as_str() {
			"food" => food = cost.amount as i64 * quantity as i64,
			"wood" => wood = cost.amount as i64 * quantity as i64,
			"stone" => stone = cost.amount as i64 * quantity as i64,
			"gold" => gold = cost.amount as i64 * quantity as i64,
			_ => {} // Ignore unknown resource types
		}
	}

	Ok((food, wood, stone, gold))
}

/// Calculates training duration with faction modifiers applied.
///
/// AIDEV-NOTE: Modifier < 1.0 means faster training (e.g., Goblin 0.8 = 20% faster)
fn calculate_training_duration(
	conn: &mut DbConn,
	player_id: &PlayerKey,
	unit: &Unit,
	quantity: i32,
) -> Result<TimeDelta> {
	// Get base training time per unit
	let base_seconds = unit.base_training_seconds as i64;

	// Get training speed modifier
	let modifier = modifier_operations::calc_multiplier(
		conn,
		player_id,
		ModifierTarget::Training,
		None, // No specific resource target for training
	)?;

	// Apply modifier: lower value = faster training
	let modifier_f64 = modifier.to_f64().unwrap_or(1.0);
	let modified_seconds = (base_seconds as f64 * modifier_f64) as i64;

	// Total time = per_unit_time * quantity
	let total_seconds = modified_seconds * quantity as i64;

	Ok(TimeDelta::seconds(total_seconds))
}

/// Calculates refund amount based on remaining time.
///
/// Uses the same faction-modified training time as the original duration calculation
/// to ensure consistent refund ratios.
///
/// Returns tuple of (food, wood, stone, gold) to refund.
fn calculate_refund(conn: &mut DbConn, entry: &TrainingQueueEntry) -> Result<(i64, i64, i64, i64)> {
	let costs = get_total_cost(conn, &entry.unit_id, entry.quantity)?;

	// If still pending (not started), give full refund rate
	let remaining_ratio = if entry.status == TrainingStatus::Pending {
		1.0
	} else {
		// Calculate based on elapsed time vs expected duration (with faction modifiers)
		let unit = units::get_by_id(conn, &entry.unit_id)?;

		// Apply faction modifier to match original duration calculation
		let modifier = modifier_operations::calc_multiplier(
			conn,
			&entry.player_id,
			ModifierTarget::Training,
			None,
		)?;
		let modifier_f64 = modifier.to_f64().unwrap_or(1.0);
		let total_seconds =
			unit.base_training_seconds as f64 * modifier_f64 * entry.quantity as f64;

		let elapsed_seconds = (Utc::now() - entry.started_at).num_seconds() as f64;

		let ratio = 1.0 - (elapsed_seconds / total_seconds);
		ratio.max(0.0) // Don't go negative
	};

	let refund_ratio = CANCEL_REFUND_RATE * remaining_ratio;

	let food = (costs.0 as f64 * refund_ratio) as i64;
	let wood = (costs.1 as f64 * refund_ratio) as i64;
	let stone = (costs.2 as f64 * refund_ratio) as i64;
	let gold = (costs.3 as f64 * refund_ratio) as i64;

	Ok((food, wood, stone, gold))
}
