use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::FromRef;
use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::sql_types::Int8;
use strum::IntoEnumIterator;
use tracing::{debug, instrument, warn};

use crate::domain::app_state::{AppPool, AppState};
use crate::domain::modifier::ModifierTarget;
use crate::domain::player::resource::{PlayerResource, ResourceType};
use crate::domain::player::PlayerKey;
use crate::game::modifiers::modifier_service::ModifierService;
use crate::game::modifiers::modifier_system::ModifierSystem;
use crate::game::resources::resource_operations;
use crate::game::resources::resource_scheduler::ProductionScheduler;
use crate::job_queue::JobQueue;
use crate::Result;

// AIDEV-NOTE: This SQL function is not standard in all SQL dialects,
// but is supported by PostgreSQL. `define_sql_function!` makes it
// available to Diesel's query builder.
define_sql_function! {
	#[sql_name = "GREATEST"]
	fn greatest(a: Int8, b: Int8) -> Int8
}

/// Service responsible for managing resources for players.
/// Handles calculation and application of resource rates, considering modifiers
/// and time-based accumulation of resources.
pub struct ResourceService {
	db_pool: AppPool,
	modifier_service: ModifierService,
	resource_scheduler: ProductionScheduler,
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
			modifier_service: ModifierService::new(pool, mod_system),
			resource_scheduler: ProductionScheduler::new(queue),
		}
	}

	/// Produces resources for a player based on their resource rates and time elapsed since last resources.
	/// Calculates the amount of resources to produce, applies modifiers, and updates the player's accumulator.
	///
	/// This method delegates to the resource_operations module for the core production logic,
	/// while maintaining responsibility for calculating production rates (with modifiers and caching)
	/// and scheduling the next production job.
	///
	/// # Arguments
	/// * `player_key` - The unique identifier of the player to produce resources for
	#[instrument(skip(self))]
	pub async fn produce_for_player(&self, player_key: &PlayerKey) -> Result<()> {
		// Calculate current production rates with modifiers applied (uses caching)
		let current_rates = self.get_production_rates(player_key).await?;

		// Delegate to operations module for production logic
		let mut conn = self.db_pool.get()?;
		resource_operations::produce_resources(&mut conn, player_key, &current_rates, None)?;

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
	///
	/// This method delegates to the resource_operations module to ensure consistent
	/// collection logic across the application.
	#[instrument(skip(self))]
	pub fn collect_resources(&self, player_id: &PlayerKey) -> Result<PlayerResource> {
		let mut conn = self.db_pool.get()?;
		resource_operations::collect_resources(&mut conn, player_id)
	}

	/// Retrieves current resources rates for all resource types for a given player,
	/// applying all active modifiers to base resources rates.
	///
	/// This async version uses caching via ModifierService for efficient background processing.
	/// For handlers that need fresh values without caching, use resource_operations::calc_prod_rates()
	#[instrument(skip_all)]
	pub async fn get_production_rates(
		&self,
		player_key: &PlayerKey,
	) -> Result<HashMap<ResourceType, BigDecimal>> {
		// Fetch all relevant production modifiers for the player with caching
		let mut mods = HashMap::new();
		for res_type in ResourceType::iter() {
			let multiplier = self
				.modifier_service
				.get_total_multiplier(player_key, ModifierTarget::Resource, Some(res_type))
				.await?;
			mods.insert(res_type, multiplier);
		}
		debug!(?mods, "Calculated modifiers (cached)");

		// Get base rates using operations module
		let mut conn = self.db_pool.get()?;
		let base_rates = resource_operations::get_base_rates(&mut conn, player_key)?;

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
}
