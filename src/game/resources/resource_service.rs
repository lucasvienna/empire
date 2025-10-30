use std::sync::Arc;

use axum::extract::FromRef;
use chrono::{Duration, Utc};
use tracing::{instrument, warn};

use crate::Result;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::jobs::JobKey;
use crate::domain::player::PlayerKey;
use crate::domain::player::accumulator::PlayerAccumulator;
use crate::domain::player::resource::PlayerResource;
use crate::domain::resource_generation::ResourceGeneration;
use crate::game::resources::resource_scheduler::ProductionScheduler;
use crate::game::resources::{ResourceProductionRates, resource_operations};
use crate::job_queue::JobQueue;

/// Service responsible for managing resources for players.
/// Handles resource production and collection operations, with scheduling support.
pub struct ResourceService {
	pool: AppPool,
	scheduler: ProductionScheduler,
}

impl FromRef<AppState> for ResourceService {
	fn from_ref(state: &AppState) -> Self {
		Self::new(&state.db_pool, &state.job_queue)
	}
}

impl ResourceService {
	pub fn new(pool: &AppPool, queue: &Arc<JobQueue>) -> Self {
		Self {
			pool: Arc::clone(pool),
			scheduler: ProductionScheduler::new(queue),
		}
	}

	/// Produces resources for a player based on pre-calculated production rates.
	/// Updates the player's accumulator with produced resources and schedules the next production job.
	///
	/// # Arguments
	/// * `player_key` - The unique identifier of the player to produce resources for
	/// * `production_rates` - Pre-calculated production rates (with modifiers already applied)
	#[instrument(skip(self, production_rates))]
	pub async fn produce(
		&self,
		player_key: &PlayerKey,
		production_rates: &ResourceProductionRates,
	) -> Result<(PlayerAccumulator, JobKey)> {
		// Delegate to operations module for production logic
		let mut conn = self.pool.get()?;
		let acc =
			resource_operations::produce_resources(&mut conn, player_key, production_rates, None)?;

		// Enqueue the next production job.
		// The 2-minute interval is a simple polling mechanism.
		let next_production_time = Utc::now() + Duration::minutes(2);
		let job_key = self
			.scheduler
			.schedule_production(player_key, next_production_time)
			.map_err(|err| {
				warn!("Failed to schedule next production: {:#?}", err);
				err
			})?;

		Ok((acc, job_key))
	}

	/// Collects resources for a player by transferring the maximum possible amount from their
	/// resource accumulator to their resource storage, constrained by the storage capacity limits.
	///
	/// # Arguments
	/// * `player_key` - The unique identifier of the player whose resources are being collected
	#[instrument(skip(self))]
	pub fn collect(&self, player_key: &PlayerKey) -> Result<PlayerResource> {
		let mut conn = self.pool.get()?;
		resource_operations::collect_resources(&mut conn, player_key)
	}

	/// Retrieves the base resource generation rates for a specific player.
	///
	/// This method fetches the unmodified base rates at which different resources
	/// are generated for the player. These rates serve as the foundation for
	/// actual resource production calculations when combined with modifiers.
	///
	/// # Arguments
	/// * `player_key` - The unique identifier of the player whose base rates are being queried
	pub fn get_base_rates(&self, player_key: &PlayerKey) -> Result<ResourceGeneration> {
		let mut conn = self.pool.get()?;
		resource_operations::get_base_rates(&mut conn, player_key)
	}
}
