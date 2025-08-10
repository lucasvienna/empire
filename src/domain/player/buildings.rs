use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::building::Building;
use crate::domain::player::{Player, PlayerKey};
use crate::schema::player_building;

pub type PlayerBuildingKey = Uuid;

#[derive(Identifiable, Queryable, Selectable, Associations, Debug)]
#[diesel(belongs_to(Player))]
#[diesel(belongs_to(Building))]
#[diesel(table_name = player_building, check_for_backend(diesel::pg::Pg))]
pub struct PlayerBuilding {
	pub id: PlayerBuildingKey,
	pub player_id: PlayerKey,
	pub building_id: i32,
	pub level: i32,
	pub upgrade_time: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone, PartialEq, Eq, Hash)]
#[diesel(table_name = player_building, check_for_backend(diesel::pg::Pg))]
pub struct NewPlayerBuilding {
	pub player_id: PlayerKey,
	pub building_id: i32,
	pub level: Option<i32>,
	pub upgrade_time: Option<String>,
}

#[derive(Identifiable, AsChangeset, Debug, Clone, PartialEq, Eq, Hash)]
#[diesel(table_name = player_building, check_for_backend(diesel::pg::Pg))]
pub struct UpdatePlayerBuilding {
	pub id: PlayerBuildingKey,
	pub level: Option<i32>,
	pub upgrade_time: Option<String>,
}
