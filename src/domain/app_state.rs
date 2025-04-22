//! Application state management module.
//!
//! This module provides the core application state management functionality:
//! - Shared database connection pool management
//! - Background job queue coordination
//! - Global application settings access
//! - Thread-safe state sharing across request handlers
//!
//! The module implements the necessary traits for axum's state extraction system
//! and provides convenient constructors for creating and managing application state.
//! All components are wrapped in Arc for thread-safe sharing across the application.

use std::fmt::Formatter;
use std::sync::Arc;

use axum::extract::{FromRef, FromRequestParts, State};
use derive_more::Deref;

use crate::configuration::Settings;
use crate::db::{connection, DbPool};
use crate::game::modifiers::modifier_system::ModifierSystem;
use crate::job_queue::JobQueue;

/// Thread-safe reference-counted wrapper around a database connection pool.
///
/// This type alias combines [`Arc`] (atomic reference counting) with [`DbPool`]
/// to provide thread-safe sharing of database connections across the application.
/// It ensures efficient connection management and safe concurrent access to the database.
///
/// # Notes
/// The pool implements [`FromRef<AppState>`] for ease of access at the controller level.
/// It can be used with `State(pool): State<AppPool>`.
pub type AppPool = Arc<DbPool>;

impl FromRef<AppState> for AppPool {
    fn from_ref(state: &AppState) -> Self {
        state.db_pool.clone()
    }
}

/// Thread-safe reference-counted wrapper around a job queue implementation.
///
/// This type alias combines [`Arc`] (atomic reference counting) with [`JobQueue`]
/// to provide thread-safe sharing of the background job processing system across
/// the application. It ensures safe concurrent access to job scheduling and execution.
///
/// # Notes
/// The queue implements [`FromRef<AppState>`] for ease of access at the controller level.
/// It can be used with `State(queue): State<AppQueue>`.
pub type AppQueue = Arc<JobQueue>;

impl FromRef<AppState> for AppQueue {
    fn from_ref(state: &AppState) -> Self {
        state.job_queue.clone()
    }
}

/// Represents the core application state shared across all requests.
///
/// This struct maintains the shared resources needed throughout the application's lifecycle:
/// - Database connection pool for handling concurrent database operations
/// - Job queue for managing background tasks and asynchronous operations
/// - Application settings for configuration management
#[derive(FromRef, Clone)]
pub struct App {
    /// Database connection pool that manages and reuses database connections
    /// for optimal performance and resource utilisation
    pub db_pool: AppPool,
    /// Centralised job queue system that handles background tasks, scheduled operations,
    /// and asynchronous processing of requests
    pub job_queue: AppQueue,
    /// Centralised modifier sub-system that manages the application's modifier subroutines
    pub modifier_system: ModifierSystem,
    /// Global application configuration containing environment-specific settings
    /// and runtime parameters
    pub settings: Settings,
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Retrieve the state from the db_pool
        let db_state = self.db_pool.state();

        f.debug_struct("App").field("db_pool", &db_state).finish()
    }
}

impl App {
    /// Creates a new [`App`] instance with a fresh database pool.
    ///
    /// # Arguments
    ///
    /// * `settings` - Application [`Settings`] containing database configuration
    ///
    /// # Returns
    ///
    /// A new [`App`] instance with an initialised database pool and job queue
    pub fn new(settings: Settings) -> App {
        let db_pool = Arc::new(connection::initialize_pool(&settings.database));
        let job_queue = Arc::new(JobQueue::new(db_pool.clone()));
        let modifier_system = ModifierSystem::with_job_queue(&settings, &job_queue);

        App {
            db_pool,
            job_queue,
            modifier_system,
            settings,
        }
    }

    /// Creates a new [`App`] instance with an existing database pool.
    ///
    /// # Arguments
    ///
    /// * `db_pool` - Existing [`AppPool`] instance to use
    /// * `settings` - Application [`Settings`] configuration
    ///
    /// # Returns
    ///
    /// A new [`App`] instance with the provided database pool and a new job queue
    pub fn with_pool(db_pool: AppPool, settings: Settings) -> App {
        let job_queue = Arc::new(JobQueue::new(db_pool.clone()));
        let modifier_system = ModifierSystem::with_job_queue(&settings, &job_queue);

        App {
            db_pool,
            job_queue,
            modifier_system,
            settings,
        }
    }
}

/// Represents a wrapper around the application state that implements axum's state extraction.
///
/// This type provides a thread-safe, reference-counted access to the core application state (`App`)
/// and implements the necessary traits for axum to extract it from requests. It wraps the `App`
/// instance in an `Arc` to allow safe sharing across multiple threads and request handlers.
///
/// The `FromRequestParts` derive enables automatic extraction in route handlers,
/// while `Deref` allows transparent access to the underlying `App` methods and fields.
#[derive(Clone, FromRequestParts, Deref)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);
