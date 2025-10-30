use std::collections::HashMap;

use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::Int8;
use tracing::{debug, trace, warn};

use crate::Result;
use crate::db::{DbConn, resources};
use crate::domain::modifier::ModifierTarget;
use crate::domain::player::PlayerKey;
use crate::domain::player::accumulator::{AccumulatorKey, PlayerAccumulator};
use crate::domain::player::resource::{PlayerResource, ResourceType};
use crate::domain::resource_generation::ResourceGeneration;
use crate::game::modifiers::modifier_operations;
use crate::game::resources::{
	ResourceMultipliers, ResourceProductionRate, ResourceProductionRates,
};

// AIDEV-NOTE: These SQL functions are not standard in all SQL dialects,
// but are supported by PostgreSQL. `define_sql_function!` makes them
// available to Diesel's query builder. The function is automatically public.
define_sql_function! {
	#[sql_name = "LEAST"]
	fn least(a: Int8, b: Int8) -> Int8
}

/// Collects resources for a player by transferring the maximum possible amount from their
/// resource accumulator to their resource storage, constrained by the storage capacity limits.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_id` - The unique identifier of the player to collect resources for
///
/// # Returns
/// The updated [`PlayerResource`] state after collecting
pub fn collect_resources(conn: &mut DbConn, player_id: &PlayerKey) -> Result<PlayerResource> {
	// This query calculates the exact amount of each resource that can be moved
	// from the accumulator to the main storage without exceeding the storage caps.
	// It uses `LEAST` to take the minimum of what's in the accumulator and the remaining capacity.
	let (collectible_food, collectible_wood, collectible_stone, collectible_gold) = {
		use crate::schema::player_accumulator::dsl as pa;
		use crate::schema::player_resource::dsl as pr;

		pa::player_accumulator
			.inner_join(pr::player_resource.on(pa::player_id.eq(pr::player_id)))
			.filter(pa::player_id.nullable().eq(player_id))
			.select((
				least(pa::food, pr::food_cap - pr::food),
				least(pa::wood, pr::wood_cap - pr::wood),
				least(pa::stone, pr::stone_cap - pr::stone),
				least(pa::gold, pr::gold_cap - pr::gold),
			))
			.first::<(i64, i64, i64, i64)>(conn)?
	};

	debug!(
		"Collectible amounts: Food: {}, Wood: {}, Stone: {}, Gold: {}",
		collectible_food, collectible_wood, collectible_stone, collectible_gold
	);

	// The resource transfer is performed in a transaction to ensure atomicity.
	// First, we drain the calculated amounts from the accumulator.
	// Second, we add those same amounts to the main resource storage.
	conn.transaction(|conn| {
		use crate::schema::player_accumulator::dsl as pa;
		use crate::schema::player_resource::dsl as pr;

		// Drain the accumulator
		diesel::update(pa::player_accumulator.filter(pa::player_id.eq(player_id)))
			.set((
				pa::food.eq(pa::food - collectible_food),
				pa::wood.eq(pa::wood - collectible_wood),
				pa::stone.eq(pa::stone - collectible_stone),
				pa::gold.eq(pa::gold - collectible_gold),
			))
			.execute(conn)?;

		// Then, increase the main resource storage
		diesel::update(pr::player_resource.filter(pr::player_id.eq(player_id)))
			.set((
				pr::food.eq(pr::food + collectible_food),
				pr::wood.eq(pr::wood + collectible_wood),
				pr::stone.eq(pr::stone + collectible_stone),
				pr::gold.eq(pr::gold + collectible_gold),
			))
			.returning(PlayerResource::as_returning())
			.get_result(conn)
			.map_err(Into::into)
	})
}

