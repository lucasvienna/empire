#![allow(dead_code)]

mod helpers;

use std::env;
use std::sync::{Arc, LazyLock};

use axum::Router;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_test::util::new_random_tokio_tcp_listener;
use diesel::{Connection, PgConnection, RunQueryDsl, sql_query};
use empire::Result;
use empire::configuration::{DatabaseSettings, get_settings};
use empire::db::DbConn;
use empire::db::connection::{DbPool, initialize_pool};
use empire::db::migrations::run_pending;
use empire::domain::app_state::{App, AppPool, AppState};
use empire::domain::auth::init_keys;
use empire::domain::factions::FactionCode;
use empire::domain::player::{Player, PlayerKey};
use empire::net::router;
use secrecy::{ExposeSecret, SecretString};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, fmt, registry};
use uuid::Uuid;

use crate::common::helpers::*;

/// Test harness containing the core application components for integration testing.
///
/// This struct provides access to the initialized application state, router, and
/// database pool without actually starting an HTTP server. It's ideal for unit tests
/// that need to test application logic directly without network overhead.
#[allow(dead_code)]
pub struct TestHarness {
	/// The initialized application instance with all dependencies
	pub app: Arc<App>,
	/// The configured Axum router ready for testing
	pub router: Router,
	/// Database connection pool for direct database access in tests
	pub db_pool: DbPool,
	/// Internal shared pointer to the connection pool.
	app_pool: AppPool,
}

/// Running test server instance with network access.
///
/// This struct represents a fully running HTTP server instance bound to a local port.
/// It provides the server's address for making HTTP requests and access to the
/// underlying database pool for test setup/teardown operations.
#[allow(dead_code)]
pub struct TestApp {
	/// The HTTP address where the server is listening (e.g., "http://localhost:8080")
	pub address: String,
	/// Database connection pool for test data management
	pub db_pool: DbPool,
	/// Internal shared pointer to the connection pool.
	app_pool: AppPool,
}

impl TestHarness {
	/// Initializes a test harness with a fresh application instance and isolated database.
	///
	/// This function performs the following setup steps:
	/// 1. Initializes tracing for test logging
	/// 2. Loads application configuration
	/// 3. Initializes JWT authentication keys
	/// 4. Creates an isolated test database with migrations
	/// 5. Sets up the application state with the test database
	/// 6. Configures the router with all routes
	///
	/// # Returns
	/// A [`TestHarness`] containing the initialized application components.
	///
	/// # Usage
	/// Use this function when you need to test application logic without starting
	/// an actual HTTP server. This is more efficient for unit and integration tests
	/// that don't require network communication.
	pub fn new() -> Self {
		// Ensure tracing is initialized for test output
		LazyLock::force(&TRACING);

		let mut settings = get_settings().expect("Failed to read configuration");
		init_keys(&settings.jwt.secret);

		// Create an isolated test database and update settings
		let (db_pool, updated_db_settings) = create_isolated_test_database(&mut settings.database);
		settings.database = updated_db_settings.clone();

		// Initialize application components
		let pool = Arc::new(db_pool.clone());
		let app_pool = Arc::clone(&pool);
		let app = Arc::new(App::with_pool(pool, settings));

		Self {
			app: Arc::clone(&app),
			db_pool,
			app_pool,
			router: router::init(AppState(app)),
		}
	}

	/// Create a player with neutral faction. Uses internal DB functions.
	pub fn create_test_user(&self, faction: Option<FactionCode>) -> Player {
		let mut conn = self.get_conn();
		create_test_user(&mut conn, faction)
	}

	pub fn create_bearer_token(&self, player_key: &PlayerKey) -> Authorization<Bearer> {
		get_bearer(player_key)
	}

	pub fn get_conn(&self) -> DbConn {
		self.db_pool.get().expect("Failed to get connection")
	}
}

impl TestApp {
	/// Spawns a test server and returns a handle to the running instance.
	///
	/// This function performs the following steps:
	/// 1. Creates a test harness with all application components
	/// 2. Binds the server to a random available port
	/// 3. Starts the server in a background task
	/// 4. Returns connection details for making HTTP requests
	///
	/// # Returns
	/// A [`TestApp`] containing the server address and database pool.
	///
	/// # Panics
	/// This function will panic if:
	/// - Test harness initialization fails
	/// - No available ports can be bound
	/// - The server fails to start
	///
	/// # Usage
	/// Use this function when you need to test the full HTTP server functionality,
	/// including middleware, routing, and request/response handling.
	pub fn new() -> Self {
		let harness = TestHarness::new();
		let app_pool = Arc::clone(&harness.app_pool);

		// Bind to a random available port
		let listener = new_random_tokio_tcp_listener().expect("Failed to bind to random port");
		let port = listener
			.local_addr()
			.expect("Failed to get local address")
			.port();

		// Start the server in a background task
		tokio::spawn(async move {
			axum::serve(listener, harness.router)
				.await
				.expect("Server failed to start");
		});

		Self {
			address: format!("http://localhost:{port}"),
			db_pool: harness.db_pool,
			app_pool,
		}
	}

	/// Create a player with neutral faction. Uses internal DB functions.
	pub fn create_test_user(&self, faction: Option<FactionCode>) -> Player {
		let mut conn = self.get_conn();
		create_test_user(&mut conn, faction)
	}

