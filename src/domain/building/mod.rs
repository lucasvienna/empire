//! Contains domain entities and types related to buildings in the game.
//! Buildings are structures that can be constructed by factions and have various levels and counts.

pub mod level;
pub mod requirement;
pub mod resources;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

use crate::domain::factions::FactionKey;
use crate::schema::building;

/// Unique identifier for a building entity
pub type BuildingKey = i32;

/// Represents a building type that can be constructed in the game
#[derive(Queryable, Selectable, Identifiable, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[diesel(table_name = building, check_for_backend(diesel::pg::Pg))]
pub struct Building {
	pub id: BuildingKey,
	pub name: String,
	pub max_level: i32,
	pub max_count: i32,
	pub faction: FactionKey,
	pub starter: bool,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Data transfer object for creating a new building
#[derive(Insertable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building, check_for_backend(diesel::pg::Pg))]
pub struct NewBuilding {
	pub name: String,
	pub max_level: i32,
	pub max_count: i32,
	pub faction: FactionKey,
	pub starter: bool,
}

/// Data transfer object for updating an existing building
#[derive(Identifiable, AsChangeset, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building, check_for_backend(diesel::pg::Pg))]
pub struct UpdateBuilding {
	pub id: BuildingKey,
	pub name: Option<String>,
	pub max_level: Option<i32>,
	pub max_count: Option<i32>,
	pub faction: Option<FactionKey>,
	pub starter: Option<bool>,
}
