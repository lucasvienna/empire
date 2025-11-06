use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json, debug_handler};
use axum_extra::json;
use tracing::{debug, info, instrument, trace};

use crate::controllers::game::buildings::models::{ConstructBuildingRequest, GameBuilding};
use crate::db::extractor::DatabaseConnection;
use crate::db::player_buildings;
use crate::domain::app_state::AppState;
use crate::domain::auth::AuthenticatedUser;
use crate::domain::player::buildings::PlayerBuildingKey;
use crate::game::building_operations;
use crate::{Result, not_implemented};

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

#[instrument(skip_all)]
#[debug_handler(state = AppState)]
pub async fn get_available_buildings(
	DatabaseConnection(conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
) -> impl IntoResponse {
	// Requirements
	// - Player faction == building faction
	// - Current amount < maximum building count
	// - Current level >= minimum building level <-- do we even have levels? keep level? player level? XP?

	not_implemented!()
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
