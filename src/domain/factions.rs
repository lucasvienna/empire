use std::io::Write;

use chrono::{DateTime, Utc};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};

use crate::schema::faction;

/// Type alias for the primary key of factions, using FactionCode as the identifier.
pub type FactionKey = FactionCode;

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
#[diesel(sql_type = crate::schema::sql_types::FactionCode)]
#[serde(rename_all = "lowercase")]
/// Represents the available faction types in the game.
/// Each variant corresponds to a distinct playable faction, except Neutral.
pub enum FactionCode {
    /// The neutral faction, which is the default faction for new players.
    Neutral,
    Human,
    Orc,
    Elf,
    Dwarf,
    Goblin,
}

impl ToSql<crate::schema::sql_types::FactionCode, Pg> for FactionCode {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            FactionCode::Neutral => out.write_all(b"neutral")?,
            FactionCode::Human => out.write_all(b"human")?,
            FactionCode::Orc => out.write_all(b"orc")?,
            FactionCode::Elf => out.write_all(b"elf")?,
            FactionCode::Dwarf => out.write_all(b"dwarf")?,
            FactionCode::Goblin => out.write_all(b"goblin")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::FactionCode, Pg> for FactionCode {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"neutral" => Ok(FactionCode::Neutral),
            b"human" => Ok(FactionCode::Human),
            b"orc" => Ok(FactionCode::Orc),
            b"elf" => Ok(FactionCode::Elf),
            b"dwarf" => Ok(FactionCode::Dwarf),
            b"goblin" => Ok(FactionCode::Goblin),
            _ => {
                let unrecognized_value = String::from_utf8_lossy(bytes.as_bytes());
                Err(format!("Unrecognized enum variant: {}", unrecognized_value).into())
            }
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = faction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
#[diesel(table_name = faction)]
/// Data structure for creating a new faction.
pub struct NewFaction {
    /// Unique identifier for the new faction
    pub id: FactionKey,
    /// Display name for the new faction
    pub name: String,
}

#[derive(AsChangeset, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = faction)]
/// Data structure for updating an existing faction's properties.
pub struct UpdateFaction {
    /// New display name for the faction
    pub name: String,
}
