use crate::controllers::user_routes;
use crate::controllers::{auth_routes, health_check_routes};
use crate::net::request_id::MakeRequestUlid;
use crate::net::server::AppState;
use axum::body::Body;
use axum::http::{HeaderName, Request};
use axum::Router;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer as TowerCompressionLayer;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::{
    catch_panic::CatchPanicLayer as TowerCatchPanicLayer, cors::CorsLayer as TowerCorsLayer,
    trace::TraceLayer as TowerTraceLayer,
};
use tracing::{error, info_span};

const REQUEST_ID_HEADER: &str = "x-request-id";

pub fn init() -> Router<AppState> {
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
        .merge(user_routes())
        .merge(auth_routes())
        .layer(middleware)
}