	pub fn create_bearer_token(&self, player_key: &PlayerKey) -> Authorization<Bearer> {
		get_bearer(player_key)
	}

	pub fn get_conn(&self) -> DbConn {
		self.db_pool.get().expect("Failed to get connection")
	}
}

/// Creates an isolated test database with a unique name and runs migrations.
///
/// This function ensures test isolation by:
/// 1. Generating a unique database name using UUID
/// 2. Creating the database with proper permissions
/// 3. Running all pending migrations
/// 4. Returning a connection pool configured for the test database
///
/// # Arguments
/// * `config` - Mutable reference to database settings that will be updated
///
/// # Returns
/// A tuple containing:
/// - [`DbPool`] - Connection pool for the test database
/// - [`DatabaseSettings`] - Updated database configuration
///
/// # Panics
/// This function will panic if:
/// - Database connection cannot be established
/// - Test database creation fails
/// - Permission grants fail
/// - Migrations fail to execute
/// - Connection pool initialization failss.
fn create_isolated_test_database(config: &mut DatabaseSettings) -> (DbPool, &mut DatabaseSettings) {
	// Generate unique database name to avoid conflicts between concurrent tests
	config.database_name = Uuid::new_v4().to_string();

	// Create connection settings for the PostgreSQL system database
	let mut system_db_settings = config.clone();
	system_db_settings.database_name = "postgres".to_string();
	system_db_settings.username = "postgres".to_string();
	system_db_settings.password = SecretString::new("password".into());
	system_db_settings.pool_size = Some(1);

	// Connect to the system database and create the test database
	let mut system_conn =
		PgConnection::establish(system_db_settings.connection_string().expose_secret())
			.expect("Failed to connect to PostgreSQL system database");

	sql_query(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
		.execute(&mut system_conn)
		.expect("Failed to create test database");

	// Switch to the newly created test database for permission setup
	system_db_settings.database_name = config.database_name.clone();
	let mut test_db_conn =
		PgConnection::establish(system_db_settings.connection_string().expose_secret())
			.expect("Failed to connect to test database");

	// Grant comprehensive permissions to the application user
	grant_database_permissions(&mut test_db_conn, &config.database_name, &config.username);

	// Connect with application credentials and run migrations
	let mut app_conn = PgConnection::establish(config.connection_string().expose_secret())
		.expect("Failed to connect to test database with application credentials");

	run_pending(&mut app_conn).expect("Failed to run database migrations");

	// Run database seeds after migrations
	empire::db::seeds::run(&mut app_conn).expect("Failed to run database seeds");

	(initialize_pool(config), config)
}

/// Grants comprehensive database permissions to the specified user.
///
/// This helper function sets up all necessary permissions for the application user
/// to operate on the test database, including:
/// - Database-level permissions
/// - Schema usage and creation rights
/// - Table permissions (current and future)
///
/// # Arguments
/// * `conn` - Database connection with administrative privileges
/// * `database_name` - Name of the database to grant permissions on
/// * `username` - Username to grant permissions to
///
/// # Panics
/// Panics if any of the permission grants fail.
fn grant_database_permissions(conn: &mut PgConnection, database_name: &str, username: &str) {
	// Grant database-level permissions
	sql_query(format!(r#"GRANT ALL ON DATABASE "{database_name}" TO "{username}";"#).as_str())
		.execute(conn)
		.expect("Failed to grant database privileges");

	// Grant schema permissions for current operations
	sql_query(format!(r#"GRANT USAGE, CREATE ON SCHEMA public TO "{username}";"#).as_str())
		.execute(conn)
		.expect("Failed to grant schema privileges");

	// Grant permissions on existing tables
	sql_query(format!(r#"GRANT ALL ON ALL TABLES IN SCHEMA public TO "{username}";"#).as_str())
		.execute(conn)
		.expect("Failed to grant table privileges");

	// Set default permissions for future tables
	sql_query(
		format!(
			r#"ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO "{username}";"#
		)
		.as_str(),
	)
	.execute(conn)
	.expect("Failed to set default table privileges");
}

/// Global tracing initialization for tests, initialized lazily.
///
/// This static ensures tracing is configured only once across all tests in the suite.
/// It respects the `TEST_LOG` environment variable to control log output during testing.
static TRACING: LazyLock<Result<()>> = LazyLock::new(configure_test_tracing);

/// Configures tracing for the test environment.
///
/// The configuration depends on the `TEST_LOG` environment variable:
/// - If `TEST_LOG` is set: Enables test-friendly output with visible logs
/// - If `TEST_LOG` is not set: Uses minimal logging to avoid test output noise
///
/// # Returns
/// `Result<()>` indicating success or failure of tracing initialization.
///
/// # Environment Variables
/// - `TEST_LOG`: When set, enables verbose logging output during tests
fn configure_test_tracing() -> Result<()> {
	let subscriber =
		registry().with(EnvFilter::from_default_env().add_directive(LevelFilter::TRACE.into()));

	if env::var("TEST_LOG").is_ok() {
		// Test mode with visible output
		let subscriber_with_fmt = subscriber.with(fmt::Layer::new().with_test_writer());
		tracing::subscriber::set_global_default(subscriber_with_fmt)
			.expect("Failed to set global tracing subscriber");
	} else {
		// Silent mode for cleaner test output
		tracing::subscriber::set_global_default(subscriber)
			.expect("Failed to set global tracing subscriber");
	}

	Ok(())
}
