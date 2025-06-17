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
            modifiers: mod_system.clone(), // essentially cloning two pointers, very inexpensive
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
        // calculate delta between last resources and now
        let last_prod = self.last_player_prod(player_key).unwrap_or_default();
        let delta_secs = BigDecimal::from((Utc::now() - last_prod).num_seconds());
        let seconds_per_hours = BigDecimal::from(3600);
        let delta_hours = (delta_secs / seconds_per_hours).round(5);
        // multiply resources rate proportionally to delta
        let current_rates = self.cur_prod_rates(player_key).await?;
        debug!(
            "Production Delta: {}h, last produced at: {}",
            delta_hours, last_prod
        );
        debug!("Current Rates: {:?}", current_rates);
        let prod_amounts: HashMap<ResourceType, i64> = current_rates
            .into_iter()
            .map(|(res_type, prod_rate)| {
                let amount = prod_rate * &delta_hours;
                let truncated: i64 = amount.to_i64().unwrap_or_default();
                (res_type, truncated)
            })
            .collect();
        debug!(
            "Producing resources for player {}: {:?}",
            player_key, prod_amounts
        );

        // add to accumulator up to the cap
        let mut conn = self.db_pool.get()?;
        let player_acc = conn.transaction(|conn| -> Result<PlayerAccumulator> {
            use crate::custom_schema::resource_generation::dsl as rg;
            use crate::schema::player_accumulator::dsl::{
                food, gold, id, player_accumulator, stone, wood,
            };
            use crate::schema::player_resource::dsl::{self as pr, player_resource};

            trace!("Entering accumulator transaction");
            let acc_key: AccumulatorKey = {
                use crate::schema::player_accumulator::dsl::{id, player_accumulator, player_id};
                player_accumulator
                    .select(id)
                    .filter(player_id.eq(player_key))
                    .for_update()
                    .first(conn)?
            };
            trace!("Found player accumulator: {:?}", acc_key);
            let acc_caps: ResourceGeneration =
                rg::resource_generation.find(player_key).first(conn)?;

            let produced_food = prod_amounts.get(&ResourceType::Food).unwrap_or(&0);
            let produced_wood = prod_amounts.get(&ResourceType::Wood).unwrap_or(&0);
            let produced_stone = prod_amounts.get(&ResourceType::Stone).unwrap_or(&0);
            let produced_gold = prod_amounts.get(&ResourceType::Gold).unwrap_or(&0);

            let res = diesel::update(player_accumulator)
                .filter(id.eq(&acc_key))
                .set((
                    food.eq(least(food + produced_food, acc_caps.food_acc_cap)),
                    wood.eq(least(wood + produced_wood, acc_caps.wood_acc_cap)),
                    stone.eq(least(stone + produced_stone, acc_caps.stone_acc_cap)),
                    gold.eq(least(gold + produced_gold, acc_caps.gold_acc_cap)),
                ))
                .returning(PlayerAccumulator::as_returning())
                .get_result(conn)?;
            debug!("New accumulator state: {:?}", res);

            let updated = diesel::update(player_resource)
                .set(pr::produced_at.eq(Utc::now()))
                .execute(conn)?;
            assert_eq!(
                updated, 1,
                "Did not update last produced at. Expected 1, got none."
            );

            Ok(res)
        })?;

        // enqueue next resources job
        let next_production_time = Utc::now() + Duration::minutes(2);
        let _ = self
            .resource_scheduler
            .schedule_production(player_key, next_production_time)
            .map_err(|err| {
                warn!("Failed to schedule next production: {:#?}", err);
                err
            });

        Ok(())
    }

    /// Collects resources for a player by transferring the maximum possible amount from their
    /// resource accumulator to their resource storage, constrained by the storage capacity limits.
    ///
    /// # Parameters
    ///
    /// - `player_id`: Reference to the primary key of the player whose resources will be collected.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the updated `Resource` structure representing the player's
    /// updated resources or an error if the operation fails.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following conditions occur:
    /// - The database query to calculate the collectible resource amounts fails.
    /// - The transaction to update both the accumulator and the resource storage fails.
    #[instrument(skip(self))]
    pub fn collect_resources(&self, player_id: &PlayerKey) -> Result<PlayerResource> {
        let mut connection = self.db_pool.get()?;
        let (food, wood, stone, gold) = crate::schema::player_accumulator::table
            .inner_join(
                crate::schema::player_resource::table
                    .on(crate::schema::player_accumulator::player_id
                        .eq(crate::schema::player_resource::player_id)),
            )
            .filter(
                crate::schema::player_accumulator::player_id
                    .nullable()
                    .eq(player_id),
            )
            .select((
                least(
                    crate::schema::player_accumulator::food,
                    crate::schema::player_resource::food_cap - crate::schema::player_resource::food,
                ),
                least(
                    crate::schema::player_accumulator::wood,
                    crate::schema::player_resource::wood_cap - crate::schema::player_resource::wood,
                ),
                least(
                    crate::schema::player_accumulator::stone,
                    crate::schema::player_resource::stone_cap
                        - crate::schema::player_resource::stone,
                ),
                least(
                    crate::schema::player_accumulator::gold,
                    crate::schema::player_resource::gold_cap - crate::schema::player_resource::gold,
                ),
            ))
            .first::<(i64, i64, i64, i64)>(&mut connection)?;
        debug!("Deltas: {:?}", (food, wood, stone, gold));

        let res: Result<PlayerResource> = connection.transaction(|conn| {
            // drain accumulator first
            let updated_rows = diesel::update(
                crate::schema::player_accumulator::table
                    .filter(crate::schema::player_accumulator::player_id.eq(player_id)),
            )
            .set((
                crate::schema::player_accumulator::food
                    .eq(crate::schema::player_accumulator::food - food),
                crate::schema::player_accumulator::wood
                    .eq(crate::schema::player_accumulator::wood - wood),
                crate::schema::player_accumulator::stone
                    .eq(crate::schema::player_accumulator::stone - stone),
                crate::schema::player_accumulator::gold
                    .eq(crate::schema::player_accumulator::gold - gold),
            ))
            .execute(conn)?;

            // then increase resources
            let res = diesel::update(
                crate::schema::player_resource::table
                    .filter(crate::schema::player_resource::player_id.eq(player_id)),
            )
            .set((
                crate::schema::player_resource::food
                    .eq(crate::schema::player_resource::food + food),
                crate::schema::player_resource::wood
                    .eq(crate::schema::player_resource::wood + wood),
                crate::schema::player_resource::stone
                    .eq(crate::schema::player_resource::stone + stone),
                crate::schema::player_resource::gold
                    .eq(crate::schema::player_resource::gold + gold),
            ))
            .returning(PlayerResource::as_returning())
            .get_result(conn)?;

            Ok(res)
        });

        res
    }

    /// Retrieves current resources rates for all resource types for a given player,
    /// applying all active modifiers to base resources rates.
    ///
    /// # Arguments
    /// * `player_key` - The unique identifier of the player
    ///
    /// # Returns
    /// HashMap containing resource types mapped to their current hourly resources rates
    #[instrument(skip_all)]
    async fn cur_prod_rates(
        &self,
        player_key: &PlayerKey,
    ) -> Result<HashMap<ResourceType, BigDecimal>> {
        let mut mods: HashMap<ResourceType, BigDecimal> = HashMap::new();
        for res in ResourceType::iter() {
            let multi = self
                .modifier_service
                .get_total_multiplier(player_key, ModifierTarget::Resource, Some(res))
                .await?;
            mods.insert(res, multi);
        }
        debug!(?mods, "Calculated modifiers");
        let rates = {
            use crate::custom_schema::resource_generation::dsl::{player_id, resource_generation};

            let mut conn = self.db_pool.get()?;
            resource_generation
                .select(ResourceGeneration::as_select())
                .filter(player_id.eq(player_key))
                .first(&mut conn)?
        };

        let current_hourly_rates = mods
            .into_iter()
            .map(|(key, mut modifier)| {
                modifier = match key {
                    ResourceType::Population => BigDecimal::from(rates.population) * modifier,
                    ResourceType::Food => BigDecimal::from(rates.food) * modifier,
                    ResourceType::Wood => BigDecimal::from(rates.wood) * modifier,
                    ResourceType::Stone => BigDecimal::from(rates.stone) * modifier,
                    ResourceType::Gold => BigDecimal::from(rates.gold) * modifier,
                };
                (key, modifier)
            })
            .collect();

        Ok(current_hourly_rates)
    }

    /// Gets the timestamp of the last resource resources for a player.
    ///
    /// # Arguments
    /// * `player_key` - The unique identifier of the player
    ///
    /// # Returns
    /// The UTC timestamp of when resources were last produced for the player
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
