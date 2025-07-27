use axum::routing::{get, post};
use axum::Router;

use crate::controllers::player::handlers::*;
use crate::domain::app_state::AppState;

pub fn player_routes() -> Router<AppState> {
    Router::new().nest(
        "/player",
        Router::new()
            .route(
                "/profile",
                get(get_player_profile).put(update_player_profile),
            )
            .route("/faction", post(join_faction)),
    )
}
