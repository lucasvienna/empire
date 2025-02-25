use std::ops::Deref;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use diesel::r2d2::Pool;
use tracing::{error, trace};

use crate::db::{DbConn, DbPool};
use crate::domain::app_state::AppState;

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
pub struct DatabaseConnection(pub DbConn);

impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    DbPool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);
        let conn = pool.get().map_err(|err| {
            error!("Failed to get a database connection: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?;
        trace!("Acquired a database connection.");

        Ok(Self(conn))
    }
}

impl FromRef<AppState> for DbPool {
    fn from_ref(state: &AppState) -> Self {
        state.db_pool.deref().clone()
    }
}
