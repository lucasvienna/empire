//! Application state management module.
//!
//! Provides core shared state functionality for the application:
//! - Shared database connection pool for efficient database access
//! - Background job queue for asynchronous task processing
//! - Centralized modifier system integration
//! - Immutable application settings
//!
//! All components are wrapped in [`Arc`] to enable safe concurrency and sharing across threads.
//! Implements traits required by axum for convenient state extraction in request handlers.

use std::fmt::{self, Formatter};
use std::sync::Arc;

use axum::extract::{FromRef, FromRequestParts, State};
use derive_more::Deref;

use crate::configuration::Settings;
use crate::db::{DbPool, connection};
use crate::game::modifiers::modifier_system::ModifierSystem;
use crate::job_queue::JobQueue;

/// Thread-safe shared handle to a database connection pool.
///
/// Combines `Arc` with `DbPool` to allow multiple parts of the application
/// to access the database concurrently and efficiently.
/// Implements `FromRef<App>` to enable extraction from app state in handlers.
pub type AppPool = Arc<DbPool>;

impl FromRef<AppState> for AppPool {
	fn from_ref(state: &AppState) -> Self {
		Arc::clone(&state.db_pool)
	}
}

/// Thread-safe shared handle to the job queue system.
///
/// Wraps `JobQueue` in an `Arc` for thread-safe sharing.
/// Implements `FromRef<App>` to allow extraction in request handlers.
pub type AppQueue = Arc<JobQueue>;

impl FromRef<AppState> for AppQueue {
	fn from_ref(state: &AppState) -> Self {
		Arc::clone(&state.job_queue)
	}
}

/// Core application state shared across all request handlers.
///
/// This struct holds primary shared resources:
/// - Database pool for handling DB queries
/// - Job queue for async/background tasks
/// - Modifier system for game-related logic
/// - Application settings loaded at startup
#[derive(Clone, FromRef)]
pub struct App {
	/// Shared database connection pool
	pub db_pool: AppPool,
	/// Shared job queue for background processing
	pub job_queue: AppQueue,
	/// Modifier system instance
	pub modifier_system: ModifierSystem,
	/// Global application settings
	pub settings: Settings,
}

impl fmt::Debug for App {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		// For brevity and relevance, only output db pool state summary
		let db_state = self.db_pool.state();
		f.debug_struct("App").field("db_pool", &db_state).finish()
	}
}

impl App {
	/// Constructs a new `App` with a fresh database pool initialized from settings.
	///
	/// # Arguments
	///
	/// * `settings` - Application configuration including DB parameters
	pub fn new(settings: Settings) -> Self {
		// Initialize DB pool from settings
		let db_pool = Arc::new(connection::initialize_pool(&settings.database));
		// Create job queue linked to DB pool for persisting jobs
		let job_queue = Arc::new(JobQueue::new(Arc::clone(&db_pool)));
		// Setup modifier system with access to job queue and settings
		let modifier_system = ModifierSystem::with_job_queue(&settings, &job_queue);

		Self {
			db_pool,
			job_queue,
			modifier_system,
			settings,
		}
	}

	/// Constructs a new `App` using an existing database pool.
	///
	/// Useful for testing or if app already manages a pool externally.
	///
	/// # Arguments
	///
	/// * `db_pool` - Pre-existing shared database pool
	/// * `settings` - Application configuration
	pub fn with_pool(db_pool: AppPool, settings: Settings) -> Self {
		let job_queue = Arc::new(JobQueue::new(Arc::clone(&db_pool)));
		let modifier_system = ModifierSystem::with_job_queue(&settings, &job_queue);

		Self {
			db_pool,
			job_queue,
			modifier_system,
			settings,
		}
	}
}

/// Thread-safe wrapper around the application state for axum integration.
///
/// Implements axum's `FromRequestParts` to enable extraction of shared app state in routes,
/// wrapped inside an `Arc` for safe concurrent access. Also, derefs transparently to `App`.
#[derive(Clone, FromRequestParts, Deref)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);
