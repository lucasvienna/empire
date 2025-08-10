use axum::routing::get;
use axum::Router;

use crate::controllers::health::handlers::*;
use crate::domain::app_state::AppState;

/// Function to define health check routes
pub fn health_routes() -> Router<AppState> {
	Router::new().nest(
		"/health",
		Router::new()
			.route("/", get(health_check))
			.route("/ready", get(readiness_check))
			.route("/live", get(liveness_check))
			.route("/service", get(services))
			.route("/metrics", get(metrics)),
	)
}
