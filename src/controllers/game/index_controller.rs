use std::collections::HashMap;

use crate::db::DbConn;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::building::BuildingKey;
use crate::domain::factions::FactionCode;
use crate::domain::player::buildings::PlayerBuildingKey;
use crate::domain::player::session::PlayerSession;
use crate::domain::player::PlayerKey;
use crate::schema::player_building::dsl::player_building;
use crate::Result;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension};
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct GameState {
    player: PlayerState,
    resources: ResourcesState,
    buildings: HashMap<BuildingKey, Vec<BuildingsState>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerState {
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
struct BuildingsState {
    pub id: PlayerBuildingKey,
    pub building_id: BuildingKey,
    pub name: String,
    pub level: i32,
    pub max_level: i32,
    pub max_count: i32,
    pub upgrade_time: String,
    pub req_food: Option<i64>,
    pub req_wood: Option<i64>,
    pub req_stone: Option<i64>,
    pub req_gold: Option<i64>,
    pub population_per_hour: i64,
    pub food_per_hour: i64,
    pub wood_per_hour: i64,
    pub stone_per_hour: i64,
    pub gold_per_hour: i64,
}

#[instrument(skip(pool))]
#[debug_handler(state = AppState)]
pub async fn get_game(
    State(pool): State<AppPool>,
    session: Extension<PlayerSession>,
) -> Result<impl IntoResponse> {
    let player_key = session.player_id;
    let mut conn = pool.get()?;

    let player_state = get_player_data(&mut conn, player_key)?;
    let resources_state = get_resources_data(&mut conn, player_key)?;
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

    Ok(axum::Json(game_state))
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

// TODO: relocate this somewhere proper
pub fn get_resources_data(
    conn: &mut DbConn,
    current_player_id: PlayerKey,
) -> Result<ResourcesState> {
    use crate::schema::player_accumulator::dsl as pa;
    use crate::schema::player_resource::dsl as pr;
    let (pr_data, pa_data) = pr::player_resource
        .inner_join(pa::player_accumulator.on(pr::player_id.eq(pa::player_id)))
        .filter(pr::player_id.eq(current_player_id))
        .select((
            // player_resource fields
            (
                pr::food,
                pr::wood,
                pr::stone,
                pr::gold,
                pr::food_cap,
                pr::wood_cap,
                pr::stone_cap,
                pr::gold_cap,
                pr::produced_at,
                pr::collected_at,
            ),
            // player_accumulator fields
            (pa::food, pa::wood, pa::stone, pa::gold),
        ))
        .first::<(
            (
                i64,
                i64,
                i64,
                i64,
                i64,
                i64,
                i64,
                i64,
                DateTime<Utc>,
                DateTime<Utc>,
            ),
            (i64, i64, i64, i64),
        )>(conn)?;

    let (_, _, food_acc_cap_val, wood_acc_cap_val, stone_acc_cap_val, gold_acc_cap_val) =
        res_gen_view(conn, &current_player_id)?;

    Ok(ResourcesState {
        food: pr_data.0,
        wood: pr_data.1,
        stone: pr_data.2,
        gold: pr_data.3,
        food_cap: pr_data.4,
        wood_cap: pr_data.5,
        stone_cap: pr_data.6,
        gold_cap: pr_data.7,
        produced_at: pr_data.8,
        collected_at: pr_data.9,
        food_acc: pa_data.0,
        wood_acc: pa_data.1,
        stone_acc: pa_data.2,
        gold_acc: pa_data.3,
        food_acc_cap: food_acc_cap_val.to_i64().unwrap_or_default(),
        wood_acc_cap: wood_acc_cap_val.to_i64().unwrap_or_default(),
        stone_acc_cap: stone_acc_cap_val.to_i64().unwrap_or_default(),
        gold_acc_cap: gold_acc_cap_val.to_i64().unwrap_or_default(),
    })
}

type ResourceGenerationView = (
    Uuid,       // player_id
    BigDecimal, // population
    BigDecimal, // food
    BigDecimal, // wood
    BigDecimal, // stone
    BigDecimal, // gold
);

/// Diesel version of the resource_generation view
fn res_gen_view(conn: &mut DbConn, player_key: &PlayerKey) -> Result<ResourceGenerationView> {
    use diesel::dsl::sum;

    use crate::schema::{building_resource as br, player_building as pb};

    let something = pb::table
        .left_join(
            br::table.on(pb::building_id
                .eq(br::building_id)
                .and(pb::level.eq(br::building_level))),
        )
        .group_by(pb::player_id)
        .filter(pb::player_id.eq(player_key))
        .select((
            pb::player_id,
            sum(br::population).assume_not_null(),
            sum(br::food).assume_not_null(),
            sum(br::wood).assume_not_null(),
            sum(br::stone).assume_not_null(),
            sum(br::gold).assume_not_null(),
        ))
        .first::<ResourceGenerationView>(conn)?;

    Ok(something)
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
            bl::upgrade_time,
            bl::req_food,
            bl::req_wood,
            bl::req_stone,
            bl::req_gold,
            // br::population, // Assuming 'population' in building_resource maps to 'population_per_hour'
            br::food,  // Assuming br.food maps to food_per_hour
            br::wood,  // Assuming br.wood maps to wood_per_hour
            br::stone, // Assuming br.stone maps to stone_per_hour
            br::gold,  // Assuming br.gold maps to gold_per_hour
        ))
        .load::<(
            PlayerBuildingKey,
            BuildingKey,
            i32, // from player_building
            String,
            i32,
            i32, // from building
            String,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>, // from building_level
            i64,
            i64,
            i64,
            i64, // from building_resource (production per hour)
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
                upgrade_time: row.6,
                req_food: row.7,
                req_wood: row.8,
                req_stone: row.9,
                req_gold: row.10,
                population_per_hour: 0, // Placeholder, assuming 'population' from br::building_resource if it exists and maps here
                food_per_hour: row.11,
                wood_per_hour: row.12,
                stone_per_hour: row.13,
                gold_per_hour: row.14,
            }
        })
        .collect())
}
