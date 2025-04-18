use std::io::Write;

use chrono::{DateTime, Utc};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{
    deserialize, serialize, AsExpression, FromSqlRow, Identifiable, Insertable, Queryable,
    Selectable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::resource::ResourceType;
use crate::schema::modifiers;

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
#[diesel(sql_type = crate::schema::sql_types::ModifierType)]
#[serde(rename_all = "lowercase")]
pub enum ModifierType {
    Percentage,
    Flat,
    Multiplier,
}

impl ToSql<crate::schema::sql_types::ModifierType, Pg> for ModifierType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ModifierType::Percentage => out.write_all(b"percentage")?,
            ModifierType::Flat => out.write_all(b"flat")?,
            ModifierType::Multiplier => out.write_all(b"multiplier")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::ModifierType, Pg> for ModifierType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"percentage" => Ok(ModifierType::Percentage),
            b"flat" => Ok(ModifierType::Flat),
            b"multiplier" => Ok(ModifierType::Multiplier),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

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
#[diesel(sql_type = crate::schema::sql_types::ModTargetType)]
#[serde(rename_all = "lowercase")]
pub enum ModTargetType {
    Resource,
    Combat,
    Training,
    Research,
}

impl ToSql<crate::schema::sql_types::ModTargetType, Pg> for ModTargetType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ModTargetType::Resource => out.write_all(b"resource")?,
            ModTargetType::Combat => out.write_all(b"combat")?,
            ModTargetType::Training => out.write_all(b"training")?,
            ModTargetType::Research => out.write_all(b"research")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::ModTargetType, Pg> for ModTargetType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"resource" => Ok(ModTargetType::Resource),
            b"combat" => Ok(ModTargetType::Combat),
            b"training" => Ok(ModTargetType::Training),
            b"research" => Ok(ModTargetType::Research),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = modifiers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Modifier {
    pub id: PK,
    pub name: String,
    pub description: String,
    pub modifier_type: ModifierType,
    pub target_type: ModTargetType,
    pub target_resource: Option<ResourceType>,
    pub stacking_group: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = modifiers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewModifier {
    pub name: String,
    pub description: String,
    pub modifier_type: ModifierType,
    pub target_type: ModTargetType,
    pub target_resource: Option<ResourceType>,
    pub stacking_group: Option<String>,
}
