//! Server startup and lifecycle management module.
//!
//! This module is responsible for:
//! - Initializing and launching the HTTP server
//! - Managing server lifecycle and graceful shutdown
//! - Starting background services and subroutines
//! - Handling system signals for graceful termination
//! - Coordinating worker pools and job queues
//!
//! The module provides the main entry point for starting the Empire server
//! and ensures proper initialisation of all required components including
//! database connections, background tasks, and HTTP services.

use std::sync::Arc;
use std::thread::available_parallelism;

use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::configuration::Settings;
use crate::domain::app_state::{App, AppPool, AppState};
use crate::game::res_gen_subroutine::init_res_gen;
use crate::job_queue::worker_pool::WorkerPool;
use crate::net::server;
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
pub async fn launch(config: Settings, pool: AppPool) -> Result<()> {
    let token = CancellationToken::new();
    let app = Arc::new(App::with_pool(pool.clone(), config.clone()));
    let subroutines = start_subroutines(&pool, token.clone());

    let default_workers = available_parallelism()?.get();
    let mut worker_pool = WorkerPool::new(Arc::clone(&app.job_queue), token.clone());
    worker_pool
        .initialise_workers(config.server.workers.unwrap_or(default_workers))
        .await?;
    let worker_monitor = worker_pool.run();

    let app_state = AppState(app);
    let (listener, router) = server::init(app_state).await?;

    info!("Empire server started!");
    info!("Listening on {}", listener.local_addr()?);

    let server = axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal(token));

    let (srv, _, _) = tokio::join!(server, subroutines, worker_monitor);
    srv.map_err(|err| {
        warn!("Server error while shutting down: {:#?}", err);
        err.into()
    })
}

/// Starts background subroutines required for the Empire server to function.
///
/// The function initialises and runs various subroutines (e.g. resource generation)
/// using the provided database connection and listens for shutdown signals
/// through the `CancellationToken`.
///
/// When a cancellation signal is received through the `token`,
/// all running subroutines will terminate.
///
/// # Arguments
///
/// * `connection` - A `DbConn` providing access to the database for the subroutines.
/// * `token` - A `CancellationToken` used to detect shutdown signals and terminate subroutines.
///
/// # Behaviour
///
/// This function uses `tokio::select!` to concurrently monitor the cancellation token and
/// any ongoing subroutine (e.g. `res_gen`). When either the token is cancelled or the
/// subroutine completes, the function exits. Ensure that any subroutines are endless.
///
/// Additional subroutines can be added inside the `tokio::select!` block by adding new arms.
async fn start_subroutines(db_pool: &AppPool, token: CancellationToken) {
    let conn = db_pool.get().expect("Failed to get database connection.");
    let res_gen = init_res_gen(conn);

    tokio::select! {
        _ = token.cancelled() => {},
        _ = res_gen => {},
        // we can add more subroutines here as desired :)
        // _ = time::sleep(Duration::from_secs(10)) => {},
    }
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
async fn shutdown_signal(token: CancellationToken) {
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

    #[cfg(unix)]
    let interrupt = async {
        signal::unix::signal(signal::unix::SignalKind::interrupt())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    #[cfg(not(unix))]
    let interrupt = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => token.cancel(),
        _ = terminate => token.cancel(),
        _ = interrupt => token.cancel(),
    }

    info!("Shutting down...");
}
