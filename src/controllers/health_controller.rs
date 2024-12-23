use crate::net::server::AppState;
use axum::http::StatusCode;
use axum::{debug_handler, Json, Router};
use serde::{Deserialize, Serialize};

/// Struct representing the health check response
#[derive(Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
}

#[debug_handler]
/// Health check handler
async fn health_check() -> Result<Json<HealthCheckResponse>, StatusCode> {
    let response = HealthCheckResponse {
        status: "OK".to_string(),
    };

    Ok(Json(response))
}

/// Function to define health check routes
pub fn health_check_routes() -> Router<AppState> {
    Router::new().route("/health", axum::routing::get(health_check))
}
