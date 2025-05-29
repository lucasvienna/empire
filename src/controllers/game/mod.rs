use axum::routing::get;
use axum::Router;

use crate::controllers::game::buildings_controller::buildings_routes;
use crate::controllers::game::index_controller::get_game;
use crate::domain::app_state::AppState;

mod buildings_controller;
mod index_controller;

pub fn game_routes() -> Router<AppState> {
    Router::new().nest(
        "/game",
        Router::new()
            .route("/", get(get_game))
            .merge(buildings_routes()),
    )
}
