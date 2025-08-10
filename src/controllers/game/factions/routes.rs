use axum::routing::get;
use axum::Router;

use crate::controllers::game::factions::handlers::*;
use crate::domain::app_state::AppState;

pub fn factions_routes() -> Router<AppState> {
	Router::new().nest(
		"/factions",
		Router::new()
			.route("/", get(get_factions))
			.route("/{faction_id}", get(get_faction)),
	)
}