/// Produces resources for a player based on their production rates and time elapsed since last production.
///
/// This function calculates the amount of resources to produce, applies production rates,
/// and updates the player's accumulator with the produced resources, respecting storage caps.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_id` - The unique identifier of the player to produce resources for
/// * `production_rates` - HashMap of production rates per hour for each resource type
/// * `up_to_time` - Optional timestamp to produce up to (defaults to now)
///
/// # Returns
/// The updated [`PlayerAccumulator`] state after production
pub fn produce_resources(
	conn: &mut DbConn,
	player_id: &PlayerKey,
	production_rates: &ResourceProductionRates,
	up_to_time: Option<DateTime<Utc>>,
) -> Result<PlayerAccumulator> {
	let target_time = up_to_time.unwrap_or_else(Utc::now);

	// Get the last production time
	let last_prod = resources::get_by_player_id(conn, player_id)?.produced_at;
	let delta = target_time - last_prod;
	let delta_hours = BigDecimal::from(delta.num_seconds()) / BigDecimal::from(3600);

	debug!(
		"Production Delta: {:.4}h, last produced at: {} for player: {}",
		delta_hours, last_prod, player_id
	);
	debug!("Production Rates: {:?}", production_rates);

	// Calculate production amounts
	let prod_amounts: HashMap<ResourceType, i64> = production_rates
		.iter()
		.map(|(res_type, prod_rate)| {
			let amount = prod_rate * &delta_hours;
			let truncated = amount.to_i64().unwrap_or_default();
			(*res_type, truncated)
		})
		.collect();

	debug!(
		"Producing resources for player {}: {:?}",
		player_id, prod_amounts
	);

	// Add generated resources to the player's accumulator, respecting storage caps
	conn.transaction(|conn| -> Result<PlayerAccumulator> {
		use crate::custom_schema::resource_generation::dsl as rg;
		use crate::schema::player_accumulator::dsl as pa;
		use crate::schema::player_resource::dsl as pr;

		trace!("Entering accumulator update transaction");

		// Lock the player's accumulator row to prevent race conditions during the update
		let acc_key: AccumulatorKey = pa::player_accumulator
			.select(pa::id)
			.filter(pa::player_id.eq(player_id))
			.for_update()
			.first(conn)?;
		trace!("Found player accumulator: {:?}", acc_key);

		let acc_caps: ResourceGeneration = rg::resource_generation.find(player_id).first(conn)?;

		let get_prod = |res_type: ResourceType| *prod_amounts.get(&res_type).unwrap_or(&0);
		let res = diesel::update(pa::player_accumulator)
			.filter(pa::id.eq(&acc_key))
			.set((
				pa::food.eq(least(
					pa::food + get_prod(ResourceType::Food),
					acc_caps.food_acc_cap,
				)),
				pa::wood.eq(least(
					pa::wood + get_prod(ResourceType::Wood),
					acc_caps.wood_acc_cap,
				)),
				pa::stone.eq(least(
					pa::stone + get_prod(ResourceType::Stone),
					acc_caps.stone_acc_cap,
				)),
				pa::gold.eq(least(
					pa::gold + get_prod(ResourceType::Gold),
					acc_caps.gold_acc_cap,
				)),
			))
			.returning(PlayerAccumulator::as_returning())
			.get_result(conn)?;
		debug!("New accumulator state: {:?}", res);

		let updated_rows = diesel::update(pr::player_resource.filter(pr::player_id.eq(player_id)))
			.set(pr::produced_at.eq(target_time))
			.execute(conn)?;

		if updated_rows != 1 {
			warn!(
				"Expected to update 1 `produced_at` timestamp, but updated {}. Player ID: {}",
				updated_rows, player_id
			);
		}

		Ok(res)
	})
}

/// Produces resources up to the current time and then collects them in a single operation.
///
/// This ensures that when a player clicks "collect", they receive resources produced
/// up to that exact moment, preventing stale values.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_id` - The unique identifier of the player
/// * `production_rates` - HashMap of production rates per hour for each resource type
///
/// # Returns
/// A tuple of (PlayerAccumulator after production, PlayerResource after collection)
pub fn produce_and_collect_resources(
	conn: &mut DbConn,
	player_id: &PlayerKey,
	production_rates: &ResourceProductionRates,
) -> Result<(PlayerAccumulator, PlayerResource)> {
	// First produce resources up to now
	let accumulator = produce_resources(conn, player_id, production_rates, None)?;

	// Then collect the produced resources
	let resources = collect_resources(conn, player_id)?;

	Ok((accumulator, resources))
}

/// Get base resource generation rates for a player from the database
pub fn get_base_rates(conn: &mut DbConn, player_key: &PlayerKey) -> Result<ResourceGeneration> {
	use crate::custom_schema::resource_generation::dsl::{player_id, resource_generation};

	resource_generation
		.select(ResourceGeneration::as_select())
		.filter(player_id.eq(player_key))
		.first(conn)
		.map_err(Into::into)
}

/// Calculate production rates by applying pre-calculated modifiers to base rates
pub fn apply_rate_modifiers(
	base_rates: &ResourceGeneration,
	modifiers: &ResourceMultipliers,
) -> ResourceProductionRates {
	modifiers
		.iter()
		.map(|(res_type, multiplier)| {
			let base_rate = match res_type {
				ResourceType::Population => base_rates.population,
				ResourceType::Food => base_rates.food,
				ResourceType::Wood => base_rates.wood,
				ResourceType::Stone => base_rates.stone,
				ResourceType::Gold => base_rates.gold,
			};

			let final_rate = ResourceProductionRate::from(base_rate) * multiplier;
			(*res_type, final_rate)
		})
		.collect()
}

/// Calculate production rates with modifiers applied
///
/// This is a pure function that calculates rates without caching.
pub fn calc_prod_rates(
	conn: &mut DbConn,
	player_id: &PlayerKey,
) -> Result<ResourceProductionRates> {
	use strum::IntoEnumIterator;

	// Get base rates from the database
	let base_rates = get_base_rates(conn, player_id)?;

	// Calculate modifiers for each resource type
	let production_rates: ResourceProductionRates = ResourceType::iter()
		.map(|res_type| {
			let multiplier = modifier_operations::calc_multiplier(
				conn,
				player_id,
				ModifierTarget::Resource,
				Some(res_type),
			)
			.unwrap_or(BigDecimal::from(1));

			let base_rate = match res_type {
				ResourceType::Population => base_rates.population,
				ResourceType::Food => base_rates.food,
				ResourceType::Wood => base_rates.wood,
				ResourceType::Stone => base_rates.stone,
				ResourceType::Gold => base_rates.gold,
			};

			let final_rate = ResourceProductionRate::from(base_rate) * multiplier;
			(res_type, final_rate)
		})
		.collect();

	Ok(production_rates)
}
