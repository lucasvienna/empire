use axum::routing::post;
use axum::Router;

use crate::domain::app_state::AppState;

pub fn resource_routes() -> Router<AppState> {
    Router::new().nest(
        "/resources",
        Router::new().route(
            "/collect",
            post(crate::controllers::game::resources::handlers::collect_resources),
        ),
    )
}
