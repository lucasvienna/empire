//! Contains domain entities for the building-unit type mapping.
//! Maps which building types can train which unit types.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use super::BuildingKey;
use crate::domain::unit::UnitType;
use crate::schema::building_unit_type;

/// Unique identifier for a building-unit type mapping
pub type BuildingUnitTypeKey = Uuid;

/// AIDEV-NOTE: Maps which building types can train which unit types.
/// e.g., Barracks -> Infantry, Stables -> Cavalry
#[derive(Queryable, Selectable, Identifiable, Serialize, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = building_unit_type, check_for_backend(diesel::pg::Pg))]
pub struct BuildingUnitType {
	pub id: BuildingUnitTypeKey,
	pub building_id: BuildingKey,
	pub unit_type: UnitType,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Data transfer object for creating a new building-unit type mapping
#[derive(Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = building_unit_type, check_for_backend(diesel::pg::Pg))]
pub struct NewBuildingUnitType {
	pub building_id: BuildingKey,
	pub unit_type: UnitType,
}
