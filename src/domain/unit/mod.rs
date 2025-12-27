//! Contains domain entities and types related to units in the game.
//! Units are military entities that can be trained by players and used in combat.

pub mod cost;
pub mod player_unit;
pub mod training;

use std::io::Write;
use std::str::from_utf8;

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

impl AsRef<str> for UnitType {
	fn as_ref(&self) -> &str {
		match self {
			UnitType::Infantry => "infantry",
			UnitType::Ranged => "ranged",
			UnitType::Cavalry => "cavalry",
			UnitType::Artillery => "artillery",
			UnitType::Magical => "magical",
		}
	}
}

impl ToSql<crate::schema::sql_types::UnitType, Pg> for UnitType {
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
		out.write_all(self.as_ref().as_bytes())?;
		Ok(IsNull::No)
	}
}

impl FromSql<crate::schema::sql_types::UnitType, Pg> for UnitType {
	fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
		match from_utf8(bytes.as_bytes())? {
			"infantry" => Ok(UnitType::Infantry),
			"ranged" => Ok(UnitType::Ranged),
			"cavalry" => Ok(UnitType::Cavalry),
			"artillery" => Ok(UnitType::Artillery),
			"magical" => Ok(UnitType::Magical),
			other => Err(format!("Unrecognized enum variant: {other}").into()),
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
