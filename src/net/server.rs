use crate::db::conn::DbPool;
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

pub async fn init() -> Result<(TcpListener, Router<AppState>)> {
    // Define the address and port to bind the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000)); // localhost:3000
    let listener = TcpListener::bind(addr).await?;
    let router = router::init();
    Ok((listener, router))
}
