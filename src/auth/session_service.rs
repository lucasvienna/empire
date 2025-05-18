//! Session management service module.
//!
//! This module provides functionality for creating, validating, and managing user sessions.
//! It handles session token generation, session creation, validation, and invalidation operations.

use std::fmt;

use axum::extract::FromRef;
use blake2::{Blake2s256, Digest};
use chrono::{Duration, Utc};
use data_encoding::BASE32_NOPAD_NOCASE;
use tracing::warn;

use crate::db::player_sessions::{PlayerSessionRepository, SessionPlayerTuple};
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::player::session::{NewPlayerSession, PlayerSession, SessionKey};
use crate::domain::player::PlayerKey;
use crate::game::service::ApiService;
use crate::{Error, ErrorKind, Result};

/// Service for managing player sessions, providing functionality for session creation,
/// validation, and invalidation.
///
/// This service handles all session-related operations including:
/// - Generating secure session tokens
/// - Creating new sessions for players
/// - Validating existing session tokens
/// - Invalidating individual or all sessions for a player
pub struct SessionService {
    repo: PlayerSessionRepository,
}

impl fmt::Debug for SessionService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SessionService")
    }
}

impl FromRef<AppState> for SessionService {
    fn from_ref(state: &AppState) -> Self {
        Self::new(&state.db_pool)
    }
}

impl ApiService for SessionService {
    fn new(pool: &AppPool) -> Self {
        Self::new(pool)
    }
}

impl SessionService {
    /// Creates a new instance of SessionService.
    ///
    /// # Parameters
    /// * `pool` - The database connection pool to use for session operations
    ///
    /// # Returns
    /// A new SessionService instance configured with the provided connection pool
    pub fn new(pool: &AppPool) -> Self {
        Self {
            repo: PlayerSessionRepository::new(pool),
        }
    }

    /// Generates a new random session token.
    ///
    /// Creates a cryptographically secure random token by generating 32 random bytes
    /// and encoding them using BASE32 encoding.
    ///
    /// # Returns
    /// A String containing the BASE32 encoded session token.
    pub fn generate_session_token(&self) -> String {
        let bytes: [u8; 32] = rand::random();
        BASE32_NOPAD_NOCASE.encode(&bytes)
    }

    /// Creates a new session for a player. The session is associated with the provided token.
    ///
    /// # Parameters
    /// * `token` - The session token to associate with the session
    /// * `player_key` - The unique identifier of the player
    ///
    /// # Returns
    /// A new Session instance containing the session details
    pub fn create_session(&self, token: String, player_key: &PlayerKey) -> Result<PlayerSession> {
        let session_id = self.encode_session_token(token);

        let new_session = NewPlayerSession {
            id: session_id,
            player_id: *player_key,
            expires_at: Utc::now() + Duration::days(30),
        };

        self.repo.create(new_session)
    }

    /// Validates a session token and retrieves associated session information.
    ///
    /// # Parameters
    /// * `token` - The session token to validate
    ///
    /// # Returns
    /// * `Ok(SessionPlayerTuple)` - If the token is valid, returns the session and player information
    /// * `Err` - If the token is invalid or an error occurs during validation
    pub fn validate_session_token(&self, token: String) -> Result<SessionPlayerTuple> {
        let session_id = self.encode_session_token(token);
        let player_session = self.repo.find_by_id(&session_id)?;

        if player_session.is_none() {
            return Err(Error::from((
                ErrorKind::NoSessionError,
                "No session found for the provided token.",
            )));
        }
        let (session, player) = player_session.unwrap();

        // Check if the session has expired.
        if session.expires_at <= Utc::now() {
            let count = self.repo.delete(&session.id)?;
            debug_assert_eq!(count, 1, "Expected exactly one session to be deleted.");
            return Err(Error::from((
                ErrorKind::SessionExpiredError,
                "The provided session has expired.",
            )));
        }

        // Refresh the session if it's within 15 days of expiration.
        if session.expires_at - Duration::days(15) < Utc::now() {
            let session = self.repo.refresh_token(&session.id)?;
            return Ok((session, player));
        }

        Ok((session, player))
    }

    /// Invalidates a specific session.
    ///
    /// # Parameters
    /// * `session_id` - The unique identifier of the session to invalidate
    pub fn invalidate_session(&self, session_id: &SessionKey) {
        let _ = self.repo.delete(session_id).map_err(|_| {
            warn!("Failed to delete session: {}", session_id);
        });
    }

    /// Invalidates all sessions associated with a specific player.
    ///
    /// # Parameters
    /// * `player_key` - The unique identifier of the player whose sessions should be invalidated
    pub fn invalidate_all_sessions(&self, player_key: &PlayerKey) {
        let _ = self.repo.delete_by_player(player_key).map_err(|_| {
            warn!("Failed to delete player sessions: {}", player_key);
        });
    }

    /// Encodes a session token using Blake2s256 hashing algorithm.
    ///
    /// # Parameters
    /// * `token` - The token to be encoded, must implement AsRef<[u8]>
    ///
    /// # Returns
    /// A String containing the hexadecimal representation of the hashed token
    fn encode_session_token(&self, token: impl AsRef<[u8]>) -> String {
        let mut hasher = Blake2s256::new();
        Digest::update(&mut hasher, token);
        let encoded_token = hasher.finalize();
        format!("{:x}", encoded_token)
    }
}
