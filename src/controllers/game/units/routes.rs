//! Route definitions for the units API endpoints.

use axum::Router;
use axum::routing::{delete, get, post};

use crate::controllers::game::units::handlers::*;
use crate::domain::app_state::AppState;

/// Returns a router with all unit training routes.
///
/// Routes:
/// - `GET /units/available?building_id={uuid}` - Get trainable units for a building
/// - `POST /units/train` - Start training units
/// - `GET /units/queue` - Get player's training queue
/// - `DELETE /units/queue/{training_id}` - Cancel training
/// - `GET /units/inventory` - Get player's unit counts
pub fn units_routes() -> Router<AppState> {
	Router::new().nest(
		"/units",
		Router::new()
			.route("/available", get(get_available_units))
			.route("/train", post(train_units))
			.route("/queue", get(get_training_queue))
			.route("/queue/{training_id}", delete(cancel_training))
			.route("/inventory", get(get_player_inventory)),
	)
}
