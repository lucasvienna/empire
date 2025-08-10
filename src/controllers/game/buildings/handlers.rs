use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension};
use axum_extra::json;
use tracing::{debug, info, instrument, trace};

use crate::controllers::game::buildings::models::GameBuilding;
use crate::db::player_buildings::PlayerBuildingRepository;
use crate::domain::app_state::AppState;
use crate::domain::auth::AuthenticatedUser;
use crate::domain::player::buildings::PlayerBuildingKey;
use crate::game::building_service::BuildingService;
use crate::Result;

#[instrument(skip(repo, player))]
#[debug_handler(state = AppState)]
pub async fn get_buildings(
	State(repo): State<PlayerBuildingRepository>,
	player: Extension<AuthenticatedUser>,
) -> impl IntoResponse {
	let player_key = player.id;
	debug!("Getting buildings for player: {}", player_key);
	let buildings = repo.get_game_buildings(&player_key).unwrap_or_default();
	trace!("Found {} buildings for player", buildings.len());
	let body: Vec<GameBuilding> = buildings.into_iter().map(|v| v.into()).collect();
	info!(
		"Retrieved {} buildings for player: {}",
		body.len(),
		player_key
	);
	json!(body)
}

#[instrument(skip(repo, player))]
#[debug_handler(state = AppState)]
pub async fn get_building(
	State(repo): State<PlayerBuildingRepository>,
	player_building_key: Path<PlayerBuildingKey>,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse, StatusCode> {
	let player_key = player.id;
	let building_key = player_building_key.0;
	debug!(
		"Getting building {:?} for player: {}",
		building_key, player_key
	);

	let result = repo.get_game_building(&player_key, &building_key);
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

#[instrument(skip(srv, repo, player))]
#[debug_handler(state = AppState)]
pub async fn upgrade_building(
	State(srv): State<BuildingService>,
	State(repo): State<PlayerBuildingRepository>,
	player_bld_key: Path<PlayerBuildingKey>,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	let player_key = player.id;
	let building_key = player_bld_key.0;
	debug!(
		"Starting upgrade building {:?} for player {}",
		building_key, player_key
	);

	let bld = srv.upgrade_building(&building_key)?;
	trace!("Building upgrade details: {:?}", bld);

	let upgrade_time = bld.upgrade_time.unwrap_or_default();
	debug!("Building upgrade will be ready in {}", upgrade_time);

	let res = repo
		.get_game_building(&player_key, &bld.id)
		.map(GameBuilding::from)?;

	info!(
		"Successfully upgraded building {:?} for player {}",
		building_key, player_key
	);

	Ok(json!(res))
}
