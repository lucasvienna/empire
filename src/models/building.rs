use diesel::prelude::*;

use crate::schema::buildings;

#[derive(Queryable, Selectable)]
#[diesel(table_name = buildings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Building {
    pub id: i32,
    pub name: String,
    pub max_level: i32,
    pub faction: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = buildings)]
pub struct NewBuilding<'a> {
    pub name: &'a str,
    pub max_level: i32,
    pub faction: Option<&'a str>,
}
