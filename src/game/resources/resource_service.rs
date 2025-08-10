use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::FromRef;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use diesel::sql_types::Int8;
use strum::IntoEnumIterator;
use tracing::{debug, instrument, trace, warn};

use crate::db::resources::ResourcesRepository;
use crate::db::Repository;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::modifier::ModifierTarget;
use crate::domain::player::accumulator::{AccumulatorKey, PlayerAccumulator};
use crate::domain::player::resource::{PlayerResource, ResourceType};
use crate::domain::player::PlayerKey;
use crate::domain::resource_generation::ResourceGeneration;
use crate::game::modifiers::modifier_service::ModifierService;
use crate::game::modifiers::modifier_system::ModifierSystem;
use crate::game::resources::resource_scheduler::ProductionScheduler;
use crate::job_queue::JobQueue;
use crate::Result;

// AIDEV-NOTE: These SQL functions are not standard in all SQL dialects,
// but are supported by PostgreSQL. `define_sql_function!` makes them
// available to Diesel's query builder.
define_sql_function! {
	#[sql_name = "LEAST"]
	fn least(a: Int8, b: Int8) -> Int8
}
define_sql_function! {
	#[sql_name = "GREATEST"]
	fn greatest(a: Int8, b: Int8) -> Int8
}

/// Service responsible for managing resources for players.
/// Handles calculation and application of resources rates, considering modifiers
/// and time-based accumulation of resources.
pub struct ResourceService {
	db_pool: AppPool,
	modifiers: ModifierSystem,
	modifier_service: ModifierService,
	resource_scheduler: ProductionScheduler,
	resources_repo: ResourcesRepository,
}

impl FromRef<AppState> for ResourceService {
	fn from_ref(state: &AppState) -> Self {
		Self::new(&state.db_pool, &state.job_queue, &state.modifier_system)
	}
}

impl ResourceService {
	pub fn new(pool: &AppPool, queue: &Arc<JobQueue>, mod_system: &ModifierSystem) -> Self {
		Self {
			db_pool: Arc::clone(pool),
			// Cloning `ModifierSystem` is inexpensive as it's composed of `Arc`s.
			modifiers: mod_system.clone(),
			modifier_service: ModifierService::new(pool, mod_system),
			resource_scheduler: ProductionScheduler::new(queue),
			resources_repo: ResourcesRepository::new(pool),
		}
	}

	/// Produces resources for a player based on their resource rates and time elapsed since last resources.
	/// Calculates the amount of resources to produce, applies modifiers, and updates the player's accumulator.
	///
	/// # Arguments
	/// * `player_key` - The unique identifier of the player to produce resources for
	#[instrument(skip(self))]
	pub async fn produce_for_player(&self, player_key: &PlayerKey) -> Result<()> {
		// We calculate the delta in hours as a `BigDecimal` for precision in production calculations.
		let last_prod = self.last_player_prod(player_key).unwrap_or_default();
		let delta = Utc::now() - last_prod;
		let delta_hours = BigDecimal::from(delta.num_seconds()) / BigDecimal::from(3600);

		let current_rates = self.cur_prod_rates(player_key).await?;
		debug!(
			"Production Delta: {:.4}h, last produced at: {}",
			delta_hours, last_prod
		);
		debug!("Current Rates: {:?}", current_rates);

		// Refactored: The logic to calculate production amounts is now more explicit.
		// The `.to_i64()` conversion can return `None` if the value is too large,
		// so `unwrap_or_default()` prevents a panic, defaulting to 0. This is a failsafe,
		// though in practice resource amounts should not overflow an i64.
		let prod_amounts: HashMap<ResourceType, i64> = current_rates
			.into_iter()
			.map(|(res_type, prod_rate)| {
				let amount = prod_rate * &delta_hours;
				let truncated = amount.to_i64().unwrap_or_default();
				(res_type, truncated)
			})
			.collect();
		debug!(
			"Producing resources for player {}: {:?}",
			player_key, prod_amounts
		);

		// Add generated resources to the player's accumulator, respecting storage caps.
		let mut conn = self.db_pool.get()?;
		conn.transaction(|conn| -> Result<PlayerAccumulator> {
			use crate::custom_schema::resource_generation::dsl as rg;
			use crate::schema::player_accumulator::dsl as pa;
			use crate::schema::player_resource::dsl as pr;

			trace!("Entering accumulator update transaction");

			// Lock the player's accumulator row to prevent race conditions during the update.
			let acc_key: AccumulatorKey = pa::player_accumulator
				.select(pa::id)
				.filter(pa::player_id.eq(player_key))
				.for_update()
				.first(conn)?;
			trace!("Found player accumulator: {:?}", acc_key);

			let acc_caps: ResourceGeneration =
				rg::resource_generation.find(player_key).first(conn)?;

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

			let updated_rows =
				diesel::update(pr::player_resource.filter(pr::player_id.eq(player_key)))
					.set(pr::produced_at.eq(Utc::now()))
					.execute(conn)?;

			if updated_rows != 1 {
				warn!(
					"Expected to update 1 `produced_at` timestamp, but updated {}. Player ID: {}",
					updated_rows, player_key
				);
			}

			Ok(res)
		})?;

		// Enqueue the next production job.
		// The 2-minute interval is a simple polling mechanism.
		let next_production_time = Utc::now() + Duration::minutes(2);
		self.resource_scheduler
			.schedule_production(player_key, next_production_time)
			.map_err(|err| {
				warn!("Failed to schedule next production: {:#?}", err);
				err
			})?;

		Ok(())
	}

