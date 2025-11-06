use serde::{Deserialize, Serialize};

use crate::db::player_buildings::FullBuilding;
use crate::domain::player::PlayerKey;
use crate::domain::player::buildings::PlayerBuildingKey;

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
