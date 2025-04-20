use std::collections::HashMap;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use tracing::{debug, info};

use crate::db::active_modifiers::ActiveModifiersRepository;
use crate::db::modifiers::ModifiersRepository;
use crate::db::{DbPool, Repository};
use crate::domain::modifier::active_modifier::{ActiveModifier, NewActiveModifier};
use crate::domain::modifier::{Modifier, ModifierTarget};
use crate::domain::resource::ResourceType;
use crate::domain::user;
use crate::game::modifiers::modifier_cache::{CacheKey, ModifierCache};
use crate::game::modifiers::modifier_scheduler::ModifierScheduler;
use crate::Error;

pub struct ModifierService {
    cache: ModifierCache,
    scheduler: ModifierScheduler,
    mod_repo: ModifiersRepository,
    active_mod_repo: ActiveModifiersRepository,
    db_pool: DbPool,
}

impl ModifierService {
    pub fn new(
        cache: ModifierCache,
        scheduler: ModifierScheduler,
        mod_repo: ModifiersRepository,
        active_mod_repo: ActiveModifiersRepository,
        db_pool: DbPool,
    ) -> Self {
        Self {
            cache,
            scheduler,
            mod_repo,
            active_mod_repo,
            db_pool,
        }
    }

    /// Apply a new modifier to a user and update all relevant systems
    pub async fn apply_modifier(&mut self, new_modifier: NewActiveModifier) -> Result<(), Error> {
        let mut conn = self.db_pool.get()?;
        // Store the modifier in the database
        let active_mod = self.active_mod_repo.create(&mut conn, new_modifier)?;

        // Calculate new aggregate values for affected resources/targets
        let modifier = self
            .mod_repo
            .get_by_id(&mut conn, &active_mod.modifier_id)?;
        let cache_key = self.create_cache_key(
            active_mod.user_id,
            modifier.target_type,
            modifier.target_resource,
        );

        // Invalidate existing cache entry
        self.cache.invalidate(&cache_key).await;

        // Calculate and cache new values
        let total_multiplier = self
            .calculate_total_multiplier(
                active_mod.user_id,
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
                .schedule_expiration(active_mod.id, active_mod.user_id, expires_at)
                .await?;
        }

        info!(
            "Applied modifier {} to user {}",
            modifier.name, active_mod.user_id
        );
        Ok(())
    }

    /// Get all active modifiers for a user
    pub fn get_active_modifiers(&self, user_id: &user::PK) -> Result<Vec<ActiveModifier>, Error> {
        let mut conn = self.db_pool.get()?;
        self.active_mod_repo.get_by_user_id(&mut conn, user_id)
    }

    /// Get the total modifier multiplier for a specific target and resource
    pub async fn get_total_multiplier(
        &self,
        user_id: user::PK,
        target_type: ModifierTarget,
        target_resource: Option<ResourceType>,
    ) -> Result<BigDecimal, Error> {
        let cache_key = self.create_cache_key(user_id, target_type, target_resource);

        // Try to get from cache first
        if let Some(entry) = self.cache.get(&cache_key).await {
            debug!("Cache hit for modifier calculation");
            return Ok(entry.total_multiplier);
        }

        debug!("Cache miss for modifier calculation");
        // Calculate and cache if not found
        let total_multiplier = self
            .calculate_total_multiplier(user_id, target_type, target_resource)
            .await?;

        // Get the nearest expiration time from active modifiers
        let expires_at = self
            .get_nearest_expiration(user_id, target_type, target_resource)
            .await?;

        // Cache the result
        self.cache
            .set(cache_key, total_multiplier.clone(), expires_at)
            .await?;

        Ok(total_multiplier)
    }

    /// Calculate the total modifier multiplier from all active modifiers
    async fn calculate_total_multiplier(
        &self,
        user_id: user::PK,
        target_type: ModifierTarget,
        target_resource: Option<ResourceType>,
    ) -> Result<BigDecimal, Error> {
        let mut conn = self.db_pool.get()?;
        let active_modifiers = self.get_active_modifiers(&user_id)?;

        // Group modifiers by stacking group
        let mut stacking_groups: HashMap<Option<String>, Vec<Modifier>> = HashMap::new();

        for active_mod in active_modifiers {
            let modifier = self
                .mod_repo
                .get_by_id(&mut conn, &active_mod.modifier_id)?;

            // Filter by target type and resource
            if modifier.target_type != target_type {
                continue;
            }
            if modifier.target_resource != target_resource {
                continue;
            }

            stacking_groups
                .entry(modifier.stacking_group.clone())
                .or_default()
                .push(modifier);
        }

        // Calculate total multiplier using stacking rules
        let mut final_multiplier = BigDecimal::from(1);

        for (_, modifiers) in stacking_groups {
            let group_multiplier = self.apply_stacking_rules(&modifiers);
            final_multiplier *= group_multiplier;
        }

        Ok(final_multiplier)
    }

    /// Get the nearest expiration time for modifiers matching the criteria
    async fn get_nearest_expiration(
        &self,
        user_id: user::PK,
        target_type: ModifierTarget,
        target_resource: Option<ResourceType>,
    ) -> Result<Option<DateTime<Utc>>, Error> {
        let active_modifiers = self.get_active_modifiers(&user_id)?;

        Ok(active_modifiers
            .into_iter()
            .filter_map(|m| m.expires_at)
            .min())
    }

    /// Apply stacking rules to a group of modifiers
    fn apply_stacking_rules(&self, modifiers: &[Modifier]) -> BigDecimal {
        // For now, just add their magnitudes
        // TODO: Implement proper stacking rules based on modifier types
        modifiers
            .iter()
            .fold(BigDecimal::from(1), |acc, m| acc + &m.magnitude)
    }

    /// Create a cache key for the given parameters
    fn create_cache_key(
        &self,
        user_id: user::PK,
        target_type: ModifierTarget,
        target_resource: Option<ResourceType>,
    ) -> CacheKey {
        CacheKey {
            user_id,
            target_type,
            target_resource,
        }
    }
}
