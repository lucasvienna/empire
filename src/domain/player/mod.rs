pub mod accumulator;
pub mod buildings;
pub mod resource;
pub mod session;
mod user_email;
mod user_name;

use std::fmt;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
pub use user_email::UserEmail;
pub use user_name::UserName;
use uuid::Uuid;

use crate::domain::factions::FactionCode;
use crate::schema::player;

/// User Primary Key
pub type PlayerKey = Uuid;

#[derive(Queryable, Selectable, Identifiable, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = player, check_for_backend(diesel::pg::Pg))]
pub struct Player {
    pub id: PlayerKey,
    pub name: String,
    pub pwd_hash: String,
    pub email: Option<String>,
    pub faction: FactionCode,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Player")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("password", &"[redacted]")
            .field("email", &self.email)
            .field("faction", &self.faction)
            .finish()
    }
}

#[derive(Insertable, PartialEq, Eq)]
#[diesel(table_name = player, check_for_backend(diesel::pg::Pg))]
pub struct NewPlayer {
    pub name: UserName,
    pub pwd_hash: String,
    pub email: Option<UserEmail>,
    pub faction: FactionCode,
}

impl fmt::Debug for NewPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NewPlayer")
            .field("name", &self.name)
            .field("password", &"[redacted]")
            .field("email", &self.email)
            .field("faction", &self.faction)
            .finish()
    }
}

#[derive(Identifiable, AsChangeset, Clone, PartialEq, Eq)]
#[diesel(table_name = player, check_for_backend(diesel::pg::Pg))]
pub struct UpdatePlayer {
    pub id: PlayerKey,
    pub name: Option<UserName>,
    pub pwd_hash: Option<String>,
    pub email: Option<UserEmail>,
    pub faction: Option<FactionCode>,
}

impl fmt::Debug for UpdatePlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UpdatePlayer")
            .field("name", &self.name)
            .field("password", &"[redacted]")
            .field("email", &self.email)
            .field("faction", &self.faction)
            .finish()
    }
}
