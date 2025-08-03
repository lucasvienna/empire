use axum::routing::get;
use axum::Router;

use crate::controllers::game::buildings::buildings_routes;
use crate::controllers::game::factions::factions_routes;
use crate::controllers::game::index_controller::get_game;
use crate::controllers::game::resources::resource_routes;
use crate::domain::app_state::AppState;

mod buildings;
mod factions;
mod index_controller;
mod resources;

pub fn game_routes() -> Router<AppState> {
    Router::new().nest(
        "/game",
        Router::new()
            .route("/", get(get_game))
            .merge(buildings_routes())
            .merge(resource_routes())
            .merge(factions_routes()),
    )
}
