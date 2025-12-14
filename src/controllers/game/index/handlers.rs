use std::collections::HashMap;

use axum::response::IntoResponse;
use axum::{Extension, Json, debug_handler};
use chrono::{DateTime, Utc};
use diesel::QueryResult;
use diesel::prelude::*;
use tracing::instrument;

use super::models::{BuildingsState, GameState, PlayerState, ResourcesState};
use crate::Result;
use crate::db::DbConn;
use crate::db::extractor::DatabaseConnection;
use crate::domain::app_state::AppState;
use crate::domain::auth::AuthenticatedUser;
use crate::domain::building::BuildingKey;
use crate::domain::factions::FactionCode;
use crate::domain::player::PlayerKey;
use crate::domain::player::buildings::PlayerBuildingKey;
use crate::game::resources::resource_operations;
use crate::schema::player_building::dsl::player_building;

#[instrument(skip(conn), fields(player_id = %player.id))]
#[debug_handler(state = AppState)]
pub(super) async fn get_game(
	DatabaseConnection(mut conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	let player_key = player.id;

	let player_state = get_player_data(&mut conn, player_key)?;
	let resource_snapshot = resource_operations::get_resource_snapshot(&mut conn, &player_key)?;
	let resources_state = ResourcesState::from(resource_snapshot);
	let buildings_list = get_player_buildings_data(&mut conn, player_key)?;

	// The GameState.buildings is a Map<BuildingKey, Vec<BuildingsState>>
	// You'll need to group the buildings_list by building_id (BuildingKey)
	let mut buildings_map: HashMap<BuildingKey, Vec<BuildingsState>> = HashMap::new();
	for building_state in buildings_list {
		buildings_map
			.entry(building_state.building_id)
			.or_default()
			.push(building_state);
	}

	let game_state = GameState {
		player: player_state,
		resources: resources_state,
		buildings: buildings_map,
	};

	Ok(Json(game_state))
}

fn get_player_data(conn: &mut DbConn, current_player_id: PlayerKey) -> QueryResult<PlayerState> {
	use crate::schema::player::dsl::*;

	#[derive(Queryable, Debug)]
	struct PlayerData {
		id: PlayerKey,
		name: String,
		faction: FactionCode,
	}

	player
		.filter(id.eq(current_player_id))
		.select((id, name, faction)) // Ensure these fields match PlayerState or PlayerData
		.first::<PlayerData>(conn)
		.map(|pd| PlayerState {
			id: pd.id,
			name: pd.name,
			faction: pd.faction,
		})
}

fn get_player_buildings_data(
	conn: &mut PgConnection,
	current_player_id: PlayerKey,
) -> QueryResult<Vec<BuildingsState>> {
	use crate::schema::building::dsl as b;
	use crate::schema::building_level::dsl as bl;
	use crate::schema::building_resource::dsl as br;
	use crate::schema::player_building::dsl as pb;

	let results = player_building
		.filter(pb::player_id.eq(current_player_id))
		.inner_join(b::building.on(pb::building_id.eq(b::id)))
		.inner_join(
			bl::building_level.on(pb::building_id
				.eq(bl::building_id)
				.and(bl::level.eq(pb::level + 1))),
		)
		.inner_join(
			br::building_resource.on(pb::building_id
				.eq(br::building_id)
				.and(pb::level.eq(br::building_level))),
		)
		.select((
			pb::id,
			pb::building_id,
			pb::level,
			b::name,
			b::max_level,
			b::max_count,
			bl::upgrade_seconds,
			pb::upgrade_finishes_at,
			bl::req_food,
			bl::req_wood,
			bl::req_stone,
			bl::req_gold,
			// br::population, // Assuming 'population' in building_resource maps to 'population_per_hour'
			br::food,  // Assuming br.food maps to food_per_hour
			br::wood,  // Assuming br.wood maps to wood_per_hour
			br::stone, // Assuming br.stone maps to stone_per_hour
			br::gold,  // Assuming br.gold maps to gold_per_hour
			pb::updated_at,
		))
		.load::<(
			PlayerBuildingKey,
			BuildingKey,
			i32, // from player_building
			String,
			i32,
			i32, // from building
			i64, // in seconds
			Option<String>,
			Option<i64>,
			Option<i64>,
			Option<i64>,
			Option<i64>, // from building_level
			i64,
			i64,
			i64,
			i64, // from building_resource (production per hour)
			DateTime<Utc>,
		)>(conn)?;

	Ok(results
		.into_iter()
		.map(|row| {
			BuildingsState {
				id: row.0,
				building_id: row.1,
				level: row.2,
				name: row.3,
				max_level: row.4,
				max_count: row.5,
				upgrade_seconds: row.6,
				upgrade_finishes_at: row.7,
				req_food: row.8,
				req_wood: row.9,
				req_stone: row.10,
				req_gold: row.11,
				population_per_hour: 0, // Placeholder, assuming 'population' from br::building_resource if it exists and maps here
				food_per_hour: row.12,
				wood_per_hour: row.13,
				stone_per_hour: row.14,
				gold_per_hour: row.15,
				updated_at: row.16,
			}
		})
		.collect())
}
