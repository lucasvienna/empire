use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{debug_handler, Extension, Router};
use axum_extra::json;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};

use crate::db::player_buildings::{FullBuilding, PlayerBuildingRepository};
use crate::domain::app_state::AppState;
use crate::domain::player::buildings::PlayerBuildingKey;
use crate::domain::player::session::PlayerSession;
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

#[instrument(skip(repo))]
#[debug_handler(state = AppState)]
async fn get_buildings(
    State(repo): State<PlayerBuildingRepository>,
    session: Extension<PlayerSession>,
) -> impl IntoResponse {
    let player_key = session.player_id;
    let buildings = repo.get_game_buildings(&player_key).unwrap_or_default();
    let body: Vec<GameBuilding> = buildings.into_iter().map(|v| v.into()).collect();
    json!(body)
}

#[instrument(skip(repo))]
#[debug_handler(state = AppState)]
async fn get_building(
    State(repo): State<PlayerBuildingRepository>,
    player_building_key: Path<PlayerBuildingKey>,
    session: Extension<PlayerSession>,
) -> Result<impl IntoResponse, StatusCode> {
    let player_key = session.player_id;
    let game_bld = repo
        .get_game_building(&player_key, &player_building_key)
        .map(GameBuilding::from)
        .map(|b| json!(b))
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(game_bld)
}

#[instrument(skip(srv, repo, session))]
#[debug_handler(state = AppState)]
async fn upgrade_building(
    State(srv): State<BuildingService>,
    State(repo): State<PlayerBuildingRepository>,
    player_bld_key: Path<PlayerBuildingKey>,
    session: Extension<PlayerSession>,
) -> Result<impl IntoResponse> {
    let player_key = session.player_id;
    debug!(
        "Upgrading building {:?} for user {}",
        player_bld_key, player_key
    );
    let bld = srv.upgrade_building(&player_bld_key)?;
    info!("Building upgraded: {:?}", bld);
    debug!("Upgrade ready in {}", bld.upgrade_time.unwrap_or_default());
    let res = repo
        .get_game_building(&player_key, &bld.id)
        .map(GameBuilding::from)?;
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
