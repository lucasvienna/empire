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
use crate::game::modifiers::modifier_processor::ModifierProcessor;
use crate::game::resources::resource_processor::ResourceProcessor;
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
    let app_state = AppState(Arc::new(App::with_pool(pool.clone(), config.clone())));

    let mut subroutines = start_subroutines(&app_state, token.clone())?;
    let monitor = subroutines.monitor();
    info!("Subroutines monitor started!");

    let (listener, router) = server::init(app_state).await?;
    info!("Listening on {}", listener.local_addr()?);

    let server = axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal(token));
    info!("Empire server started!");

    let (srv, _) = tokio::join!(server, monitor);
    srv.map_err(|err| {
        warn!("Server error while shutting down: {:#?}", err);
        err.into()
    })
}

/// Initializes and starts worker pools for background processing.
///
/// This function sets up worker pools for processing game modifiers and resources. It creates
/// a new WorkerPool instance and initializes workers for different processing tasks.
///
/// # Arguments
///
/// * `app_state` - A reference to `AppState` providing access to global state and shared resources
/// * `token` - A `CancellationToken` used to coordinate graceful shutdown of workers
///
/// # Returns
///
/// Returns a `Result<WorkerPool>` containing the initialized worker pool if successful
///
/// # Details
///
/// The function performs the following:
/// - Creates a new WorkerPool with the provided job queue and cancellation token
/// - Calculates the number of workers based on available CPU cores (half of available cores)
/// - Initializes ModifierProcessor workers for handling game modifiers
/// - Initializes ResourceProcessor workers for handling resource calculations
/// - Adds the modifier workers to the pool
///
/// The worker count is automatically adjusted based on the system's available parallelism
/// to ensure optimal resource utilization.
fn start_subroutines(app_state: &AppState, token: CancellationToken) -> Result<WorkerPool> {
    let mut worker_pool = WorkerPool::new(Arc::clone(&app_state.job_queue), token.clone());

    let default_workers = available_parallelism()?.get() / 2;
    let mod_workers = ModifierProcessor::initialise_n(default_workers, app_state);
    let res_workers = ResourceProcessor::initialise_n(default_workers, app_state);
    worker_pool.add_workers(mod_workers);

    Ok(worker_pool)
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
