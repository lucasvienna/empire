#![allow(dead_code)]

mod helpers;

use std::env;
use std::sync::{Arc, LazyLock};

use axum::Router;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_test::util::new_random_tokio_tcp_listener;
use derive_more::Deref;
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
use tokio::task::AbortHandle;
use tracing::{info, warn};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, fmt, registry};
use uuid::Uuid;

use crate::common::helpers::*;

type ConnString = String;
type DbName = String;

/// RAII teardown guard for a test-owned PostgreSQL database.
///
/// When the last [`Arc`] clone of this guard is dropped, the associated database
/// is forcibly dropped via `DROP DATABASE ... WITH (FORCE)`, connecting through the
/// admin connection string stored in field 0. Field 1 holds the UUID name of the
/// test database to drop.
///
/// The guard is always held behind [`Arc`] and co-owned by every test escape
/// handle ([`TestPool`], [`TestRouter`]), so teardown fires only after the last
/// extracted component goes out of scope. Failures during teardown are logged at
/// `warn` via `tracing` and swallowed — panicking in `Drop` during a failing test
/// would abort the whole test binary.
// AIDEV-NOTE: Teardown correctness depends on (a) the admin conn string pointing
// at the `postgres` DB (not the test DB — a session cannot drop its own database),
// and (b) every test-facing handle co-owning an Arc clone so drop order waits for
// the last user.
pub struct DbGuard(ConnString, DbName);

/// Drop guard that aborts its wrapped [`AbortHandle`] when the guard is dropped.
///
/// Used by [`TestApp`] to cancel the spawned axum server task on scope exit instead
/// of leaving it dangling in the runtime.
struct AbortOnDrop(AbortHandle);

