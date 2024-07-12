use diesel::prelude::*;

use crate::schema::factions;

#[derive(Queryable, Selectable)]
#[diesel(table_name = factions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Faction {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = factions)]
pub struct NewFaction<'a> {
    pub id: &'a str,
    pub name: &'a str,
}
