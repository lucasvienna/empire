use diesel::prelude::*;

use crate::schema::building_levels;

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = building_levels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BuildingLevel {
    pub id: PK,
    pub building_id: i32,
    pub level: i32,
    pub upgrade_time: String,
    pub req_food: Option<i32>,
    pub req_wood: Option<i32>,
    pub req_stone: Option<i32>,
    pub req_gold: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = building_levels)]
pub struct NewBuildingLevel<'a> {
    pub building_id: i32,
    pub level: i32,
    pub upgrade_time: &'a str,
    pub req_food: Option<i32>,
    pub req_wood: Option<i32>,
    pub req_stone: Option<i32>,
    pub req_gold: Option<i32>,
}

pub type PK = i32;
