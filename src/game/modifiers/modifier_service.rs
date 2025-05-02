use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::FromRef;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use tracing::{debug, info, trace};

use crate::db::active_modifiers::ActiveModifiersRepository;
use crate::db::modifiers::ModifiersRepository;
use crate::db::Repository;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::modifier::active_modifier::{ActiveModifier, NewActiveModifier};
use crate::domain::modifier::full_modifier::FullModifier;
use crate::domain::modifier::{Modifier, ModifierTarget, StackingBehaviour};
use crate::domain::player::resource::ResourceType;
use crate::domain::player::PlayerKey;
use crate::game::modifiers::modifier_cache::{CacheKey, ModifierCache};
use crate::game::modifiers::modifier_scheduler::ModifierScheduler;
use crate::game::modifiers::modifier_system::ModifierSystem;
use crate::{Error, Result};

pub struct ModifierService {
    pool: AppPool,
    cache: Arc<ModifierCache>,
    scheduler: Arc<ModifierScheduler>,
    mod_repo: ModifiersRepository,
    active_mod_repo: ActiveModifiersRepository,
}

impl FromRef<AppState> for ModifierService {
    fn from_ref(input: &AppState) -> Self {
        ModifierService::new(&input.db_pool, &input.modifier_system)
    }
}

impl ModifierService {
    pub fn new(pool: &AppPool, mod_system: &ModifierSystem) -> Self {
        Self {
            pool: Arc::clone(pool),
            cache: Arc::clone(&mod_system.cache),
            scheduler: Arc::clone(&mod_system.scheduler),
            mod_repo: ModifiersRepository::new(pool),
            active_mod_repo: ActiveModifiersRepository::new(pool),
        }
    }

