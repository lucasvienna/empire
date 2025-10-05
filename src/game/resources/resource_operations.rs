use diesel::prelude::*;
use diesel::sql_types::Int8;
use tracing::debug;

use crate::db::DbConn;
use crate::domain::player::resource::PlayerResource;
use crate::domain::player::PlayerKey;
use crate::Result;

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
/// The updated `PlayerResource` state after collecting
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
