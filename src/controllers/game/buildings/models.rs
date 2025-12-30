use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::player_buildings::FullBuilding;
use crate::domain::factions::FactionKey;
use crate::domain::player::PlayerKey;
use crate::domain::player::buildings::PlayerBuildingKey;
use crate::domain::unit::UnitType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameBuilding {
	pub id: PlayerBuildingKey,
	pub player_id: PlayerKey,
	pub building_id: i32,
	pub level: i32,
	pub max_level: i32,
	pub max_count: i32,
	pub upgrade_seconds: i64,
	pub req_food: Option<i64>,
	pub req_wood: Option<i64>,
	pub req_stone: Option<i64>,
	pub req_gold: Option<i64>,
}

impl From<FullBuilding> for GameBuilding {
	fn from(value: FullBuilding) -> Self {
		let (pb, bld, bl, br) = value;
		GameBuilding {
			id: pb.id,
			player_id: pb.player_id,
			building_id: bld.id,
			level: pb.level,
			max_level: bld.max_level,
			max_count: bld.max_count,
			upgrade_seconds: bl.upgrade_seconds,
			req_food: bl.req_food,
			req_wood: bl.req_wood,
			req_stone: bl.req_stone,
			req_gold: bl.req_gold,
		}
	}
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstructBuildingRequest {
	pub building_id: i32,
}

/// Full building definition with all levels, used by `/game/buildings/all`
#[derive(Serialize, Debug, Clone)]
pub struct BuildingDefinition {
	pub id: i32,
	pub name: String,
	pub max_level: i32,
	pub max_count: i32,
	pub faction: FactionKey,
	pub starter: bool,
	pub unit_types: Vec<UnitType>,
	pub levels: Vec<BuildingLevelInfo>,
}

/// Level-specific information including costs, production, capacity, and requirements
#[derive(Serialize, Debug, Clone)]
pub struct BuildingLevelInfo {
	pub level: i32,
	pub upgrade_seconds: i64,
	pub training_capacity: Option<i32>,
	pub costs: ResourceCosts,
	pub production: ResourceProduction,
	pub capacity: ResourceCapacity,
	pub requirements: Vec<LevelRequirement>,
}

/// Resource costs required for construction or upgrade
#[derive(Serialize, Debug, Clone, Default)]
pub struct ResourceCosts {
	pub food: i64,
	pub wood: i64,
	pub stone: i64,
	pub gold: i64,
}

/// Resource production rates per hour
#[derive(Serialize, Debug, Clone, Default)]
pub struct ResourceProduction {
	pub population: i64,
	pub food: i64,
	pub wood: i64,
	pub stone: i64,
	pub gold: i64,
}

/// Resource storage and accumulator capacities
#[derive(Serialize, Debug, Clone, Default)]
pub struct ResourceCapacity {
	pub food: i64,
	pub wood: i64,
	pub stone: i64,
	pub gold: i64,
	pub food_acc: i64,
	pub wood_acc: i64,
	pub stone_acc: i64,
	pub gold_acc: i64,
}

/// Prerequisite for upgrading to a specific building level
#[derive(Serialize, Debug, Clone)]
pub struct LevelRequirement {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_building_id: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_building_level: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_tech_id: Option<Uuid>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_tech_level: Option<i32>,
}
