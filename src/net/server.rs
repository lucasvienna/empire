//! Server initialisation module providing functionality to set up and configure
//! the HTTP server with the appropriate TCP listener and router configuration.

use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

use crate::domain::app_state::AppState;
use crate::net::router;

/// Initialises the server by creating a TCP listener and configuring the router.
///
/// # Arguments
///
/// * `state` - The application state containing server settings and shared resources
///
/// # Returns
///
/// Returns a tuple containing:
/// * A `TcpListener` bound to the configured host and port
/// * A configured `Router` instance with all application routes
///
/// # Errors
///
/// Returns an error if binding to the specified address fails
pub async fn init(state: AppState) -> Result<(TcpListener, Router)> {
	let settings = &state.settings.server;
	let addr = SocketAddr::from((settings.axum_host, settings.axum_port));
	let listener = TcpListener::bind(addr).await?;
	let router = router::init(state);
	Ok((listener, router))
}
