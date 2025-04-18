use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::building_levels;

#[derive(Queryable, Selectable, Identifiable, AsChangeset, Debug)]
#[diesel(table_name = building_levels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BuildingLevel {
    pub id: PK,
    pub building_id: i32,
    pub level: i32,
    pub upgrade_time: String,
    pub req_food: Option<i32>,
    pub req_wood: Option<i32>,
    pub req_stone: Option<i32>,
    pub req_gold: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Identifiable, Debug)]
#[diesel(table_name = building_levels, primary_key(building_id), check_for_backend(diesel::pg::Pg))]
pub struct NewBuildingLevel {
    pub building_id: i32,
    pub level: i32,
    pub upgrade_time: String,
    pub req_food: Option<i32>,
    pub req_wood: Option<i32>,
    pub req_stone: Option<i32>,
    pub req_gold: Option<i32>,
}

#[derive(Identifiable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = building_levels, check_for_backend(diesel::pg::Pg))]
pub struct UpdateBuildingLevel {
    pub id: PK,
    pub upgrade_time: Option<String>,
    pub req_food: Option<i32>,
    pub req_wood: Option<i32>,
    pub req_stone: Option<i32>,
    pub req_gold: Option<i32>,
}

pub type PK = Uuid;
