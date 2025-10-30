use std::io::Write;
use std::str::{FromStr, from_utf8};

use chrono::{DateTime, Utc};
use derive_more::Display;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{AsExpression, FromSqlRow, deserialize, serialize};
use serde::{Deserialize, Serialize};

use crate::schema::faction;

/// Type alias for the primary key of factions, using FactionCode as the identifier.
pub type FactionKey = FactionCode;

#[derive(
	AsExpression,
	FromSqlRow,
	Serialize,
	Deserialize,
	Display,
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
	PartialOrd,
	Ord,
)]
#[diesel(sql_type = crate::schema::sql_types::FactionCode)]
#[serde(rename_all = "lowercase")]
/// Represents the available faction types in the game.
/// Each variant corresponds to a distinct playable faction, except Neutral.
#[derive(Default)]
pub enum FactionCode {
	/// The neutral faction, which is the default faction for new players.
	#[default]
	Neutral,
	Human,
	Orc,
	Elf,
	Dwarf,
	Goblin,
}

impl AsRef<str> for FactionCode {
	fn as_ref(&self) -> &str {
		match self {
			Self::Neutral => "neutral",
			Self::Human => "human",
			Self::Orc => "orc",
			Self::Elf => "elf",
			Self::Dwarf => "dwarf",
			Self::Goblin => "goblin",
		}
	}
}

impl FromStr for FactionCode {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"neutral" => Ok(FactionCode::Neutral),
			"human" => Ok(FactionCode::Human),
			"orc" => Ok(FactionCode::Orc),
			"elf" => Ok(FactionCode::Elf),
			"dwarf" => Ok(FactionCode::Dwarf),
			"goblin" => Ok(FactionCode::Goblin),
			other => Err(format!("Unrecognized enum variant: {other}")),
		}
	}
}

impl ToSql<crate::schema::sql_types::FactionCode, Pg> for FactionCode {
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
		out.write_all(self.as_ref().as_bytes())?;
		Ok(IsNull::No)
	}
}

impl FromSql<crate::schema::sql_types::FactionCode, Pg> for FactionCode {
	fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
		let s = from_utf8(bytes.as_bytes())?;
		Ok(Self::from_str(s)?)
	}
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = faction, check_for_backend(diesel::pg::Pg))]
/// Represents a faction entity in the game with its properties.
pub struct Faction {
	/// Unique identifier of the faction using FactionCode
	pub id: FactionKey,
	/// Display name of the faction
	pub name: String,
	/// Timestamp when the faction was created
	pub created_at: DateTime<Utc>,
	/// Timestamp when the faction was last updated
	pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = faction, check_for_backend(diesel::pg::Pg))]
/// Data structure for creating a new faction.
pub struct NewFaction {
	/// Unique identifier for the new faction
	pub id: FactionKey,
	/// Display name for the new faction
	pub name: String,
}

#[derive(Identifiable, AsChangeset, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = faction, check_for_backend(diesel::pg::Pg))]
/// Data structure for updating an existing faction's properties.
pub struct UpdateFaction {
	/// Unique identifier for the faction
	pub id: FactionKey,
	/// New display name for the faction
	pub name: Option<String>,
}
