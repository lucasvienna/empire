use crate::controllers::game::index_controller::get_resources_data;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::player::session::PlayerSession;
use crate::game::resources::resource_service::ResourceService;
use crate::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{debug_handler, Extension, Json, Router};
use serde_json::json;
use tracing::{debug, info, instrument, warn};

#[instrument(skip(pool, srv))]
#[debug_handler(state = AppState)]
async fn collect_resources(
    State(pool): State<AppPool>,
    State(srv): State<ResourceService>,
    session: Extension<PlayerSession>,
) -> Result<impl IntoResponse> {
    let player_key = session.player_id;
    debug!("Collecting resources for player: {}", player_key);
    let resources = srv.collect_resources(&player_key);
    match resources {
        Ok(res) => {
            info!("Collected resources: {}", res.id);
            let mut conn = pool.get()?;
            let res_state = get_resources_data(&mut conn, player_key)?;
            let body = json!(res_state);
            Ok((StatusCode::OK, Json(body)))
        }
        Err(err) => {
            warn!("Error collecting resources: {}", err);
            let body = json!({ "status": "fail", "message": err.to_string() });
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(body)))
        }
    }
}

pub fn resource_routes() -> Router<AppState> {
    Router::new().nest(
        "/resources",
        Router::new().route("/collect", post(collect_resources)),
    )
}
