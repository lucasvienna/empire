use axum::Router;
use axum::routing::get;

use super::handlers::get_game;
use crate::domain::app_state::AppState;

pub fn index_routes() -> Router<AppState> {
	Router::new().route("/", get(get_game))
}