	/// Collects resources for a player by transferring the maximum possible amount from their
	/// resource accumulator to their resource storage, constrained by the storage capacity limits.
	#[instrument(skip(self))]
	pub fn collect_resources(&self, player_id: &PlayerKey) -> Result<PlayerResource> {
		let mut connection = self.db_pool.get()?;

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
				.first::<(i64, i64, i64, i64)>(&mut connection)?
		};

		debug!(
			"Collectible amounts: Food: {}, Wood: {}, Stone: {}, Gold: {}",
			collectible_food, collectible_wood, collectible_stone, collectible_gold
		);

		// The resource transfer is performed in a transaction to ensure atomicity.
		// First, we drain the calculated amounts from the accumulator.
		// Second, we add those same amounts to the main resource storage.
		connection.transaction(|conn| {
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

	/// Retrieves current resources rates for all resource types for a given player,
	/// applying all active modifiers to base resources rates.
	#[instrument(skip_all)]
	async fn cur_prod_rates(
		&self,
		player_key: &PlayerKey,
	) -> Result<HashMap<ResourceType, BigDecimal>> {
		// Fetch all relevant production modifiers for the player.
		// This is done sequentially; for higher performance, this could be parallelized
		// using `futures::future::try_join_all` if `get_total_multiplier` is I/O bound.
		let mut mods = HashMap::new();
		for res_type in ResourceType::iter() {
			let multiplier = self
				.modifier_service
				.get_total_multiplier(player_key, ModifierTarget::Resource, Some(res_type))
				.await?;
			mods.insert(res_type, multiplier);
		}
		debug!(?mods, "Calculated modifiers");

		// Fetch the player's base resource generation rates from the database.
		let base_rates = {
			use crate::custom_schema::resource_generation::dsl::{player_id, resource_generation};
			let mut conn = self.db_pool.get()?;
			resource_generation
				.select(ResourceGeneration::as_select())
				.filter(player_id.eq(player_key))
				.first(&mut conn)?
		};

		let current_hourly_rates = mods
			.into_iter()
			.map(|(res_type, modifier)| {
				let base_rate = match res_type {
					ResourceType::Population => base_rates.population,
					ResourceType::Food => base_rates.food,
					ResourceType::Wood => base_rates.wood,
					ResourceType::Stone => base_rates.stone,
					ResourceType::Gold => base_rates.gold,
				};
				let final_rate = BigDecimal::from(base_rate) * modifier;
				(res_type, final_rate)
			})
			.collect();

		Ok(current_hourly_rates)
	}

	/// Gets the timestamp of the last resource production for a player.
	fn last_player_prod(&self, player_key: &PlayerKey) -> Result<DateTime<Utc>> {
		let player_resource = self.resources_repo.get_by_player_id(player_key)?;
		Ok(player_resource.produced_at)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_big_int() {
		let bigint = BigDecimal::try_from(10.0050).unwrap();
		let rounded = bigint.round(0);
		assert_eq!(rounded, BigDecimal::from(10));

		let as_i64 = rounded.to_i64().unwrap();
		assert_eq!(as_i64, 10);
	}
}
