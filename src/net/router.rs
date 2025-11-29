//! Router configuration module that sets up the application's HTTP routing
//! and middleware stack.
//!
//! This module is responsible for:
//! - Configuring middleware layers for request processing
//! - Setting up request ID generation and propagation
//! - Establishing request tracing and logging
//! - Implementing security features like CORS and authentication
//! - Defining the application's route structure
//! - Managing timeouts and error handling

use std::time::Duration;

use axum::body::Body;
use axum::http::{HeaderName, Request, StatusCode};
use axum::{Router, middleware};
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer as TowerCatchPanicLayer;
use tower_http::compression::CompressionLayer as TowerCompressionLayer;
use tower_http::cors::CorsLayer as TowerCorsLayer;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer as TowerTraceLayer;
use tracing::{error, info_span};

use crate::controllers::routes::{
	auth_routes, game_routes, health_routes, player_routes, protected_auth_routes, user_routes,
};
use crate::domain::app_state::AppState;
use crate::net::auth::auth_middleware;
use crate::net::request_id::MakeRequestUlid;

/// HTTP header name used for request ID tracking across the application.
/// This header is set and propagated through middleware layers to enable
/// request tracing and correlation.
const REQUEST_ID_HEADER: &str = "x-request-id";

/// Initialises and configures the application router with all necessary middleware and routes.
///
/// # Arguments
///
/// * `state` - Application state containing shared resources like database connections and configuration
///
/// # Returns
///
/// Returns a configured Router instance with the following features:
/// - Request ID generation and propagation
/// - Request tracing and logging
/// - Panic recovery
/// - CORS support
/// - Response compression
/// - Request timeout
/// - Authentication middleware for protected routes
pub fn init(state: AppState) -> Router {
	let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);

	let middleware = ServiceBuilder::new()
		.layer(SetRequestIdLayer::new(
			x_request_id.clone(),
			MakeRequestUlid,
		))
		.layer(
			TowerTraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
				// Log the request id as generated.
				let request_id = request.headers().get(REQUEST_ID_HEADER);

				match request_id {
					Some(request_id) => info_span!(
						"http_request",
						request_id = ?request_id,
						method = %request.method(),
						path = %request.uri().path(),
					),
					None => {
						error!("could not extract request_id");
						info_span!(
							"http_request",
							method = %request.method(),
							path = %request.uri().path(),
						)
					}
				}
			}),
		)
		.layer(TowerCatchPanicLayer::new())
		.layer(TowerCorsLayer::permissive())
		.layer(TowerCompressionLayer::new())
		.layer(TimeoutLayer::with_status_code(
			StatusCode::REQUEST_TIMEOUT,
			Duration::from_secs(10),
		))
		.layer(PropagateRequestIdLayer::new(x_request_id));

	let protected_routes = Router::new()
		.merge(protected_auth_routes())
		.merge(player_routes())
		.merge(user_routes())
		.merge(game_routes())
		.layer(middleware::from_fn_with_state(
			state.clone(),
			auth_middleware,
		));

	Router::new()
		.merge(health_routes())
		.merge(auth_routes())
		.merge(protected_routes)
		.layer(middleware)
		.with_state(state)
}
