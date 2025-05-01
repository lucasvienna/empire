use std::io::Write;

use chrono::{DateTime, Utc};
use derive_more::Display;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{
    deserialize, serialize, AsChangeset, AsExpression, FromSqlRow, Identifiable, Insertable,
    Queryable, Selectable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{modifier, player};
use crate::schema::active_modifiers;

pub type ActiveModifierKey = Uuid;

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
#[diesel(sql_type = crate::schema::sql_types::ModifierSourceType)]
#[serde(rename_all = "lowercase")]
#[display("{_variant}")]
pub enum ModifierSourceType {
    Faction,
    Item,
    Skill,
    Research,
    Event,
}

impl ToSql<crate::schema::sql_types::ModifierSourceType, Pg> for ModifierSourceType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ModifierSourceType::Faction => out.write_all(b"faction")?,
            ModifierSourceType::Item => out.write_all(b"item")?,
            ModifierSourceType::Skill => out.write_all(b"skill")?,
            ModifierSourceType::Research => out.write_all(b"research")?,
            ModifierSourceType::Event => out.write_all(b"event")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::ModifierSourceType, Pg> for ModifierSourceType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"faction" => Ok(ModifierSourceType::Faction),
            b"item" => Ok(ModifierSourceType::Item),
            b"skill" => Ok(ModifierSourceType::Skill),
            b"research" => Ok(ModifierSourceType::Research),
            b"event" => Ok(ModifierSourceType::Event),
            _ => {
                let unrecognized_value = String::from_utf8_lossy(bytes.as_bytes());
                Err(format!("Unrecognized enum variant: {}", unrecognized_value).into())
            }
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = active_modifiers, check_for_backend(diesel::pg::Pg))]
pub struct ActiveModifier {
    pub id: ActiveModifierKey,
    pub player_id: player::PlayerKey,
    pub modifier_id: modifier::ModifierKey,
    pub started_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub source_type: ModifierSourceType,
    pub source_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = active_modifiers, check_for_backend(diesel::pg::Pg))]
pub struct NewActiveModifier {
    pub player_id: player::PlayerKey,
    pub modifier_id: modifier::ModifierKey,
    pub started_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub source_type: ModifierSourceType,
    pub source_id: Option<Uuid>,
}

#[derive(Identifiable, AsChangeset, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = active_modifiers, check_for_backend(diesel::pg::Pg))]
pub struct UpdateActiveModifier {
    pub id: ActiveModifierKey,
    pub started_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub source_type: Option<ModifierSourceType>,
    pub source_id: Option<Uuid>,
}
