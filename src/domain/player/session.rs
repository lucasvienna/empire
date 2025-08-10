use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};

use crate::domain::player::{Player, PlayerKey};
use crate::schema::player_session;

/// Primary key for PlayerSession
pub type SessionKey = String;

/// Represents a user session with its associated metadata.
#[derive(
	Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
#[diesel(belongs_to(Player))]
#[diesel(table_name = player_session, check_for_backend(diesel::pg::Pg))]
pub struct PlayerSession {
	/// Unique identifier for the session
	pub id: SessionKey,
	/// The key of the player who owns this session
	pub player_id: PlayerKey,
	/// Timestamp when the session expires
	pub expires_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = player_session, check_for_backend(diesel::pg::Pg))]
pub struct NewPlayerSession {
	pub id: SessionKey,
	pub player_id: PlayerKey,
	pub expires_at: DateTime<Utc>,
}
