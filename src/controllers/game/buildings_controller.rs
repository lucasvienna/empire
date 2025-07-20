use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{debug_handler, Extension, Router};
use axum_extra::json;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument, trace};

use crate::db::player_buildings::{FullBuilding, PlayerBuildingRepository};
use crate::domain::app_state::AppState;
use crate::domain::auth::AuthenticatedUser;
use crate::domain::player::buildings::PlayerBuildingKey;
use crate::domain::player::PlayerKey;
use crate::game::building_service::BuildingService;
use crate::Result;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct GameBuilding {
    pub id: PlayerBuildingKey,
    pub player_id: PlayerKey,
    pub building_id: i32,
    pub level: i32,
    pub max_level: i32,
    pub max_count: i32,
    pub upgrade_time: String,
    pub req_food: Option<i64>,
    pub req_wood: Option<i64>,
    pub req_stone: Option<i64>,
    pub req_gold: Option<i64>,
}
impl From<FullBuilding> for GameBuilding {
    fn from(value: FullBuilding) -> Self {
        let (pb, bld, bl, br) = value;
        GameBuilding {
            id: pb.id,
            player_id: pb.player_id,
            building_id: bld.id,
            level: pb.level,
            max_level: bld.max_level,
            max_count: bld.max_count,
            upgrade_time: bl.upgrade_time,
            req_food: bl.req_food,
            req_wood: bl.req_wood,
            req_stone: bl.req_stone,
            req_gold: bl.req_gold,
        }
    }
}

#[instrument(skip(repo, player))]
#[debug_handler(state = AppState)]
async fn get_buildings(
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
async fn get_building(
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
async fn upgrade_building(
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

pub fn buildings_routes() -> Router<AppState> {
    Router::new().nest(
        "/buildings",
        Router::new()
            .route("/", get(get_buildings))
            .route("/{player_bld_key}", get(get_building))
            .route("/{player_bld_key}/upgrade", post(upgrade_building)),
    )
}
