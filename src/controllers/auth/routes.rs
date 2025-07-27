use axum::routing::{get, post};
use axum::Router;

use crate::controllers::auth::handlers::*;
use crate::domain::app_state::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

pub fn protected_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/logout", post(logout))
        .route("/session", get(session))
}
