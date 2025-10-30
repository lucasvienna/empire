use axum::Router;
use axum::routing::{get, post};

use crate::controllers::game::buildings::handlers::*;
use crate::domain::app_state::AppState;
pub fn buildings_routes() -> Router<AppState> {
	Router::new().nest(
		"/buildings",
		Router::new()
			.route("/", get(get_buildings))
			.route("/{player_bld_key}", get(get_building))
			.route("/{player_bld_key}/upgrade", post(upgrade_building)),
	)
}
