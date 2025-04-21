//! Repository implementation for managing player resources in the database.
//! Provides CRUD operations and resource deduction functionality.

use std::fmt;

use diesel::prelude::*;
use tracing::debug;

use crate::db::{DbConn, DbPool, Repository};
use crate::domain::error::Result;
use crate::domain::player::resource::{
    NewPlayerResource, PlayerResource, PlayerResourceKey, UpdatePlayerResource,
};
use crate::schema::player_resource::dsl::*;

/// Repository for managing player resources in the database.
/// Provides CRUD operations and resource deduction functionality.
///
/// # Fields
/// * `connection` - Database connection pool
pub struct ResourcesRepository {
    connection: DbConn,
}

impl fmt::Debug for ResourcesRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResourcesRepository")
    }
}

impl Repository<PlayerResource, NewPlayerResource, &UpdatePlayerResource, PlayerResourceKey>
    for ResourcesRepository
{
    fn try_from_pool(pool: &DbPool) -> Result<Self> {
        Ok(Self {
            connection: pool.get()?,
        })
    }

    fn from_connection(connection: DbConn) -> Self {
        Self { connection }
    }

    fn get_all(&mut self) -> Result<Vec<PlayerResource>> {
        debug!("Getting all resources");
        let buildings = player_resource
            .select(PlayerResource::as_select())
            .load(&mut self.connection)?;
        Ok(buildings)
    }

    fn get_by_id(&mut self, player_key: &PlayerResourceKey) -> Result<PlayerResource> {
        debug!("Getting resource by ID: {}", player_key);
        let resource = player_resource
            .find(player_key)
            .first(&mut self.connection)?;
        debug!("Got resource: {:?}", resource);
        Ok(resource)
    }

    fn create(&mut self, entity: NewPlayerResource) -> Result<PlayerResource> {
        debug!("Creating resource: {:?}", entity);
        let resource = diesel::insert_into(player_resource)
            .values(entity)
            .returning(PlayerResource::as_returning())
            .get_result(&mut self.connection)?;
        debug!("Created resource: {:?}", resource);
        Ok(resource)
    }

    fn update(&mut self, changeset: &UpdatePlayerResource) -> Result<PlayerResource> {
        debug!(
            "Updating resource '{}': {:?}",
            changeset.player_id, changeset
        );
        let resource = diesel::update(player_resource)
            .set(changeset)
            .get_result(&mut self.connection)?;
        debug!("Updated resource: {:?}", resource);
        Ok(resource)
    }

    fn delete(&mut self, resource_key: &PlayerResourceKey) -> Result<usize> {
        debug!("Deleting resource: {}", resource_key);
        let res =
            diesel::delete(player_resource.find(resource_key)).execute(&mut self.connection)?;
        debug!("Deleted resource: {}", res);
        Ok(res)
    }
}

/// Represents resource amounts to be deducted from a player's resources.
/// The tuple contains amounts in the following order:
/// * food (i32)
/// * wood (i32)
/// * stone (i32)
/// * gold (i32)
pub type Deduction = (i32, i32, i32, i32);

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
        &mut self,
        player_key: &PlayerResourceKey,
        amounts: &Deduction,
    ) -> Result<PlayerResource> {
        debug!(
            "Deducting resources {:?} from player {}",
            amounts, player_key
        );
        let res: PlayerResource = player_resource
            .find(player_key)
            .first(&mut self.connection)?;
        debug!("Current resources: {:?}", res);
        let updated_res = diesel::update(player_resource.find(player_key))
            .set((
                food.eq(food - amounts.0),
                wood.eq(wood - amounts.1),
                stone.eq(stone - amounts.2),
                gold.eq(gold - amounts.3),
            ))
            .get_result(&mut self.connection)?;
        debug!("Updated resources: {:?}", updated_res);
        Ok(updated_res)
    }
}
