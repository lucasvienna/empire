use std::io::Write;

use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};

use crate::schema::factions;

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
pub enum FactionCode {
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
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = factions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Faction {
    pub id: PK,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = factions)]
pub struct NewFaction {
    pub id: PK,
    pub name: String,
}

pub type PK = FactionCode;
