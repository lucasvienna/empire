use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::building_level;

/// Unique identifier type for building levels
pub type BuildingLevelKey = Uuid;

/// Represents a building level in the game with its requirements and upgrade details
#[derive(
    Queryable, Selectable, Identifiable, AsChangeset, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
#[diesel(table_name = building_level)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BuildingLevel {
    pub id: BuildingLevelKey,
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

/// Data required to create a new building level
#[derive(Insertable, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building_level, check_for_backend(diesel::pg::Pg))]
pub struct NewBuildingLevel {
    pub building_id: i32,
    pub level: i32,
    pub upgrade_time: String,
    pub req_food: Option<i32>,
    pub req_wood: Option<i32>,
    pub req_stone: Option<i32>,
    pub req_gold: Option<i32>,
}

/// Data structure for updating an existing building level
#[derive(Identifiable, AsChangeset, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = building_level, check_for_backend(diesel::pg::Pg))]
pub struct UpdateBuildingLevel {
    pub id: BuildingLevelKey,
    pub upgrade_time: Option<String>,
    pub req_food: Option<i32>,
    pub req_wood: Option<i32>,
    pub req_stone: Option<i32>,
    pub req_gold: Option<i32>,
}
