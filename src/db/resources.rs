//! Repository implementation for managing player resources in the database.
//! Provides CRUD operations and resource deduction functionality.

use std::fmt;
use std::sync::Arc;

use diesel::prelude::*;
use tracing::debug;

use crate::db::{DbConn, Repository};
use crate::domain::app_state::AppPool;
use crate::domain::error::Result;
use crate::domain::player::resource::{
    NewPlayerResource, PlayerResource, PlayerResourceKey, UpdatePlayerResource,
};
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

    fn get_all(&self) -> Result<Vec<PlayerResource>> {
        debug!("Getting all resources");
        let mut conn = self.pool.get()?;
        let buildings = player_resource
            .select(PlayerResource::as_select())
            .load(&mut conn)?;
        Ok(buildings)
    }

    fn get_by_id(&self, player_key: &PlayerResourceKey) -> Result<PlayerResource> {
        debug!("Getting resource by ID: {}", player_key);
        let mut conn = self.pool.get()?;
        let resource = player_resource.find(player_key).first(&mut conn)?;
        debug!("Got resource: {:?}", resource);
        Ok(resource)
    }

    fn create(&self, entity: NewPlayerResource) -> Result<PlayerResource> {
        debug!("Creating resource: {:?}", entity);
        let mut conn = self.pool.get()?;
        let resource = diesel::insert_into(player_resource)
            .values(entity)
            .returning(PlayerResource::as_returning())
            .get_result(&mut conn)?;
        debug!("Created resource: {:?}", resource);
        Ok(resource)
    }

    fn update(&self, changeset: &UpdatePlayerResource) -> Result<PlayerResource> {
        debug!(
            "Updating resource '{}': {:?}",
            changeset.player_id, changeset
        );
        let mut conn = self.pool.get()?;
        let resource = diesel::update(player_resource)
            .set(changeset)
            .get_result(&mut conn)?;
        debug!("Updated resource: {:?}", resource);
        Ok(resource)
    }

    fn delete(&self, resource_key: &PlayerResourceKey) -> Result<usize> {
        debug!("Deleting resource: {}", resource_key);
        let mut conn = self.pool.get()?;
        let res = diesel::delete(player_resource.find(resource_key)).execute(&mut conn)?;
        debug!("Deleted resource: {}", res);
        Ok(res)
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
    /// Deducts specified amounts of resources from a player's resource pool.
    ///
    /// # Arguments
    /// * `player_key` - The unique identifier of the player
    /// * `amounts` - The amounts to deduct as a tuple of (food, wood, stone, gold)
    ///
    /// # Returns
    /// * `Result<PlayerResource>` - The updated player resources after deduction
    pub fn deduct(
        &self,
        conn: &mut DbConn,
        player_key: &PlayerResourceKey,
        amounts: &Deduction,
    ) -> Result<PlayerResource> {
        debug!(
            "Deducting resources {:?} from player {}",
            amounts, player_key
        );
        let res: PlayerResource = player_resource.find(player_key).first(conn)?;
        debug!("Current resources: {:?}", res);
        let updated_res = diesel::update(player_resource.find(player_key))
            .set((
                food.eq(food - amounts.0),
                wood.eq(wood - amounts.1),
                stone.eq(stone - amounts.2),
                gold.eq(gold - amounts.3),
            ))
            .get_result(conn)?;
        debug!("Updated resources: {:?}", updated_res);
        Ok(updated_res)
    }
}
