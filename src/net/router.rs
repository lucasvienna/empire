use crate::controllers::health_check_routes;
use crate::controllers::user_routes;
use crate::net::server::AppState;
use axum::body::Body;
use axum::http::Request;
use axum::Router;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::{
    catch_panic::CatchPanicLayer as TowerCatchPanicLayer, cors::CorsLayer as TowerCorsLayer,
    trace::TraceLayer as TowerTraceLayer,
};
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::debug_span;

pub fn init() -> Router<AppState> {
    Router::new()
        .merge(health_check_routes())
        .merge(user_routes())
        .layer(
            TowerTraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                debug_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    path = %request.uri().path(),
                )
            }),
        )
        .layer(TowerCatchPanicLayer::new())
        .layer(TowerCorsLayer::very_permissive())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(RequestIdLayer)
}
