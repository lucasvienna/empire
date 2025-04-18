use std::io::Write;

use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};

use crate::domain::user::{self, User};
use crate::schema::user_resources;

pub type PK = user::PK;

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
#[diesel(sql_type = crate::schema::sql_types::ResourceType)]
#[serde(rename_all = "lowercase")]
pub enum ResourceType {
    Population,
    Food,
    Wood,
    Stone,
    Gold,
}

impl ToSql<crate::schema::sql_types::ResourceType, Pg> for ResourceType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ResourceType::Population => out.write_all(b"population")?,
            ResourceType::Food => out.write_all(b"food")?,
            ResourceType::Wood => out.write_all(b"wood")?,
            ResourceType::Stone => out.write_all(b"stone")?,
            ResourceType::Gold => out.write_all(b"gold")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::ResourceType, Pg> for ResourceType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"population" => Ok(ResourceType::Population),
            b"food" => Ok(ResourceType::Food),
            b"wood" => Ok(ResourceType::Wood),
            b"stone" => Ok(ResourceType::Stone),
            b"gold" => Ok(ResourceType::Gold),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    AsChangeset,
    Associations,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[diesel(table_name = user_resources, primary_key(user_id))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserResource {
    pub user_id: user::PK,
    pub food: i32,
    pub wood: i32,
    pub stone: i32,
    pub gold: i32,
    pub food_cap: i32,
    pub wood_cap: i32,
    pub stone_cap: i32,
    pub gold_cap: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = user_resources)]
pub struct NewResource {
    pub user_id: user::PK,
    pub food: Option<i32>,
    pub wood: Option<i32>,
    pub stone: Option<i32>,
    pub gold: Option<i32>,
}

#[derive(AsChangeset, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = user_resources)]
pub struct UpdateResource {
    pub food: Option<i32>,
    pub wood: Option<i32>,
    pub stone: Option<i32>,
    pub gold: Option<i32>,
}
