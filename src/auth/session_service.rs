//! Session management service module.
//!
//! This module provides functionality for creating, validating, and managing user sessions.
//! It handles all session-related operations, including:
//! - Generating secure session tokens
//! - Creating new sessions for players
//! - Validating existing session tokens
//! - Invalidating individual or all sessions for a player
use blake2::{Blake2s256, Digest};
use chrono::{Duration, Utc};
use data_encoding::BASE32_NOPAD_NOCASE;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::db::player_sessions::{self, SessionPlayerTuple};
use crate::db::DbConn;
use crate::domain::player::session::{NewPlayerSession, PlayerSession, SessionKey};
use crate::domain::player::PlayerKey;
use crate::{Error, ErrorKind, Result};

/// Generates a new random session token.
///
/// Creates a cryptographically secure random token by generating 32 random bytes
/// and encoding them using BASE32 encoding.
///
/// # Returns
/// A String containing the BASE32 encoded session token.
#[instrument()]
pub fn generate_session_token() -> String {
	debug!("Generating new session token");
	let bytes: [u8; 32] = rand::random();
	let token = BASE32_NOPAD_NOCASE.encode(&bytes);
	trace!("Generated token of length {}", token.len());
	token
}

/// Creates a new session for a player. The session is associated with the provided token.
///
/// # Parameters
/// * `conn` - Database connection
/// * `token` - The session token to associate with the session
/// * `player_key` - The unique identifier of the player
///
/// # Returns
/// A new Session instance containing the session details
#[instrument(skip(conn, token))]
pub fn create_session(
	conn: &mut DbConn,
	token: String,
	player_key: &PlayerKey,
) -> Result<PlayerSession> {
	debug!("Creating new session");
	let session_id = encode_session_token(token);
	trace!("Encoded session ID: {}", session_id);

	let expires_at = Utc::now() + Duration::days(30);
	let new_session = NewPlayerSession {
		id: session_id,
		player_id: *player_key,
		expires_at,
	};
	trace!("New session expires at: {}", expires_at);

	let session = player_sessions::create(conn, new_session)?;
	info!("Created new session");
	Ok(session)
}

/// Validates a session token and retrieves associated session information.
///
/// # Parameters
/// * `conn` - Database connection
/// * `token` - The session token to validate
///
/// # Returns
/// * `Ok(SessionPlayerTuple)` - If the token is valid, returns the session and player information
/// * `Err` - If the token is invalid, or an error occurs during validation
#[instrument(skip(conn, token))]
pub fn validate_session_token(conn: &mut DbConn, token: String) -> Result<SessionPlayerTuple> {
	debug!("Starting session token validation");
	let session_id = encode_session_token(token);
	trace!("Encoded session ID: {}", session_id);

	let player_session = player_sessions::find_by_id(conn, &session_id)?;

	if player_session.is_none() {
		warn!(%session_id, "No session found");
		return Err(Error::from((
			ErrorKind::NoSessionError,
			"No session found for the provided token.",
		)));
	}

	let (session, player) = player_session.unwrap();
	debug!(player_key = %player.id, "Found session for player");

	// Check if the session has expired.
	if session.expires_at <= Utc::now() {
		warn!(player_key = %player.id, "Session expired for player");
		let count = player_sessions::delete(conn, &session.id)?;
		debug_assert_eq!(count, 1, "Expected exactly one session to be deleted.");
		return Err(Error::from((
			ErrorKind::SessionExpiredError,
			"The provided session has expired.",
		)));
	}

	// Refresh the session if it's within 15 days of expiration.
	if session.expires_at - Duration::days(15) < Utc::now() {
		debug!(player_key = %player.id, "Refreshing session for player");
		let refreshed_session = player_sessions::refresh_token(conn, &session.id)?;
		info!(player_key = %player.id, "Session refreshed for player");
		return Ok((refreshed_session, player));
	}

	info!(player_key = %player.id, "Session validated successfully for player");
	Ok((session, player))
}

/// Invalidates a specific session.
///
/// # Parameters
/// * `conn` - Database connection
/// * `session_id` - The unique identifier of the session to invalidate
#[instrument(skip(conn))]
pub fn invalidate_session(conn: &mut DbConn, session_id: &SessionKey) {
	debug!("Starting invalidate session: {}", session_id);
	match player_sessions::delete(conn, session_id) {
		Ok(count) => {
			if count > 0 {
				info!("Successfully invalidated session: {}", session_id);
			} else {
				debug!("No session found to invalidate with ID: {}", session_id);
			}
		}
		Err(e) => {
			error!("Failed to delete session {}: {}", session_id, e);
		}
	}
}

/// Invalidates all sessions associated with a specific player.
///
/// # Parameters
/// * `conn` - Database connection
/// * `player_key` - The unique identifier of the player whose sessions should be invalidated
#[instrument(skip(conn))]
pub fn invalidate_all_sessions(conn: &mut DbConn, player_key: &PlayerKey) {
	debug!(
		"Starting invalidate all sessions for player: {}",
		player_key
	);
	match player_sessions::delete_by_player(conn, player_key) {
		Ok(count) => {
			info!("Invalidated {} sessions for player: {}", count, player_key);
		}
		Err(e) => {
			error!("Failed to delete sessions for player {}: {}", player_key, e);
		}
	}
}

/// Encodes a session token using Blake2s256 hashing algorithm.
///
/// # Parameters
/// * `token` - The token to be encoded, must implement AsRef<[u8]>
///
/// # Returns
/// A String containing the hexadecimal representation of the hashed token
#[instrument(skip_all)]
fn encode_session_token(token: impl AsRef<[u8]>) -> String {
	trace!("Encoding session token");
	let mut hasher = Blake2s256::new();
	Digest::update(&mut hasher, token);
	let encoded_token = hasher.finalize();
	let result = format!("{encoded_token:x}");
	trace!("Token encoded successfully");
	result
}
