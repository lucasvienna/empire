use std::io::Write;

use chrono::{DateTime, Utc};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};

use crate::domain::player::{Player, PlayerKey};
use crate::schema::player_resource;

pub type PK = PlayerKey;

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
            _ => {
                let unrecognized_value = String::from_utf8_lossy(bytes.as_bytes());
                Err(format!("Unrecognized enum variant: {}", unrecognized_value).into())
            }
        }
    }
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
#[diesel(table_name = player_resource, primary_key(player_id))]
#[diesel(belongs_to(Player))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PlayerResource {
    pub player_id: PlayerKey,
    pub food: i32,
    pub wood: i32,
    pub stone: i32,
    pub gold: i32,
    pub food_cap: i32,
    pub wood_cap: i32,
    pub stone_cap: i32,
    pub gold_cap: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = player_resource, check_for_backend(diesel::pg::Pg))]
pub struct NewPlayerResource {
    pub player_id: PlayerKey,
    pub food: Option<i32>,
    pub wood: Option<i32>,
    pub stone: Option<i32>,
    pub gold: Option<i32>,
}

#[derive(Identifiable, AsChangeset, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = player_resource, primary_key(player_id))]
pub struct UpdatePlayerResource {
    pub player_id: PlayerKey,
    pub food: Option<i32>,
    pub wood: Option<i32>,
    pub stone: Option<i32>,
    pub gold: Option<i32>,
}
