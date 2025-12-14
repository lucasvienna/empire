use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::building::BuildingKey;
use crate::domain::factions::FactionCode;
use crate::domain::player::PlayerKey;
use crate::domain::player::buildings::PlayerBuildingKey;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
	pub player: PlayerState,
	pub resources: ResourcesState,
	pub buildings: HashMap<BuildingKey, Vec<BuildingsState>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerState {
	pub id: PlayerKey,
	pub name: String,
	pub faction: FactionCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcesState {
	pub food: i64,
	pub wood: i64,
	pub stone: i64,
	pub gold: i64,
	pub food_cap: i64,
	pub wood_cap: i64,
	pub stone_cap: i64,
	pub gold_cap: i64,
	pub food_rate: i64,
	pub wood_rate: i64,
	pub stone_rate: i64,
	pub gold_rate: i64,
	pub food_acc: i64,
	pub wood_acc: i64,
	pub stone_acc: i64,
	pub gold_acc: i64,
	pub food_acc_cap: i64,
	pub wood_acc_cap: i64,
	pub stone_acc_cap: i64,
	pub gold_acc_cap: i64,
	pub produced_at: DateTime<Utc>,
	pub collected_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildingsState {
	pub id: PlayerBuildingKey,
	pub building_id: BuildingKey,
	pub name: String,
	pub level: i32,
	pub max_level: i32,
	pub max_count: i32,
	pub upgrade_seconds: i64,
	pub upgrade_finishes_at: Option<String>,
	pub req_food: Option<i64>,
	pub req_wood: Option<i64>,
	pub req_stone: Option<i64>,
	pub req_gold: Option<i64>,
	pub population_per_hour: i64,
	pub food_per_hour: i64,
	pub wood_per_hour: i64,
	pub stone_per_hour: i64,
	pub gold_per_hour: i64,
	pub updated_at: DateTime<Utc>,
}
