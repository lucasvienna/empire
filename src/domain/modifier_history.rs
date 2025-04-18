use std::io::Write;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::domain::active_modifier::ModifierSourceType;
use crate::domain::{modifier, user};
use crate::schema::modifier_history;

pub type PK = Uuid;

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
#[diesel(sql_type = crate::schema::sql_types::ModifierActionType)]
#[serde(rename_all = "lowercase")]
pub enum ModifierActionType {
    Applied,
    Expired,
    Removed,
    Updated,
}

impl ToSql<crate::schema::sql_types::ModifierActionType, Pg> for ModifierActionType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ModifierActionType::Applied => out.write_all(b"applied")?,
            ModifierActionType::Expired => out.write_all(b"expired")?,
            ModifierActionType::Removed => out.write_all(b"removed")?,
            ModifierActionType::Updated => out.write_all(b"updated")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::ModifierActionType, Pg> for ModifierActionType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"applied" => Ok(ModifierActionType::Applied),
            b"expired" => Ok(ModifierActionType::Expired),
            b"removed" => Ok(ModifierActionType::Removed),
            b"updated" => Ok(ModifierActionType::Updated),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone)]
#[diesel(table_name = modifier_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ModifierHistory {
    pub id: PK,
    pub user_id: user::PK,
    pub modifier_id: modifier::PK,
    pub action_type: ModifierActionType,
    pub magnitude: BigDecimal,
    pub occurred_at: DateTime<Utc>,
    pub source_type: ModifierSourceType,
    pub source_id: Option<Uuid>,
    pub previous_state: Option<JsonValue>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = modifier_history)]
pub struct NewModifierHistory {
    pub user_id: user::PK,
    pub modifier_id: modifier::PK,
    pub action_type: ModifierActionType,
    pub magnitude: BigDecimal,
    pub source_type: ModifierSourceType,
    pub source_id: Option<Uuid>,
    pub previous_state: Option<JsonValue>,
    pub reason: Option<String>,
}