    /// Apply a new modifier to a player and update all relevant systems
    pub async fn apply_modifier(&mut self, new_modifier: NewActiveModifier) -> Result<(), Error> {
        // Store the modifier in the database
        let active_mod = self.active_mod_repo.create(new_modifier)?;

        // Calculate new aggregate values for affected resources/targets
        let modifier = self.mod_repo.get_by_id(&active_mod.modifier_id)?;
        let cache_key = self.create_cache_key(
            &active_mod.player_id,
            modifier.target_type,
            modifier.target_resource,
        );

        // Invalidate existing cache entry
        self.cache.invalidate(&cache_key).await;

        // Calculate and cache new values
        let total_multiplier = self
            .calculate_total_multiplier(
                &active_mod.player_id,
                modifier.target_type,
                modifier.target_resource,
            )
            .await?;

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

    /// Get all active modifiers for a player
    fn get_active_modifiers(&self, player_id: &PlayerKey) -> Result<Vec<ActiveModifier>, Error> {
        let mut conn = self.pool.get()?;
        self.active_mod_repo.get_by_player_id(&mut conn, player_id)
    }

    fn get_full_modifiers(&self, player_key: &PlayerKey) -> Result<Vec<FullModifier>, Error> {
        use diesel::prelude::*;

        use crate::schema::active_modifiers::dsl::*;
        use crate::schema::modifiers::dsl::*;

        let mut conn = self.pool.get()?;
        let mods = active_modifiers
            .inner_join(modifiers)
            .filter(player_id.eq(player_key))
            .select((ActiveModifier::as_select(), Modifier::as_select()))
            .load::<(ActiveModifier, Modifier)>(&mut conn)?;

        Ok(mods.into_iter().map(|(am, m)| m.into_full(am)).collect())
    }

    /// Get the total modifier multiplier for a specific target and resource
    pub async fn get_total_multiplier(
        &self,
        player_id: &PlayerKey,
        target_type: ModifierTarget,
        target_resource: Option<ResourceType>,
    ) -> Result<BigDecimal, Error> {
        let cache_key = self.create_cache_key(player_id, target_type, target_resource);

        // Try to get from cache first
        if let Some(entry) = self.cache.get(&cache_key).await {
            debug!(?cache_key, "Cache hit for modifier calculation");
            return Ok(entry.total_multiplier);
        }

        debug!(?cache_key, "Cache miss for modifier calculation");
        // Calculate and cache if not found
        let total_multiplier = self
            .calculate_total_multiplier(player_id, target_type, target_resource)
            .await?;
        trace!("Total modifier multiplier: {}", total_multiplier);

        // Get the nearest expiration time from active modifiers
        let expires_at = self
            .get_nearest_expiration(player_id, target_type, target_resource)
            .await?;
        trace!(?expires_at, "Nearest expiration time");

        // Cache the result
        self.cache
            .set(cache_key, total_multiplier.clone(), expires_at)
            .await?;

        Ok(total_multiplier)
    }

    /// Calculate the total modifier multiplier from all active modifiers
    async fn calculate_total_multiplier(
        &self,
        player_id: &PlayerKey,
        target_type: ModifierTarget,
        target_resource: Option<ResourceType>,
    ) -> Result<BigDecimal, Error> {
        let player_mods = self.get_full_modifiers(player_id)?;
        let modifiers: Vec<FullModifier> = player_mods
            .into_iter()
            .filter(|m| m.target_type == target_type && m.target_resource == target_resource)
            .collect();

        self.calculate_modifiers(&modifiers)
    }

    /// Calculate the final modifier value for a collection of modifiers
    pub fn calculate_modifiers(&self, modifiers: &[FullModifier]) -> Result<BigDecimal> {
        let global_max_cap: BigDecimal = BigDecimal::from(3); // 300%
        let global_min_floor: BigDecimal =
            BigDecimal::try_from(0.5).expect("Failed to create a 0.5 numeric."); // 50%
        let base = BigDecimal::from(1);

        if modifiers.is_empty() {
            return Ok(base);
        }

        // Step 1: Group modifiers by their stacking behavior
        let mut additive_mods: Vec<&FullModifier> = Vec::new();
        let mut multiplicative_mods: Vec<&FullModifier> = Vec::new();
        let mut highest_only_groups: HashMap<String, Vec<&FullModifier>> = HashMap::new();

        for modifier in modifiers {
            match modifier.stacking_behaviour {
                StackingBehaviour::Additive => additive_mods.push(modifier),
                StackingBehaviour::Multiplicative => multiplicative_mods.push(modifier),
                StackingBehaviour::HighestOnly => {
                    let group = modifier
                        .stacking_group
                        .clone()
                        .unwrap_or_else(|| modifier.get_stacking_group());
                    highest_only_groups.entry(group).or_default().push(modifier);
                }
            }
        }

        // Step 2: Calculate additive modifiers
        let additive_total = additive_mods
            .iter()
            .fold(BigDecimal::from(0), |acc, m| acc + &m.magnitude);

        // Step 3: Calculate highest-only modifiers
        let highest_only_values: Vec<BigDecimal> = highest_only_groups
            .values()
            .map(|group| {
                group
                    .iter()
                    .map(|m| &m.magnitude)
                    .max_by(|a, b| a.cmp(b))
                    .unwrap_or(&BigDecimal::from(0))
                    .clone()
            })
            .collect();

        // Step 4: Calculate multiplicative effect
        let multiplicative_total = multiplicative_mods
            .iter()
            .map(|m| &m.magnitude)
            .chain(highest_only_values.iter())
            .fold(base.clone(), |acc, magnitude| {
                acc * (base.clone() + magnitude)
            });

        // Step 5: Combine all effects
        let total = (base + additive_total) * multiplicative_total;

        // Step 6: Apply global caps and floors
        let final_value = if total > global_max_cap {
            global_max_cap
        } else if total < global_min_floor {
            global_min_floor
        } else {
            total
        };

        Ok(final_value)
    }

    /// Get the nearest expiration time for modifiers matching the criteria
    async fn get_nearest_expiration(
        &self,
        player_id: &PlayerKey,
        target_type: ModifierTarget,
        target_resource: Option<ResourceType>,
    ) -> Result<Option<DateTime<Utc>>, Error> {
        let active_modifiers = self.get_active_modifiers(player_id)?;

        Ok(active_modifiers
            .into_iter()
            .filter_map(|m| m.expires_at)
            .min())
    }

    /// Create a cache key for the given parameters
    fn create_cache_key(
        &self,
        player_id: &PlayerKey,
        target_type: ModifierTarget,
        target_resource: Option<ResourceType>,
    ) -> CacheKey {
        CacheKey {
            player_id: *player_id,
            target_type,
            target_resource,
        }
    }
}
