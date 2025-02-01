use std::sync::Arc;
use tokio::signal;
use tracing::info;

use crate::configuration::Settings;
use crate::db::connection::DbPool;
use crate::net::server;
use crate::net::server::AppState;
use crate::Result;

/// Launches the Empire server with the specified configuration and database connection pool.
///
/// This function performs the following actions:
/// - Initializes the server listener and router from the provided server configuration.
/// - Sets the application state, including the database connection pool.
/// - Logs the server's listening address.
/// - Starts serving requests with Axum, ensuring graceful shutdown on receiving termination signals.
///
/// # Arguments
///
/// * `config` - The application settings, which include server and database configurations.
/// * `pool` - A `DbPool` for managing database connections.
///
/// # Errors
///
/// Returns an error if any of the following occur:
/// - Initialization of the server's listener or router fails.
/// - Retrieving the server's local address fails.
/// - Starting the Axum server or handling graceful shutdown encounters an issue.
pub async fn launch(config: Settings, pool: DbPool) -> Result<()> {
    let (listener, router) = server::init(config.server).await?;
    let router = router.with_state(AppState {
        db_pool: Arc::new(pool),
    });

    info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Waits for a shutdown signal in the application.
///
/// This function listens for two types of signals:
/// - `Ctrl+C` signal on all platforms.
/// - `SIGTERM` signal on Unix-based systems.
///
/// When any of these signals is received, the function returns, allowing the application
/// to proceed with a graceful shutdown. On non-Unix platforms (e.g., Windows), only `Ctrl+C`
/// is handled.
///
/// # Panics
///
/// - If the `Ctrl+C` signal handler fails to install.
/// - On Unix-based systems, if the `SIGTERM` signal handler fails to install.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
