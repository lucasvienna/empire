use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json, debug_handler};
use serde_json::json;
use tracing::{debug, info, instrument, warn};

use crate::Result;
use crate::controllers::game::index::get_resources_data;
use crate::db::extractor::DatabaseConnection;
use crate::domain::app_state::AppState;
use crate::domain::auth::AuthenticatedUser;
use crate::game::resources::resource_operations;

#[instrument(skip(conn))]
#[debug_handler(state = AppState)]
pub async fn collect_resources(
	DatabaseConnection(mut conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	let player_key = player.id;
	debug!("Collecting resources for player: {}", player_key);

	// Calculate production rates with modifiers (no caching, fresh values)
	let production_rates = resource_operations::calc_prod_rates(&mut conn, &player_key)?;

	// Produce resources up to now and then collect them
	let result = resource_operations::produce_and_collect_resources(
		&mut conn,
		&player_key,
		&production_rates,
	);

	match result {
		Ok((_accumulator, res)) => {
			info!("Produced and collected resources: {}", res.id);
			let res_state = get_resources_data(&mut conn, player_key)?;
			let body = json!(res_state);
			Ok((StatusCode::OK, Json(body)))
		}
		Err(err) => {
			warn!("Error producing/collecting resources: {}", err);
			let body = json!({ "status": "fail", "message": err.to_string() });
			Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(body)))
		}
	}
}
