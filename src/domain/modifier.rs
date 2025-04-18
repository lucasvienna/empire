use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::io::Write;

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
