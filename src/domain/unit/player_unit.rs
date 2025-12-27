//! Contains domain entities for player unit ownership.
//! Tracks the quantity of each unit type a player owns.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use super::{Unit, UnitKey};
use crate::domain::player::{Player, PlayerKey};
use crate::schema::player_unit;

/// Unique identifier for a player unit entity
pub type PlayerUnitKey = Uuid;

/// Represents a player's ownership of a specific unit type
#[derive(
	Queryable, Selectable, Identifiable, Associations, Serialize, Debug, Clone, PartialEq, Eq,
)]
#[diesel(belongs_to(Player))]
#[diesel(belongs_to(Unit))]
#[diesel(table_name = player_unit, check_for_backend(diesel::pg::Pg))]
pub struct PlayerUnit {
	pub id: PlayerUnitKey,
	pub player_id: PlayerKey,
	pub unit_id: UnitKey,
	pub quantity: i32,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Data transfer object for creating a new player unit entry
#[derive(Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = player_unit, check_for_backend(diesel::pg::Pg))]
pub struct NewPlayerUnit {
	pub player_id: PlayerKey,
	pub unit_id: UnitKey,
	pub quantity: i32,
}

/// Data transfer object for updating a player unit entry
#[derive(Identifiable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = player_unit, check_for_backend(diesel::pg::Pg))]
pub struct UpdatePlayerUnit {
	pub id: PlayerUnitKey,
	pub quantity: Option<i32>,
}
