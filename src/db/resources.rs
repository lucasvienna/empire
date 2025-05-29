//! Repository implementation for managing player resources in the database.
//! Provides CRUD operations and resource deduction functionality.

use std::fmt;
use std::sync::Arc;

use diesel::prelude::*;
use tracing::{debug, info, instrument, trace};

use crate::db::{DbConn, Repository};
use crate::domain::app_state::AppPool;
use crate::domain::error::Result;
use crate::domain::player::resource::{
    NewPlayerResource, PlayerResource, PlayerResourceKey, UpdatePlayerResource,
};
use crate::domain::player::PlayerKey;
use crate::schema::player_resource::dsl::*;

/// Repository for managing player resources in the database.
/// Provides CRUD operations and resource deduction functionality.
///
/// # Fields
/// * `pool` - Thread-safe connection pool of type [`AppPool`] for database access
pub struct ResourcesRepository {
    pool: AppPool,
}

impl fmt::Debug for ResourcesRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResourcesRepository")
    }
}

impl Repository<PlayerResource, NewPlayerResource, &UpdatePlayerResource, PlayerResourceKey>
    for ResourcesRepository
{
    fn new(pool: &AppPool) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    #[instrument(skip(self))]
    fn get_all(&self) -> Result<Vec<PlayerResource>> {
        debug!("Starting get all resources");
        let mut conn = self.pool.get()?;
        let resources = player_resource
            .select(PlayerResource::as_select())
            .load(&mut conn)?;
        info!("Completed get all resources, count: {}", resources.len());
        Ok(resources)
    }

    #[instrument(skip(self))]
    fn get_by_id(&self, player_key: &PlayerResourceKey) -> Result<PlayerResource> {
        debug!("Starting get resource by ID: {}", player_key);
        let mut conn = self.pool.get()?;
        let resource = player_resource.find(player_key).first(&mut conn)?;
        trace!("Got resource details: {:?}", resource);
        info!("Completed get resource for ID: {}", player_key);
        Ok(resource)
    }

    #[instrument(skip(self, entity))]
    fn create(&self, entity: NewPlayerResource) -> Result<PlayerResource> {
        debug!("Starting create resource for player: {}", entity.player_id);
        let mut conn = self.pool.get()?;
        let resource = diesel::insert_into(player_resource)
            .values(entity)
            .returning(PlayerResource::as_returning())
            .get_result(&mut conn)?;
        trace!("Created resource details: {:?}", resource);
        info!(
            "Completed create resource for player: {}",
            resource.player_id
        );
        Ok(resource)
    }

    #[instrument(skip(self, changeset))]
    fn update(&self, changeset: &UpdatePlayerResource) -> Result<PlayerResource> {
        debug!(
            "Starting update resource for player: {}",
            changeset.player_id
        );
        let mut conn = self.pool.get()?;
        let resource = diesel::update(player_resource)
            .set(changeset)
            .returning(PlayerResource::as_returning())
            .get_result(&mut conn)?;
        trace!("Updated resource details: {:?}", resource);
        info!(
            "Completed update resource for player: {}",
            resource.player_id
        );
        Ok(resource)
    }

    #[instrument(skip(self))]
    fn delete(&self, resource_key: &PlayerResourceKey) -> Result<usize> {
        debug!("Starting delete resource: {}", resource_key);
        let mut conn = self.pool.get()?;
        let count = diesel::delete(player_resource.find(resource_key)).execute(&mut conn)?;
        info!(
            "Completed delete resource: {}, count: {}",
            resource_key, count
        );
        Ok(count)
    }
}

/// Represents resource amounts to be deducted from a player's resources.
/// The tuple contains amounts in the following order:
/// * food (i64)
/// * wood (i64)
/// * stone (i64)
/// * gold (i64)
pub type Deduction = (i64, i64, i64, i64);

impl ResourcesRepository {
    /// Retrieves resource information for a specific player by their player ID.
    ///
    /// # Arguments
    /// * `player_key` - The unique identifier of the player
    ///
    /// # Returns
    /// * `Result<PlayerResource>` - The player's resource information if found
    #[instrument(skip(self))]
    pub fn get_by_player_id(&self, player_key: &PlayerKey) -> Result<PlayerResource> {
        debug!("Getting player resources");
        let mut conn = self.pool.get()?;
        let res = player_resource
            .select(PlayerResource::as_select())
            .filter(player_id.eq(player_key))
            .first(&mut conn)?;
        trace!("Fetched player resource details: {:?}", res);
        Ok(res)
    }

    /// Deducts specified amounts of resources from a player's resource pool.
    ///
    /// # Arguments
    /// * `player_key` - The unique identifier of the player
    /// * `amounts` - The amounts to deduct as a tuple of (food, wood, stone, gold)
    ///
    /// # Returns
    /// * `Result<PlayerResource>` - The updated player resources after deduction
    #[instrument(skip(self, conn))]
    pub fn deduct(
        &self,
        conn: &mut DbConn,
        player_key: &PlayerKey,
        amounts: &Deduction,
    ) -> Result<PlayerResource> {
        debug!(
            "Starting deduct resources from player {}: food={}, wood={}, stone={}, gold={}",
            player_key, amounts.0, amounts.1, amounts.2, amounts.3
        );
        let res: PlayerResource = player_resource
            .filter(player_id.eq(player_key))
            .first(conn)?;
        trace!("Current resources before deduction: {:?}", res);
        let updated_res = diesel::update(player_resource.filter(player_id.eq(player_key)))
            .set((
                food.eq(food - amounts.0),
                wood.eq(wood - amounts.1),
                stone.eq(stone - amounts.2),
                gold.eq(gold - amounts.3),
            ))
            .returning(PlayerResource::as_returning())
            .get_result(conn)?;
        trace!("Updated resources after deduction: {:?}", updated_res);
        info!(
            "Completed deduct resources from player {}: food={}, wood={}, stone={}, gold={}",
            player_key, amounts.0, amounts.1, amounts.2, amounts.3
        );
        Ok(updated_res)
    }
}
