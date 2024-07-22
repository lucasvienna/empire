use diesel::prelude::*;

use crate::schema::user_buildings;

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = user_buildings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserBuilding {
    pub id: PK,
    pub user_id: i32,
    pub building_id: i32,
    pub level: i32,
    pub upgrade_time: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = user_buildings)]
pub struct NewUserBuilding {
    pub user_id: i32,
    pub building_id: i32,
    pub level: Option<i32>,
}

pub type PK = i32;
