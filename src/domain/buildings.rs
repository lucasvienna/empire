//! Contains domain entities and types related to buildings in the game.
//! Buildings are structures that can be constructed by factions and have various levels and counts.

use diesel::prelude::*;

use crate::domain::factions;
use crate::schema::building;

/// Unique identifier for a building entity
pub type BuildingKey = i32;

/// Represents a building type that can be constructed in the game
#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq, Eq, Hash)]
#[diesel(table_name = building)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Building {
    pub id: BuildingKey,
    pub name: String,
    pub max_level: i32,
    pub max_count: i32,
    pub faction: factions::FactionKey,
    pub starter: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Data transfer object for creating a new building
#[derive(Insertable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewBuilding {
    pub name: String,
    pub max_level: i32,
    pub max_count: i32,
    pub faction: factions::FactionKey,
    pub starter: bool,
}

/// Data transfer object for updating an existing building
#[derive(Identifiable, AsChangeset, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateBuilding { // TODO: make all of these optional
    pub id: BuildingKey,
    pub name: String,
    pub max_level: i32,
    pub max_count: i32,
    pub faction: factions::FactionKey,
    pub starter: bool,
}
