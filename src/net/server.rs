use crate::configuration::ServerSettings;
use crate::db::connection::DbPool;
use crate::net::router;
use anyhow::Result;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
}

pub async fn init(settings: ServerSettings) -> Result<(TcpListener, Router<AppState>)> {
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.rest_port));
    let listener = TcpListener::bind(addr).await?;
    let router = router::init();
    Ok((listener, router))
}
