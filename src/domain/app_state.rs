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
use crate::job_queue::JobQueue;

/// Shared database pool
pub type AppPool = Arc<DbPool>;

/// Shared job queue
pub type AppQueue = Arc<JobQueue>;

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
        App {
            db_pool,
            job_queue,
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
        App {
            db_pool,
            job_queue,
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