impl Drop for DbGuard {
	fn drop(&mut self) {
		if let Ok(mut conn) = PgConnection::establish(&self.0) {
			let result = sql_query(format!(r#"DROP DATABASE "{}" WITH (FORCE);"#, self.1).as_str())
				.execute(&mut conn);
			match result {
				Ok(_) => info!("Dropped test database {}", self.1),
				Err(err) => warn!("Failed to drop test database {}: {:?}", self.1, err),
			}
		} else {
			warn!("Failed to connect to test database {}", self.1);
		}
	}
}

/// Diesel connection pool bundled with a co-owned [`DbGuard`] clone.
///
/// Derefs to the inner [`DbPool`], so any `&DbPool` method — `.get()`, pool-state
/// inspection, etc. — is available through `&TestPool` transparently. The guard
/// clone at field 1 is what keeps the test database alive for the lifetime of
/// this value.
///
/// Extract the inner pool via `.0` only when handing off to production code that
/// requires an owned `DbPool`, and make sure something else (the [`TestHarness`],
/// another handle) still holds the guard for the duration of that use.
#[derive(Deref)]
pub struct TestPool(#[deref] pub DbPool, Arc<DbGuard>);

/// Axum [`Router`] bundled with a co-owned [`DbGuard`] clone.
///
/// Derefs to [`Router`] for `&Router` method access. Several common methods on
/// `Router` consume `self` by value (e.g. `tower::ServiceExt::oneshot`,
/// `axum::serve`) and cannot be reached through `Deref`; use [`TestRouter::split`]
/// or [`TestRouter::owned`] to extract the inner router for those call sites.
#[derive(Deref)]
pub struct TestRouter(#[deref] pub Router, Arc<DbGuard>);

impl TestRouter {
	/// Consume `self` and return the inner [`Router`], dropping this handle's
	/// [`Arc<DbGuard>`] clone.
	///
	/// Use this **only** when the test provably does not touch the database, or
	/// when another named binding (a [`TestHarness`], a [`TestPool`]) still holds
	/// a guard clone for the rest of the test. Calling this on a temporary — e.g.
	/// `TestHarness::new().router.owned()` — drops the last guard clone at the end
	/// of the statement, tearing the database down before the router is used.
	///
	/// For tests that need both the router (by value) *and* the database, prefer
	/// [`TestRouter::split`].
	pub fn owned(self) -> Router {
		self.0
	}

	/// Split `self` into the inner [`Router`] and its [`Arc<DbGuard>`] clone.
	///
	/// The idiomatic call site binds the guard to a named local so it lives until
	/// end of scope, then consumes the router:
	///
	/// ```ignore
	/// let (router, _guard) = harness.router.split();
	/// let response = router.oneshot(request).await.unwrap();
	/// ```
	///
	/// This keeps the database alive for the rest of the test regardless of what
	/// the consuming router method does with its argument.
	pub fn split(self) -> (Router, Arc<DbGuard>) {
		(self.0, self.1)
	}
}

/// Test harness containing the core application components for integration testing.
///
/// This struct provides access to the initialized application state, router, and
/// database pool without actually starting an HTTP server. It's ideal for unit tests
/// that need to test application logic directly without network overhead.
pub struct TestHarness {
	/// Internal shared pointer to the connection pool.
	app_pool: AppPool,
	/// The initialized application instance with all dependencies
	pub app: Arc<App>,
	/// The configured Axum router ready for testing
	pub router: TestRouter,
	/// Database connection pool for direct database access in tests
	pub db_pool: TestPool,
}

/// Running test server instance with network access.
///
/// This struct represents a fully running HTTP server instance bound to a local port.
/// It provides the server's address for making HTTP requests and access to the
/// underlying database pool for test setup/teardown operations.
pub struct TestApp {
	/// Internal shared pointer to the connection pool.
	app_pool: AppPool,
	/// The HTTP address where the server is listening (e.g., "http://localhost:8080")
	pub address: String,
	/// Database connection pool for test data management
	pub db_pool: TestPool,
	/// Server join handle drop guard.
	_handle: AbortOnDrop,
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
		let (db_pool, sys_con_str) = create_isolated_test_database(&mut settings.database);
		let test_db_name = settings.database.database_name.clone();

		// Initialize application components
		let pool = Arc::new(db_pool.clone());
		let app_pool = Arc::clone(&pool);
		let app = Arc::new(App::with_pool(pool, settings.clone()));
		let guard = Arc::new(DbGuard(sys_con_str, test_db_name));

		Self {
			app: Arc::clone(&app),
			db_pool: TestPool(db_pool, guard.clone()),
			app_pool,
			router: TestRouter(router::init(AppState(app)), guard),
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

	pub fn app_pool(&self) -> AppPool {
		self.app_pool.clone()
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
		let handle = tokio::spawn(async move {
			axum::serve(listener, harness.router.0)
				.await
				.expect("Server failed to start");
		});

		Self {
			address: format!("http://localhost:{port}"),
			db_pool: harness.db_pool,
			app_pool,
			_handle: AbortOnDrop(handle.abort_handle()),
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
fn create_isolated_test_database(config: &mut DatabaseSettings) -> (DbPool, String) {
	// Generate unique database name to avoid conflicts between concurrent tests
	config.database_name = Uuid::new_v4().to_string();

	// Create connection settings for the PostgreSQL system database
	let mut db_config = config.clone();
	db_config.database_name = "postgres".to_string();
	db_config.username = "postgres".to_string();
	db_config.password = SecretString::new("password".into());
	db_config.pool_size = Some(1);
	let sys_con_str = db_config.connection_string().expose_secret().to_owned();

	// Connect to the system database and create the test database
	let mut system_conn = PgConnection::establish(&sys_con_str)
		.expect("Failed to connect to PostgreSQL system database");

	sql_query(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
		.execute(&mut system_conn)
		.expect("Failed to create test database");

	// Switch to the newly created test database for permission setup
	db_config.database_name = config.database_name.clone();
	let mut test_db_conn = PgConnection::establish(db_config.connection_string().expose_secret())
		.expect("Failed to connect to test database");

	// Grant comprehensive permissions to the application user
	grant_database_permissions(&mut test_db_conn, &config.database_name, &config.username);

	// Connect with application credentials and run migrations
	let mut app_conn = PgConnection::establish(config.connection_string().expose_secret())
		.expect("Failed to connect to test database with application credentials");

	run_pending(&mut app_conn).expect("Failed to run database migrations");

	// Run database seeds after migrations
	empire::db::seeds::run(&mut app_conn).expect("Failed to run database seeds");

	(initialize_pool(config), sys_con_str)
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
