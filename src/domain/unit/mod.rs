//! Contains domain entities and types related to units in the game.
//! Units are military entities that can be trained by players and used in combat.

pub mod cost;
pub mod player_unit;
pub mod training;

use std::io::Write;

use chrono::{DateTime, Utc};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{AsExpression, FromSqlRow, deserialize, serialize};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::unit;

/// Unique identifier for a unit entity
pub type UnitKey = Uuid;

/// Represents the type/class of a unit
#[derive(
	AsExpression,
	FromSqlRow,
	Serialize,
	Deserialize,
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
	PartialOrd,
	Ord,
)]
#[diesel(sql_type = crate::schema::sql_types::UnitType)]
#[serde(rename_all = "lowercase")]
pub enum UnitType {
	Infantry,
	Ranged,
	Cavalry,
	Artillery,
	Magical,
}

impl ToSql<crate::schema::sql_types::UnitType, Pg> for UnitType {
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
		match *self {
			UnitType::Infantry => out.write_all(b"infantry")?,
			UnitType::Ranged => out.write_all(b"ranged")?,
			UnitType::Cavalry => out.write_all(b"cavalry")?,
			UnitType::Artillery => out.write_all(b"artillery")?,
			UnitType::Magical => out.write_all(b"magical")?,
		}
		Ok(IsNull::No)
	}
}

impl FromSql<crate::schema::sql_types::UnitType, Pg> for UnitType {
	fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
		match bytes.as_bytes() {
			b"infantry" => Ok(UnitType::Infantry),
			b"ranged" => Ok(UnitType::Ranged),
			b"cavalry" => Ok(UnitType::Cavalry),
			b"artillery" => Ok(UnitType::Artillery),
			b"magical" => Ok(UnitType::Magical),
			_ => {
				let unrecognized_value = String::from_utf8_lossy(bytes.as_bytes());
				Err(format!("Unrecognized enum variant: {unrecognized_value}").into())
			}
		}
	}
}

/// Represents a unit definition that can be trained in the game
#[derive(Queryable, Selectable, Identifiable, Serialize, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = unit, check_for_backend(diesel::pg::Pg))]
pub struct Unit {
	pub id: UnitKey,
	pub name: String,
	pub unit_type: UnitType,
	pub base_atk: i32,
	pub base_def: i32,
	pub base_training_seconds: i32,
	pub description: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Data transfer object for creating a new unit
#[derive(Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = unit, check_for_backend(diesel::pg::Pg))]
pub struct NewUnit {
	pub name: String,
	pub unit_type: UnitType,
	pub base_atk: i32,
	pub base_def: i32,
	pub base_training_seconds: i32,
	pub description: Option<String>,
}

/// Data transfer object for updating an existing unit
#[derive(Identifiable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = unit, check_for_backend(diesel::pg::Pg))]
pub struct UpdateUnit {
	pub id: UnitKey,
	pub name: Option<String>,
	pub unit_type: Option<UnitType>,
	pub base_atk: Option<i32>,
	pub base_def: Option<i32>,
	pub base_training_seconds: Option<i32>,
	pub description: Option<String>,
}
