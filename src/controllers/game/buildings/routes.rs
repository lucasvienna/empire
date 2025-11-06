use axum::Router;
use axum::routing::{get, post};

use crate::controllers::game::buildings::handlers::*;
use crate::domain::app_state::AppState;

pub fn buildings_routes() -> Router<AppState> {
	Router::new().nest(
		"/buildings",
		Router::new()
			.route("/", get(get_player_buildings))
			.route("/available", get(get_available_buildings))
			.route("/construct", post(construct_player_building))
			.nest(
				"/{player_bld_key}",
				Router::new()
					.route("/", get(get_player_building))
					.route("/upgrade", post(upgrade_building))
					.route("/upgrade/confirm", post(confirm_upgrade)),
			),
	)
}
