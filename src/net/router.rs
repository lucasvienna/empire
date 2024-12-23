use crate::controllers::health_controller::health_check_routes;
use crate::controllers::user_controller::user_routes;
use crate::net::server::AppState;
use axum::Router;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::{
    catch_panic::CatchPanicLayer as TowerCatchPanicLayer, cors::CorsLayer as TowerCorsLayer,
    trace::TraceLayer as TowerTraceLayer,
};

pub fn init() -> Router<AppState> {
    Router::new()
        .merge(health_check_routes())
        .merge(user_routes())
        .layer(TowerTraceLayer::new_for_http())
        .layer(TowerCatchPanicLayer::new())
        .layer(TowerCorsLayer::very_permissive())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
}
