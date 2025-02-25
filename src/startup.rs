use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::configuration::Settings;
use crate::db::connection::DbPool;
use crate::domain::app_state::AppState;
use crate::game::res_gen_subroutine::init_res_gen;
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
pub async fn launch(config: Settings, pool: DbPool) -> Result<()> {
    let token = CancellationToken::new();
    let subroutines = start_subroutines(pool.clone(), token.clone());

    let (listener, router) = server::init(AppState::new(pool, config)).await?;

    info!("Empire server started!");
    info!("Listening on {}", listener.local_addr()?);

    let server = axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal(token));

    let (srv, _) = tokio::join!(server, subroutines);
    srv.map_err(|err| {
        warn!("Server error while shutting down: {:#?}", err);
        err.into()
    })
}

/// Starts background subroutines required for the Empire server to function.
///
/// The function initializes and runs various subroutines (e.g., resource generation)
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
/// # Behavior
///
/// This function utilizes `tokio::select!` to concurrently monitor the cancellation token and
/// any ongoing subroutine (e.g., `res_gen`). When either the token is cancelled or the
/// subroutine completes, the function exits. Ensure that any subroutines are endless.
///
/// Additional subroutines can be added inside the `tokio::select!` block by adding new arms.
async fn start_subroutines(db_pool: DbPool, token: CancellationToken) {
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

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => token.cancel(),
        _ = terminate => token.cancel(),
    }

    info!("Shutting down...");
}
