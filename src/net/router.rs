use crate::controllers::user_controller::user_routes;
use crate::net::server::AppState;
use axum::routing::get;
use axum::Router;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::{
    catch_panic::CatchPanicLayer as TowerCatchPanicLayer, cors::CorsLayer as TowerCorsLayer,
    trace::TraceLayer as TowerTraceLayer,
};

pub(in crate::net) fn init() -> Router<AppState> {
    Router::new()
        .merge(user_routes())
        .route("/", get(root_handler))
        .layer(TowerTraceLayer::new_for_http())
        .layer(TowerCatchPanicLayer::new())
        .layer(TowerCorsLayer::very_permissive())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
}

// Define a simple handler function for the root route
async fn root_handler() -> &'static str {
    "Hello, World!"
}
