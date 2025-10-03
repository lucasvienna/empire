//! Database access layer for player session entities.
//!
//! This module manages player authentication sessions, providing functionality
//! for creating, finding, deleting, and refreshing user sessions. It handles
//! session expiration, cleanup operations, and maintains the relationship
//! between sessions and players for secure authentication management.

use chrono::{Duration, Utc};
use diesel::prelude::*;

use crate::db::DbConn;
use crate::domain::player::session::{NewPlayerSession, PlayerSession, SessionKey};
use crate::domain::player::{Player, PlayerKey};
use crate::schema::player_session::dsl::*;
use crate::Result;

pub type SessionPlayerTuple = (PlayerSession, Player);

/// Finds a player session by its unique identifier along with associated player information.
///
/// # Parameters
/// * `conn` - Database connection
/// * `key` - The session key to search for
///
/// # Returns
/// * `Ok(Some((PlayerSession, Player)))` - If the session is found, returns both session and player data
/// * `Ok(None)` - If no session matches the provided key
/// * `Err` - If there was an error executing the database query
pub fn find_by_id(conn: &mut DbConn, key: &SessionKey) -> Result<Option<SessionPlayerTuple>> {
	use crate::schema::player::dsl::player;

	let tuple: Option<SessionPlayerTuple> = player_session
		.find(key)
		.inner_join(player)
		.select((PlayerSession::as_select(), Player::as_select()))
		.get_result::<SessionPlayerTuple>(conn)
		.optional()?;

	Ok(tuple)
}

/// Creates a new player session in the database.
///
/// # Parameters
/// * `conn` - Database connection
/// * `new_session` - The [`NewPlayerSession`] instance containing the session data to be persisted
///
/// # Returns
/// * `Ok(PlayerSession)` - The created session with its database-assigned values
/// * `Err` - If there was an error creating the session (e.g., database connection issues)
pub fn create(conn: &mut DbConn, new_session: NewPlayerSession) -> Result<PlayerSession> {
	let session = diesel::insert_into(player_session)
		.values(new_session)
		.returning(PlayerSession::as_returning())
		.get_result(conn)?;
	Ok(session)
}

/// Deletes a player session from the database by its unique identifier.
///
/// # Parameters
/// * `conn` - Database connection
/// * `key` - The session key of the session to be deleted
///
/// # Returns
/// * `Ok(usize)` - The number of sessions deleted (typically 1 if successful, 0 if no session found)
/// * `Err` - If there was an error executing the database deletion
pub fn delete(conn: &mut DbConn, key: &SessionKey) -> Result<usize> {
	let deleted_count = diesel::delete(player_session.find(key)).execute(conn)?;
	Ok(deleted_count)
}

/// Deletes all sessions associated with a specific player.
///
/// # Parameters
/// * `conn` - Database connection
/// * `player_key` - The unique identifier of the player whose sessions should be deleted
///
/// # Returns
/// * `Ok(usize)` - The number of sessions deleted
/// * `Err` - If there was an error executing the database deletion
pub fn delete_by_player(conn: &mut DbConn, player_key: &PlayerKey) -> Result<usize> {
	let deleted_count =
		diesel::delete(player_session.filter(player_id.eq(player_key))).execute(conn)?;
	Ok(deleted_count)
}

/// Refreshes a session's expiration date by extending it by 30 days from the current time.
///
/// # Parameters
/// * `conn` - Database connection
/// * `key` - The session key of the session to be refreshed
///
/// # Returns
/// * `Ok(PlayerSession)` - The updated session with the new expiration date
/// * `Err` - If there was an error updating the session
pub fn refresh_token(conn: &mut DbConn, key: &SessionKey) -> Result<PlayerSession> {
	let new_expiry_date = Utc::now() + Duration::days(30);
	let fresh_session = diesel::update(player_session.find(key))
		.set(expires_at.eq(new_expiry_date))
		.returning(PlayerSession::as_returning())
		.get_result(conn)?;
	Ok(fresh_session)
}
