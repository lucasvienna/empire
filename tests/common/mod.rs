use std::env;
use std::sync::{Arc, LazyLock};

use axum::Router;
use axum_test::util::new_random_tokio_tcp_listener;
use diesel::{sql_query, Connection, PgConnection, RunQueryDsl};
use empire::configuration::{get_settings, DatabaseSettings};
use empire::db::connection::{initialize_pool, DbPool};
use empire::db::migrations::run_pending;
use empire::domain::app_state::AppState;
use empire::net::router;
use empire::Result;
use secrecy::{ExposeSecret, SecretString};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, registry, EnvFilter};
use uuid::Uuid;

pub struct TestServer {
    pub router: Router,
    pub db_pool: DbPool,
}

#[allow(dead_code)]
pub struct TestApp {
    pub address: String,
    pub db_pool: DbPool,
}

/// Creates and initializes the application, returning a `TestServer`.
///
/// This function performs the following steps:
/// - Creates a test database connection pool and initializes the application state.
/// - Runs database migrations to ensure the schema is up-to-date.
/// - Configures the application's router with the initialized state.
///
/// # Returns
/// A [`TestServer`] containing the initialized `Router` and test database pool.
///
/// # Usage
/// This function is typically used in testing environments where an
/// independent instance of the application is required.
///
/// [`TestServer`]: TestServer
pub fn init_server() -> TestServer {
    LazyLock::force(&TRACING);

    let mut config = get_settings().expect("Failed to read configuration.");
    env::set_var("JWT_SECRET", "fake testing secret");

    let (pool, db_settings) = initialize_test_pool(&mut config.database);
    config.database = db_settings.clone();
    let state = AppState {
        db_pool: Arc::new(pool.clone()),
        settings: config,
    };

    TestServer {
        router: router::init(state),
        db_pool: pool,
    }
}

/// Starts the application and returns the test application instance.
///
/// This function performs the following steps:
/// - Initializes the test server, including a test database and app state.
/// - Starts the Axum server on a randomly assigned local port.
///
/// # Returns
/// A [`TestApp`] instance containing the server's address and database pool.
///
/// # Panics
/// This function will panic if:
/// - The test server initialization fails.
/// - The server fails to start serving requests.
///
/// [`TestApp`]: TestApp
pub fn spawn_app() -> TestApp {
    let server = init_server();
    let listener = new_random_tokio_tcp_listener().expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        axum::serve(listener, server.router)
            .await
            .expect("Expect server to start serving");
    });
    TestApp {
        address: format!("http://localhost:{}", port),
        db_pool: server.db_pool,
    }
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
fn initialize_test_pool(config: &mut DatabaseSettings) -> (DbPool, &mut DatabaseSettings) {
    config.database_name = Uuid::new_v4().to_string();

    let mut db_settings = config.clone();
    db_settings.database_name = "postgres".to_string();
    db_settings.username = "postgres".to_string();
    db_settings.password = SecretString::new("password".into());
    db_settings.pool_size = Some(1);

    // === CREATE EPHEMERAL DATABASE ===
    let mut conn = PgConnection::establish(db_settings.connection_string().expose_secret())
        .expect("Failed to connect to database.");
    sql_query(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .execute(&mut conn)
        .expect("Failed to create test schema");
    sql_query(
        format!(
            r#"GRANT ALL ON DATABASE "{}" TO "{}";"#,
            config.database_name, config.username
        )
        .as_str(),
    )
    .execute(&mut conn)
    .expect("Failed to grant database privileges to test user.");

    // === SET EPHEMERAL DATABASE UP ===
    let mut conn = PgConnection::establish(config.connection_string().expose_secret())
        .expect("Failed to connect to database.");
    sql_query(
        format!(
            r#"GRANT ALL ON ALL TABLES IN SCHEMA public TO "{}";"#,
            config.username
        )
        .as_str(),
    )
    .execute(&mut conn)
    .expect("Failed to grant table privileges to test user in public schema");
    run_pending(&mut conn).expect("Failed to run migrations");
    (initialize_pool(config), config)
}

static TRACING: LazyLock<Result<()>> = LazyLock::new(init_test_tracing);

fn init_test_tracing() -> Result<()> {
    if env::var("TEST_LOG").is_ok() {
        let subscriber = registry()
            .with(EnvFilter::from_default_env().add_directive(LevelFilter::TRACE.into()))
            .with(fmt::Layer::new().with_test_writer());
        tracing::subscriber::set_global_default(subscriber).expect("Failed to set global default.");
    } else {
        let subscriber =
            registry().with(EnvFilter::from_default_env().add_directive(LevelFilter::TRACE.into()));
        tracing::subscriber::set_global_default(subscriber).expect("Failed to set global default.");
    }

    Ok(())
}
