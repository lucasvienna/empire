use axum::Router;
use axum_test::util::new_random_tokio_tcp_listener;
use diesel::{sql_query, Connection, PgConnection, RunQueryDsl};
use empire::configuration::{get_configuration, DatabaseSettings};
use empire::db::connection::{create_pool_from_settings, DbPool};
use empire::db::migrations::run_migrations;
use empire::net::router;
use empire::net::server::AppState;
use std::sync::Arc;
use uuid::Uuid;

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
        db_pool: Arc::new(initialize_test_pool()),
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

/// Initializes the test database pool.
///
/// This function performs the following steps:
/// - Reads the configuration from the application settings.
/// - Generates a unique database name for the test to avoid conflicts.
/// - Connects to the default `postgres` database and creates a new test database.
/// - Runs the database migrations on the newly created test database to set up the schema.
/// - Creates and returns a connection pool for the test database.
///
/// # Returns
/// A [`DbPool`] connected to the test database.
///
/// # Panics
/// This function will panic if:
/// - The configuration file cannot be read.
/// - The connection to the database cannot be established.
/// - The test database cannot be created.
/// - Migrations fail to execute.
/// - The connection pool cannot be created.
///
/// [`DbPool`]: DbPool
pub fn initialize_test_pool() -> DbPool {
    let mut config = get_configuration()
        .expect("Failed to read configuration.")
        .database;
    config.database_name = Uuid::now_v7().to_string();

    let db_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: "password".to_string(),
        pool_size: Some(1),
        ..config
    };

    let mut conn = PgConnection::establish(&db_settings.connection_string())
        .expect("Failed to connect to database.");

    sql_query(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .execute(&mut conn)
        .expect("Failed to create test schema");

    run_migrations(&mut conn).expect("Failed to run migrations.");
    create_pool_from_settings(db_settings).unwrap()
}
