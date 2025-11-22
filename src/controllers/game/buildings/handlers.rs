use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json, debug_handler};
use axum_extra::json;
use tracing::{debug, info, instrument, trace};

use crate::Result;
use crate::controllers::game::buildings::models::{ConstructBuildingRequest, GameBuilding};
use crate::db::building_requirements::get_construction_reqs;
use crate::db::extractor::DatabaseConnection;
use crate::db::player_buildings;
use crate::db::player_buildings::get_player_bld_counts_levels;
use crate::domain::app_state::AppState;
use crate::domain::auth::AuthenticatedUser;
use crate::domain::player::buildings::PlayerBuildingKey;
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
	let avail = gen_avail_list(blds, bld_data, reqs);

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
