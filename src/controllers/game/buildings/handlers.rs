use std::collections::HashMap;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json, debug_handler};
use axum_extra::json;
use tracing::{debug, info, instrument, trace};

use crate::Result;
use crate::controllers::game::buildings::models::{
	BuildingDefinition, BuildingLevelInfo, ConstructBuildingRequest, GameBuilding,
	LevelRequirement, ResourceCapacity, ResourceCosts, ResourceProduction,
};
use crate::db::building_requirements::get_construction_reqs;
use crate::db::extractor::DatabaseConnection;
use crate::db::player_buildings::get_player_bld_counts_levels;
use crate::db::{building_requirements, building_unit_types, buildings, player_buildings};
use crate::domain::app_state::AppState;
use crate::domain::auth::AuthenticatedUser;
use crate::domain::building::BuildingKey;
use crate::domain::building::level::BuildingLevelKey;
use crate::domain::player::buildings::PlayerBuildingKey;
use crate::domain::unit::UnitType;
use crate::game::buildings::building_operations;
use crate::game::buildings::requirement_operations::gen_avail_list;

#[instrument(skip(conn, player))]
#[debug_handler(state = AppState)]
pub async fn get_player_buildings(
	DatabaseConnection(mut conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
) -> impl IntoResponse {
	let player_key = player.id;
	debug!("Getting buildings for player: {}", player_key);
	let buildings =
		player_buildings::get_game_buildings(&mut conn, &player_key).unwrap_or_default();
	trace!("Found {} buildings for player", buildings.len());
	let body: Vec<GameBuilding> = buildings.into_iter().map(GameBuilding::from).collect();
	info!(
		"Retrieved {} buildings for player: {}",
		body.len(),
		player_key
	);
	json!(body)
}

#[instrument(skip(conn, player))]
#[debug_handler(state = AppState)]
pub async fn get_player_building(
	DatabaseConnection(mut conn): DatabaseConnection,
	player_building_key: Path<PlayerBuildingKey>,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse, StatusCode> {
	let player_key = player.id;
	let building_key = player_building_key.0;
	debug!(
		"Getting building {:?} for player: {}",
		building_key, player_key
	);

	let result = player_buildings::get_game_building(&mut conn, &player_key, &building_key);
	if let Err(e) = &result {
		debug!("Building not found: {}", e);
		return Err(StatusCode::NOT_FOUND);
	}

	let building = result.unwrap();
	trace!("Found building details: {:?}", building);

	let game_bld = GameBuilding::from(building);
	info!(
		"Retrieved building {:?} for player: {}",
		building_key, player_key
	);

	Ok(json!(game_bld))
}

/// Returns the full building catalog with availability metadata.
#[instrument(skip_all)]
#[debug_handler(state = AppState)]
pub async fn get_available_buildings(
	DatabaseConnection(mut conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	// Requirements
	// - Player faction == building faction (or neutral)
	// - Current amount < maximum building count
	// - Prerequisites fulfilled (Keep level, tech tree node, etc)

	debug!(
		"Getting available buildings for faction: {}",
		&player.faction
	);
	let (blds, bld_data) = get_player_bld_counts_levels(&mut conn, &player)?;
	let reqs = get_construction_reqs(&mut conn, &player.faction)?;
	let mut avail = gen_avail_list(blds, bld_data, reqs);
	avail.sort_by_key(|a| a.building.id);

	Ok(Json(avail))
}

#[instrument(skip_all)]
#[debug_handler(state = AppState)]
pub async fn construct_player_building(
	DatabaseConnection(mut conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
	Json(bld_req): Json<ConstructBuildingRequest>,
) -> Result<impl IntoResponse> {
	let player_key = player.id;
	let bld_key = bld_req.building_id;
	debug!(
		"Starting building {} construction for player {}",
		bld_key, player_key
	);

	let bld = building_operations::construct_building(&mut conn, &player_key, &bld_key)?;
	trace!("Building construction details: {:?}", bld);

	let res = player_buildings::get_game_building(&mut conn, &player_key, &bld.id)
		.map(GameBuilding::from)?;

	info!(
		"Successfully constructed building {:?} for player {}",
		bld_key, player_key
	);

	Ok(json!(res))
}

#[instrument(skip(conn, player))]
#[debug_handler(state = AppState)]
pub async fn upgrade_building(
	DatabaseConnection(mut conn): DatabaseConnection,
	player_bld_key: Path<PlayerBuildingKey>,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	let player_key = player.id;
	let building_key = player_bld_key.0;
	debug!(
		"Starting upgrade building {:?} for player {}",
		building_key, player_key
	);

	let bld = building_operations::upgrade_building(&mut conn, &building_key)?;
	trace!("Building upgrade details: {:?}", bld);

	let upgrade_time = bld.upgrade_finishes_at.unwrap_or_default();
	debug!("Building upgrade will be ready at {}", upgrade_time);

	let res = player_buildings::get_game_building(&mut conn, &player_key, &bld.id)
		.map(GameBuilding::from)?;

	info!(
		"Successfully started upgrading building {:?} for player {}",
		building_key, player_key
	);

	Ok(json!(res))
}

#[instrument(skip_all)]
#[debug_handler(state = AppState)]
pub async fn confirm_upgrade(
	DatabaseConnection(mut conn): DatabaseConnection,
	player_bld_key: Path<PlayerBuildingKey>,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	let player_key = player.id;

	building_operations::confirm_upgrade(&mut conn, &player_bld_key)?;
	let res = player_buildings::get_game_building(&mut conn, &player_key, &player_bld_key)
		.map(GameBuilding::from)?;

	Ok(json!(res))
}

/// Returns all building definitions for the player's faction with all levels.
///
/// Includes resources, capacities, upgrade times & requirements, units available,
/// and queue size (training capacity).
#[instrument(skip_all)]
#[debug_handler(state = AppState)]
pub async fn get_all_building_definitions(
	DatabaseConnection(mut conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	debug!(
		"Getting all building definitions for faction: {}",
		&player.faction
	);

	// Query 1+2: Get buildings with levels and resources
	let bld_level_data = buildings::get_faction_building_definitions(&mut conn, &player.faction)?;
	trace!("Fetched {} building-level records", bld_level_data.len());

	// Query 3: Get all requirements and group by building_level_id
	let all_reqs = building_requirements::get_all(&mut conn)?;
	let reqs_by_level: HashMap<BuildingLevelKey, Vec<LevelRequirement>> =
		all_reqs.into_iter().fold(HashMap::new(), |mut map, req| {
			map.entry(req.building_level_id)
				.or_default()
				.push(LevelRequirement {
					required_building_id: req.required_building_id,
					required_building_level: req.required_building_level,
					required_tech_id: req.required_tech_id,
					required_tech_level: req.required_tech_level,
				});
			map
		});

	// Query 4: Get all unit types and group by building_id
	let all_unit_types = building_unit_types::get_all(&mut conn)?;
	let unit_types_by_bld: HashMap<BuildingKey, Vec<UnitType>> =
		all_unit_types
			.into_iter()
			.fold(HashMap::new(), |mut map, but| {
				map.entry(but.building_id).or_default().push(but.unit_type);
				map
			});

	// Assemble into BuildingDefinition structs
	// Group by building_id since the query returns one row per level
	let mut definitions_map: HashMap<BuildingKey, BuildingDefinition> = HashMap::new();

	for (bld, lvl, res) in bld_level_data {
		let level_info = BuildingLevelInfo {
			level: lvl.building_level,
			upgrade_seconds: lvl.upgrade_seconds,
			training_capacity: lvl.training_capacity,
			costs: ResourceCosts {
				food: lvl.req_food.unwrap_or(0),
				wood: lvl.req_wood.unwrap_or(0),
				stone: lvl.req_stone.unwrap_or(0),
				gold: lvl.req_gold.unwrap_or(0),
			},
			production: ResourceProduction {
				population: res.population,
				food: res.food,
				wood: res.wood,
				stone: res.stone,
				gold: res.gold,
			},
			capacity: ResourceCapacity {
				food: res.food_cap,
				wood: res.wood_cap,
				stone: res.stone_cap,
				gold: res.gold_cap,
				food_acc: res.food_acc_cap,
				wood_acc: res.wood_acc_cap,
				stone_acc: res.stone_acc_cap,
				gold_acc: res.gold_acc_cap,
			},
			requirements: reqs_by_level.get(&lvl.id).cloned().unwrap_or_default(),
		};

		definitions_map
			.entry(bld.id)
			.and_modify(|def| def.levels.push(level_info.clone()))
			.or_insert_with(|| BuildingDefinition {
				id: bld.id,
				name: bld.name.clone(),
				max_level: bld.max_level,
				max_count: bld.max_count,
				faction: bld.faction,
				starter: bld.starter,
				unit_types: unit_types_by_bld.get(&bld.id).cloned().unwrap_or_default(),
				levels: vec![level_info],
			});
	}

	// Convert to sorted vector
	let mut definitions: Vec<BuildingDefinition> = definitions_map.into_values().collect();
	definitions.sort_by_key(|d| d.id);

	info!(
		"Retrieved {} building definitions for faction: {}",
		definitions.len(),
		&player.faction
	);

	Ok(json!(definitions))
}
