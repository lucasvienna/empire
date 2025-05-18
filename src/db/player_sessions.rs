use std::fmt;
use std::sync::Arc;

use chrono::{Duration, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::domain::app_state::AppPool;
use crate::domain::player::session::{NewPlayerSession, PlayerSession, SessionKey};
use crate::domain::player::{Player, PlayerKey};
use crate::schema::player_session::dsl::*;
use crate::Result;

/// Repository for managing player sessions in the database.
///
/// Provides CRUD operations and additional functionality for managing player sessions,
/// including session creation, validation, expiration management, and cleanup of expired sessions.
///
/// # Fields
/// * `pool` - Thread-safe connection pool of type [`AppPool`] for database access
pub struct PlayerSessionRepository {
    pool: AppPool,
}

impl fmt::Debug for PlayerSessionRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlayerSessionRepository")
    }
}

pub type SessionPlayerTuple = (PlayerSession, Player);

impl PlayerSessionRepository {
    /// Creates a new repository instance with the provided database connection pool.
    ///
    /// # Parameters
    /// * `pool` - The database connection pool to use for database operations
    pub fn new(pool: &AppPool) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    /// Finds a player session by its unique identifier along with associated player information.
    ///
    /// # Parameters
    /// * `key` - The session key to search for
    ///
    /// # Returns
    /// * `Ok(Some((PlayerSession, Player)))` - If the session is found, returns both session and player data
    /// * `Ok(None)` - If no session matches the provided key
    /// * `Err` - If there was an error executing the database query
    pub fn find_by_id(&self, key: &SessionKey) -> Result<Option<SessionPlayerTuple>> {
        use crate::schema::player::dsl::player;
        let mut conn = self.pool.get()?;

        let tuple: Option<SessionPlayerTuple> = player_session
            .find(key)
            .inner_join(player)
            .select((PlayerSession::as_select(), Player::as_select()))
            .get_result::<SessionPlayerTuple>(&mut conn)
            .optional()?;

        Ok(tuple)
    }

    /// Creates a new player session in the database.
    ///
    /// # Parameters
    /// * `new_session` - The [`NewPlayerSession`] instance containing the session data to be persisted
    ///
    /// # Returns
    /// * `Ok(PlayerSession)` - The created session with its database-assigned values
    /// * `Err` - If there was an error creating the session (e.g., database connection issues)
    pub fn create(&self, new_session: NewPlayerSession) -> Result<PlayerSession> {
        let mut conn = self.pool.get()?;
        let session = diesel::insert_into(player_session)
            .values(new_session)
            .returning(PlayerSession::as_returning())
            .get_result(&mut conn)?;
        Ok(session)
    }

    /// Deletes a player session from the database by its unique identifier.
    ///
    /// # Parameters
    /// * `key` - The session key of the session to be deleted
    ///
    /// # Returns
    /// * `Ok(usize)` - The number of sessions deleted (typically 1 if successful, 0 if no session found)
    /// * `Err` - If there was an error executing the database deletion
    pub fn delete(&self, key: &SessionKey) -> Result<usize> {
        let mut conn = self.pool.get()?;
        let deleted_count = diesel::delete(player_session.find(key)).execute(&mut conn)?;
        Ok(deleted_count)
    }

    /// Deletes all sessions associated with a specific player.
    ///
    /// # Parameters
    /// * `player_key` - The unique identifier of the player whose sessions should be deleted
    ///
    /// # Returns
    /// * `Ok(usize)` - The number of sessions deleted
    /// * `Err` - If there was an error executing the database deletion
    pub fn delete_by_player(&self, player_key: &PlayerKey) -> Result<usize> {
        let mut conn = self.pool.get()?;
        let deleted_count =
            diesel::delete(player_session.filter(player_id.eq(player_key))).execute(&mut conn)?;
        Ok(deleted_count)
    }

    /// Refreshes a session's expiration date by extending it by 30 days from the current time.
    ///
    /// # Parameters
    /// * `key` - The session key of the session to be refreshed
    ///
    /// # Returns
    /// * `Ok(PlayerSession)` - The updated session with the new expiration date
    /// * `Err` - If there was an error updating the session
    pub fn refresh_token(&self, key: &SessionKey) -> Result<PlayerSession> {
        let mut conn = self.pool.get()?;
        let new_expiry_date = Utc::now() + Duration::days(30);
        let fresh_session = diesel::update(player_session.find(key))
            .set(expires_at.eq(new_expiry_date))
            .returning(PlayerSession::as_returning())
            .get_result(&mut conn)?;
        Ok(fresh_session)
    }
}
