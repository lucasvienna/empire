//! Building level management module.
//!
//! This module provides types and structures for managing building levels in the game.
//! It includes functionality for:
//! - Representing building levels and their requirements
//! - Handling building level creation and updates
//! - Managing building upgrade requirements and timing
//! - Tracking resource requirements for building upgrades

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::building::BuildingKey;
use crate::schema::building_level;

/// Unique identifier type for building levels
pub type BuildingLevelKey = Uuid;

/// Represents a building level in the game with its requirements and upgrade details
#[derive(
	Queryable, Selectable, Identifiable, AsChangeset, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
#[diesel(table_name = building_level)]
#[diesel(belongs_to(Building))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BuildingLevel {
	pub id: BuildingLevelKey,
	pub building_id: BuildingKey,
	#[diesel(column_name = level)]
	pub building_level: i32,
	pub upgrade_seconds: i64,
	pub req_food: Option<i64>,
	pub req_wood: Option<i64>,
	pub req_stone: Option<i64>,
	pub req_gold: Option<i64>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Data required to create a new building level
#[derive(Insertable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building_level, check_for_backend(diesel::pg::Pg))]
pub struct NewBuildingLevel {
	pub building_id: i32,
	#[diesel(column_name = level)]
	pub building_level: i32,
	pub upgrade_seconds: i64,
	pub req_food: Option<i64>,
	pub req_wood: Option<i64>,
	pub req_stone: Option<i64>,
	pub req_gold: Option<i64>,
}

/// Data structure for updating an existing building level
#[derive(Identifiable, AsChangeset, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building_level, check_for_backend(diesel::pg::Pg))]
pub struct UpdateBuildingLevel {
	pub id: BuildingLevelKey,
	pub upgrade_seconds: Option<i64>,
	pub req_food: Option<i64>,
	pub req_wood: Option<i64>,
	pub req_stone: Option<i64>,
	pub req_gold: Option<i64>,
}
