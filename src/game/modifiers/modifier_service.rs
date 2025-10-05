use std::sync::Arc;

use axum::extract::FromRef;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use tracing::{info, trace};

use crate::db::{active_modifiers, modifiers};
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::modifier::active_modifier::NewActiveModifier;
use crate::domain::modifier::ModifierTarget;
use crate::domain::player::resource::ResourceType;
use crate::domain::player::PlayerKey;
use crate::game::modifiers::modifier_cache::{CacheKey, ModifierCache};
use crate::game::modifiers::modifier_operations;
use crate::game::modifiers::modifier_scheduler::ModifierScheduler;
use crate::game::modifiers::modifier_system::ModifierSystem;
use crate::{Error, Result};

pub struct ModifierService {
	pool: AppPool,
	cache: Arc<ModifierCache>,
	scheduler: Arc<ModifierScheduler>,
}

impl FromRef<AppState> for ModifierService {
	fn from_ref(input: &AppState) -> Self {
		ModifierService::new(&input.db_pool, &input.modifier_system)
	}
}

impl ModifierService {
	pub fn new(pool: &AppPool, mod_system: &ModifierSystem) -> Self {
		// we expose the cache and scheduler instead of saving the mod sys to
		// drop one layer of indirection
		Self {
			pool: Arc::clone(pool),
			cache: Arc::clone(&mod_system.cache),
			scheduler: Arc::clone(&mod_system.scheduler),
		}
	}

	/// Apply a new modifier to a player and update all relevant systems
	pub async fn apply_modifier(&mut self, new_modifier: NewActiveModifier) -> Result<(), Error> {
		let mut conn = self.pool.get()?;

		// Store the modifier in the database
		let active_mod = active_modifiers::create(&mut conn, new_modifier)?;

		// Calculate new aggregate values for affected resources/targets
		let modifier = modifiers::get_by_id(&mut conn, &active_mod.modifier_id)?;
		let cache_key = CacheKey {
			player_id: active_mod.player_id,
			target_type: modifier.target_type,
			target_resource: modifier.target_resource,
		};

		// Invalidate existing cache entry
		self.cache.invalidate(&cache_key).await;

		// Calculate and cache new values
		let total_multiplier = modifier_operations::calc_multiplier(
			&mut conn,
			&active_mod.player_id,
			modifier.target_type,
			modifier.target_resource,
		)?;

		// Update cache with new values
		self.cache
			.set(cache_key, total_multiplier, active_mod.expires_at)
			.await?;

		// Schedule expiration job if needed
		if let Some(expires_at) = active_mod.expires_at {
			self.scheduler
				.schedule_expiration(active_mod.id, active_mod.player_id, expires_at)?;
		}

		info!(
			"Applied modifier {} to player {}",
			modifier.name, active_mod.player_id
		);
		Ok(())
	}

	/// Get the total modifier multiplier for a specific target and resource
	pub async fn get_total_multiplier(
		&self,
		player_id: &PlayerKey,
		target_type: ModifierTarget,
		target_resource: Option<ResourceType>,
	) -> Result<BigDecimal, Error> {
		let cache_key = CacheKey {
			player_id: *player_id,
			target_type,
			target_resource,
		};

		// Try to get from cache first
		if let Some(entry) = self.cache.get(&cache_key).await {
			trace!(%cache_key, total_multiplier = %entry.total_multiplier, "Cache hit for modifier calculation");
			return Ok(entry.total_multiplier);
		}

		// Calculate and cache if not found
		let total_multiplier = {
			let mut conn = self.pool.get()?;
			modifier_operations::calc_multiplier(&mut conn, player_id, target_type, target_resource)
		}?;
		trace!(%cache_key, %total_multiplier, "Cache miss, calculated total modifier");

		// Get the nearest expiration time from active modifiers
		let expires_at = self.get_nearest_expiration(player_id, target_type, target_resource)?;

		// Cache the result
		self.cache
			.set(cache_key, total_multiplier.clone(), expires_at)
			.await?;

		Ok(total_multiplier)
	}

	/// Get the nearest expiration time for modifiers matching the criteria
	fn get_nearest_expiration(
		&self,
		player_id: &PlayerKey,
		target_type: ModifierTarget,
		target_resource: Option<ResourceType>,
	) -> Result<Option<DateTime<Utc>>, Error> {
		let mut conn = self.pool.get()?;
		let active_modifiers = modifier_operations::get_active_mods(&mut conn, player_id)?;

		Ok(active_modifiers
			.into_iter()
			.filter_map(|m| m.expires_at)
			.min())
	}
}
