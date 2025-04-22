//! Service layer module that provides common functionality for API services.
//!
//! This module defines core service traits and implementations that are used
//! to abstract database operations and business logic from the HTTP layer.

use axum::extract::FromRef;

use crate::domain::app_state::{AppPool, AppState};

/// Generic trait for implementing API services with shared state access.
///
/// This trait provides a common interface for services that need access to
/// the application's shared state and database connection pool. It requires
/// implementors to be extractable from the shared state type `S` via `FromRef`.
///   
/// # Type Parameters
///
/// * `S` - The shared state type that must implement `Send` + `Sync` for thread safety
pub trait ApiService<S = AppState>: FromRef<S>
where
    S: Send + Sync,
{
    /// Creates a new instance of the service with the provided database pool.
    ///
    /// # Arguments
    /// * `pool` - Reference to the application's database connection pool
    fn new(pool: &AppPool) -> Self;
}
