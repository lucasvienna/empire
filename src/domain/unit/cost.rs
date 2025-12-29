//! Contains domain entities for unit training costs.
//! Each unit can have multiple resource costs associated with it.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use super::{Unit, UnitKey};
use crate::domain::player::resource::ResourceType;
use crate::schema::unit_cost;

/// Unique identifier for a unit cost entity
pub type UnitCostKey = Uuid;

/// Represents a resource cost for training a unit
#[derive(
	Queryable, Selectable, Identifiable, Associations, Serialize, Debug, Clone, PartialEq, Eq,
)]
#[diesel(table_name = unit_cost, belongs_to(Unit), check_for_backend(diesel::pg::Pg))]
pub struct UnitCost {
	pub id: UnitCostKey,
	pub unit_id: UnitKey,
	pub resource: ResourceType,
	pub amount: i64,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Data transfer object for creating a new unit cost
#[derive(Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = unit_cost, check_for_backend(diesel::pg::Pg))]
pub struct NewUnitCost {
	pub unit_id: UnitKey,
	pub resource: ResourceType,
	pub amount: i64,
}
