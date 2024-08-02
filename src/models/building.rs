use diesel::prelude::*;

use crate::schema::buildings;

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = buildings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Building {
    pub id: PK,
    pub name: String,
    pub max_level: i32,
    pub max_count: i32,
    pub faction: i32,
    pub starter: bool,
}

#[derive(Insertable)]
#[diesel(table_name = buildings)]
pub struct NewBuilding<'a> {
    pub name: &'a str,
    pub max_level: i32,
    pub max_count: i32,
    pub faction: i32,
    pub starter: bool,
}

pub type PK = i32;
