use crate::configuration::{ServerSettings, Settings};
use crate::db::connection::DbPool;
use crate::net::router;
use anyhow::Result;
use axum::Router;
use std::fmt::{Debug, Formatter};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
    pub settings: Settings,
}

impl Debug for AppState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Retrieve the state from the db_pool
        let db_state = self.db_pool.state();

        f.debug_struct("AppState")
            .field("db_pool", &db_state)
            .finish()
    }
}

pub async fn init(settings: &ServerSettings) -> Result<(TcpListener, Router<AppState>)> {
    let addr = SocketAddr::from((settings.axum_host, settings.axum_port));
    let listener = TcpListener::bind(addr).await?;
    let router = router::init();
    Ok((listener, router))
}
