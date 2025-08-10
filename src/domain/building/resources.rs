//! Building resource management module
//!
//! This module defines the structure and types for managing resources associated with buildings,
//! including resource amounts, storage capacities, and accumulation limits.

use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable, Selectable};
use uuid::Uuid;

use crate::domain::building::BuildingKey;
use crate::schema::building_resource;

/// Unique identifier type for building resources
pub type BuildingResourceKey = Uuid;

/// Represents the resources and capacities associated with a building
///
/// This structure tracks various resource production amounts, storage capacities,
/// and accumulation capacities for a building at a specific level.
#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building_resource, check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Building))]
pub struct BuildingResource {
	/// Unique identifier for the building resource record
	pub id: BuildingResourceKey,
	/// Reference to the associated building
	pub building_id: BuildingKey,
	/// Current level of the building
	pub building_level: i32,
	/// Current population supported by the building
	pub population: i64,
	/// Food produced per hour
	pub food: i64,
	/// Wood produced per hour
	pub wood: i64,
	/// Stone produced per hour
	pub stone: i64,
	/// Gold produced per hour
	pub gold: i64,
	/// Maximum storage capacity for food
	pub food_cap: i64,
	/// Maximum storage capacity for wood
	pub wood_cap: i64,
	/// Maximum storage capacity for stone
	pub stone_cap: i64,
	/// Maximum storage capacity for gold
	pub gold_cap: i64,
	/// Maximum accumulation capacity for food
	pub food_acc_cap: i64,
	/// Maximum accumulation capacity for wood
	pub wood_acc_cap: i64,
	/// Maximum accumulation capacity for stone
	pub stone_acc_cap: i64,
	/// Maximum accumulation capacity for gold
	pub gold_acc_cap: i64,
	/// Timestamp when the resource record was created
	pub created_at: DateTime<Utc>,
	/// Timestamp when the resource record was last updated
	pub updated_at: DateTime<Utc>,
}
