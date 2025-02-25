use std::time::Duration;

use axum::body::Body;
use axum::http::{HeaderName, Request};
use axum::{middleware, Router};
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer as TowerCatchPanicLayer;
use tower_http::compression::CompressionLayer as TowerCompressionLayer;
use tower_http::cors::CorsLayer as TowerCorsLayer;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer as TowerTraceLayer;
use tracing::{error, info_span};

use crate::controllers::{auth_routes, health_check_routes, user_routes};
use crate::domain::app_state::AppState;
use crate::net::auth::auth_middleware;
use crate::net::request_id::MakeRequestUlid;

const REQUEST_ID_HEADER: &str = "x-request-id";

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
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(PropagateRequestIdLayer::new(x_request_id));

    Router::new()
        .merge(health_check_routes())
        .merge(auth_routes())
        .merge(user_routes().layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        )))
        .layer(middleware)
        .with_state(state)
}
