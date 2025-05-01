use std::io::Write;

use bigdecimal::BigDecimal;
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

use crate::domain::player::resource::ResourceType;
use crate::schema::modifiers;

pub mod active_modifier;
pub mod full_modifier;
pub mod modifier_history;
pub mod modifier_state;

pub type ModifierKey = Uuid;

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
            _ => {
                let unrecognized_value = String::from_utf8_lossy(bytes.as_bytes());
                Err(format!("Unrecognized enum variant: {}", unrecognized_value).into())
            }
        }
    }
}

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
#[diesel(sql_type = crate::schema::sql_types::ModifierTarget)]
#[serde(rename_all = "lowercase")]
#[display("{_variant}")]
pub enum ModifierTarget {
    Resource,
    Combat,
    Training,
    Research,
}

impl ToSql<crate::schema::sql_types::ModifierTarget, Pg> for ModifierTarget {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ModifierTarget::Resource => out.write_all(b"resource")?,
            ModifierTarget::Combat => out.write_all(b"combat")?,
            ModifierTarget::Training => out.write_all(b"training")?,
            ModifierTarget::Research => out.write_all(b"research")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::ModifierTarget, Pg> for ModifierTarget {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"resource" => Ok(ModifierTarget::Resource),
            b"combat" => Ok(ModifierTarget::Combat),
            b"training" => Ok(ModifierTarget::Training),
            b"research" => Ok(ModifierTarget::Research),
            _ => {
                let unrecognized_value = String::from_utf8_lossy(bytes.as_bytes());
                Err(format!("Unrecognized enum variant: {}", unrecognized_value).into())
            }
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
#[diesel(sql_type = crate::schema::sql_types::StackingBehaviour)]
#[serde(rename_all = "lowercase")]
pub enum StackingBehaviour {
    Additive,
    Multiplicative,
    HighestOnly,
}

impl ToSql<crate::schema::sql_types::StackingBehaviour, Pg> for StackingBehaviour {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            StackingBehaviour::Additive => out.write_all(b"additive")?,
            StackingBehaviour::Multiplicative => out.write_all(b"multiplicative")?,
            StackingBehaviour::HighestOnly => out.write_all(b"highest")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::StackingBehaviour, Pg> for StackingBehaviour {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"additive" => Ok(StackingBehaviour::Additive),
            b"multiplicative" => Ok(StackingBehaviour::Multiplicative),
            b"highest" => Ok(StackingBehaviour::HighestOnly),
            _ => {
                let unrecognized_value = String::from_utf8_lossy(bytes.as_bytes());
                Err(format!("Unrecognized enum variant: {}", unrecognized_value).into())
            }
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = modifiers, check_for_backend(diesel::pg::Pg))]
pub struct Modifier {
    pub id: ModifierKey,
    pub name: String,
    pub description: String,
    pub modifier_type: ModifierType,
    pub magnitude: BigDecimal,
    pub target_type: ModifierTarget,
    pub target_resource: Option<ResourceType>,
    pub stacking_behaviour: StackingBehaviour,
    pub stacking_group: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = modifiers, check_for_backend(diesel::pg::Pg))]
pub struct NewModifier {
    pub name: String,
    pub description: String,
    pub modifier_type: ModifierType,
    pub magnitude: BigDecimal,
    pub target_type: ModifierTarget,
    pub target_resource: Option<ResourceType>,
    pub stacking_behaviour: Option<StackingBehaviour>,
    pub stacking_group: Option<String>,
}

#[derive(Identifiable, AsChangeset, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = modifiers, check_for_backend(diesel::pg::Pg))]
pub struct UpdateModifier {
    pub id: ModifierKey,
    pub name: Option<String>,
    pub description: Option<String>,
    pub modifier_type: Option<ModifierType>,
    pub magnitude: Option<BigDecimal>,
    pub target_type: Option<ModifierTarget>,
    pub target_resource: Option<ResourceType>,
    pub stacking_behaviour: Option<StackingBehaviour>,
    pub stacking_group: Option<String>,
}
