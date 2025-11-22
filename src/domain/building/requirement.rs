//! Building requirements management module
//!
//! This module defines the structure and types for managing requirements
//! associated with buildings, including buildings or tech tree nodes.

use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::domain::building::BuildingKey;
use crate::domain::building::level::BuildingLevelKey;
use crate::schema::building_requirement;

/// Unique identifier type for building requirements
pub type BuildingRequirementKey = Uuid;

/// Represents the required buildings or tech tree nodes necessary to upgrade a
/// building to a given level
///
/// These are mutually exclusive and enforced by database constraints. You can
/// always assume that if a building requirement exists, then the building ID
/// and level will be present and the tech will be [None]. The converse is also
/// true.
#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building_requirement, check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(BuildingLevel))]
pub struct BuildingRequirement {
	/// Unique identifier for the building requirement record
	pub id: BuildingRequirementKey,
	/// Reference to the associated building level
	pub building_level_id: BuildingLevelKey,
	/// Reference to the required building
	pub required_building_id: Option<BuildingKey>,
	/// Required building level, if any. If not specified, the requirement is
	/// for the building itself to be built.
	pub required_building_level: Option<i32>,
	/// Reference to the required tech tree node
	pub required_tech_id: Option<Uuid>,
	/// Required tech tree node level, if any. If not specified, the requirement is
	/// for the tech tree node itself to be unlocked.
	pub required_tech_level: Option<i32>,
	/// Timestamp when the resource record was created
	pub created_at: DateTime<Utc>,
	/// Timestamp when the resource record was last updated
	pub updated_at: DateTime<Utc>,
}

/// Data required to create a new building requirement
#[derive(Insertable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building_requirement, check_for_backend(diesel::pg::Pg))]
pub struct NewBuildingRequirement {
	/// Reference to the associated building level
	pub building_level_id: BuildingLevelKey,
	/// Reference to the required building
	pub required_building_id: Option<BuildingKey>,
	/// Required building level, if any
	pub required_building_level: Option<i32>,
	/// Reference to the required tech tree node
	pub required_tech_id: Option<Uuid>,
	/// Required tech tree node level, if any
	pub required_tech_level: Option<i32>,
}

/// Data structure for updating an existing building requirement
#[derive(Identifiable, AsChangeset, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building_requirement, check_for_backend(diesel::pg::Pg))]
pub struct UpdateBuildingRequirement {
	/// Unique identifier for the building requirement record
	pub id: BuildingRequirementKey,
	/// Reference to the associated building level
	pub building_level_id: Option<BuildingLevelKey>,
	/// Reference to the required building
	pub required_building_id: Option<BuildingKey>,
	/// Required building level, if any
	pub required_building_level: Option<i32>,
	/// Reference to the required tech tree node
	pub required_tech_id: Option<Uuid>,
	/// Required tech tree node level, if any
	pub required_tech_level: Option<i32>,
}
