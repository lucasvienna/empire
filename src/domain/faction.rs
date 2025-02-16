use diesel::prelude::*;

use crate::schema::factions;

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = factions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Faction {
    pub id: PK,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = factions)]
pub struct NewFaction<'a> {
    pub id: PK,
    pub name: &'a str,
}

pub type PK = i32;
