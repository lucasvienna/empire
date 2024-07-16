use diesel::prelude::*;

use crate::schema::factions;

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = factions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Faction {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = factions)]
pub struct NewFaction<'a> {
    pub id: i32,
    pub name: &'a str,
}
