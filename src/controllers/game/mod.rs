use axum::routing::{get, post};
use axum::Router;

use crate::controllers::game::buildings::buildings_routes;
use crate::controllers::game::index_controller::{get_game, join_faction};
use crate::controllers::game::resources::resource_routes;
use crate::domain::app_state::AppState;

mod buildings;
mod index_controller;
mod resources;

pub fn game_routes() -> Router<AppState> {
    Router::new().nest(
        "/game",
        Router::new()
            .route("/", get(get_game))
            .route("/join_faction", post(join_faction))
            .merge(buildings_routes())
            .merge(resource_routes()),
    )
}
