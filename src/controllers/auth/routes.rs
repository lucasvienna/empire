use axum::routing::{get, post};
use axum::Router;

use crate::domain::app_state::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(crate::controllers::auth::handlers::login))
        .route(
            "/register",
            post(crate::controllers::auth::handlers::register),
        )
}

pub fn protected_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/logout", post(crate::controllers::auth::handlers::logout))
        .route("/session", get(crate::controllers::auth::handlers::session))
}
