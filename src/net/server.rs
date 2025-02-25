use std::fmt::Formatter;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

use crate::configuration::Settings;
use crate::db::connection::DbPool;
use crate::net::router;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
    pub settings: Settings,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Retrieve the state from the db_pool
        let db_state = self.db_pool.state();

        f.debug_struct("AppState")
            .field("db_pool", &db_state)
            .finish()
    }
}

pub async fn init(state: AppState) -> Result<(TcpListener, Router)> {
    let settings = &state.settings.server;
    let addr = SocketAddr::from((settings.axum_host, settings.axum_port));
    let listener = TcpListener::bind(addr).await?;
    let router = router::init(state);
    Ok((listener, router))
}
