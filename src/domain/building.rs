use diesel::prelude::*;

use crate::domain::faction;
use crate::schema::buildings;

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq, Eq, Hash)]
#[diesel(table_name = buildings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Building {
    pub id: PK,
    pub name: String,
    pub max_level: i32,
    pub max_count: i32,
    pub faction: faction::PK,
    pub starter: bool,
}

#[derive(Insertable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = buildings)]
pub struct NewBuilding {
    pub name: String,
    pub max_level: i32,
    pub max_count: i32,
    pub faction: faction::PK,
    pub starter: bool,
}

#[derive(AsChangeset, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = buildings)]
pub struct UpdateBuilding {
    pub name: String,
    pub max_level: i32,
    pub max_count: i32,
    pub faction: faction::PK,
    pub starter: bool,
}

pub type PK = i32;
