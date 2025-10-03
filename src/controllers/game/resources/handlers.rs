use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension, Json};
use serde_json::json;
use tracing::{debug, info, instrument, warn};

use crate::controllers::game::index_controller::get_resources_data;
use crate::db::extractor::DatabaseConnection;
use crate::domain::app_state::AppState;
use crate::domain::auth::AuthenticatedUser;
use crate::game::resources::resource_service::ResourceService;
use crate::Result;

#[instrument(skip(conn, srv))]
#[debug_handler(state = AppState)]
pub async fn collect_resources(
	DatabaseConnection(mut conn): DatabaseConnection,
	State(srv): State<ResourceService>,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	let player_key = player.id;
	debug!("Collecting resources for player: {}", player_key);
	let resources = srv.collect_resources(&player_key);
	match resources {
		Ok(res) => {
			info!("Collected resources: {}", res.id);
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
