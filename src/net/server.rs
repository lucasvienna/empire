use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

use crate::domain::app_state::AppState;
use crate::net::router;

pub async fn init(state: AppState) -> Result<(TcpListener, Router)> {
    let settings = &state.settings.server;
    let addr = SocketAddr::from((settings.axum_host, settings.axum_port));
    let listener = TcpListener::bind(addr).await?;
    let router = router::init(state);
    Ok((listener, router))
}
