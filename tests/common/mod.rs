use axum::Router;
use axum_test::util::new_random_tokio_tcp_listener;
use empire::db::connection::get_test_pool;
use empire::net::router;
use empire::net::server::AppState;
use std::sync::Arc;

/// Creates and initializes the application, returning an Axum `Router`.
///
/// This function performs the following steps:
/// - Creates a test database connection pool and initializes the state.
/// - Runs database migrations to ensure the schema is up-to-date.
/// - Initializes the server and obtains the application router.
/// - Associates the application state with the router.
///
/// # Returns
/// A Result containing the initialized `Router` or an error if something fails.
///
/// # Errors
/// This function will return an error if:
/// - The database connection pool cannot be created.
/// - Migrations fail to execute.
/// - The server initialization fails.
///
/// # Usage
/// Typically used in testing or isolated environments where an
/// independent instance of the app is required.
pub fn get_app() -> anyhow::Result<Router> {
    let state = AppState {
        db_pool: Arc::new(get_test_pool()),
    };

    Ok(router::init().with_state(state))
}

/// Starts the application and returns the server's socket address.
///
/// # Errors
/// Returns an error if the database connection, migrations, or server initialization fails.
pub fn spawn_app() -> String {
    let router = get_app().unwrap();
    let listener = new_random_tokio_tcp_listener().expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        axum::serve(listener, router)
            .await
            .expect("Expect server to start serving");
    });
    format!("http://127.0.0.1:{}", port)
}
