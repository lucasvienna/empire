use std::sync::Arc;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use derive_more::Deref;
use tracing::{error, trace};

use crate::db::DbConn;
use crate::domain::app_state::{AppPool, AppState};

/// An extractor that acquires a database connection from a configured connection pool.
///
/// # Overview
///
/// This type implements [`FromRequestParts`] to fetch a [`DbConn`] from a provided
/// pool. The primary use case is to easily obtain an active database connection
/// in an [axum] handler without manually managing the pool.
///
/// # Error Handling
/// If obtaining a connection from the pool fails, this type returns a tuple
/// containing [`StatusCode::INTERNAL_SERVER_ERROR`] as well as the error message.
///
/// # Notes
/// 1. The reliability of the database operations depends on proper configuration
///    of the underlying pool/dependencies.
/// 2. This extractor uses the [`FromRef`] trait to retrieve the appropriate
///    connection pool reference from your state. Ensure your application state
///    type provides the needed reference to the pool.
#[derive(Deref)]
pub struct DatabaseConnection(pub DbConn);

impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    DatabasePool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = DatabasePool::from_ref(state);
        let conn = pool.get().map_err(|err| {
            error!("Failed to get a database connection: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?;
        trace!("Acquired a database connection.");

        Ok(Self(conn))
    }
}

/// An extractor that provides direct access to the database connection pool.
///
/// # Overview
///
/// This type implements [`FromRequestParts`] to fetch a reference to the [`DbPool`]
/// directly. This is useful when you need access to the pool itself rather than
/// a single connection.
///
/// # Error Handling
///
/// While this extractor typically won't fail (as it only clones a reference),
/// it still returns a [`StatusCode::INTERNAL_SERVER_ERROR`] with an error message
/// if the pool reference cannot be obtained.
///
/// # Notes
///
/// 1. This extractor is less commonly needed than [`DatabaseConnection`], as most
///    handlers only need a single connection.
/// 2. Like [`DatabaseConnection`], this type relies on [`FromRef`] to access the
///    pool from your application state.
#[derive(Deref)]
pub struct DatabasePool(pub AppPool);

impl<S> FromRequestParts<S> for DatabasePool
where
    S: Send + Sync,
    DatabasePool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = DatabasePool::from_ref(state);
        trace!("Acquired database pool reference.");
        Ok(pool)
    }
}

impl FromRef<AppState> for DatabasePool {
    fn from_ref(AppState(app): &AppState) -> Self {
        DatabasePool(Arc::clone(&app.db_pool))
    }
}
