use diesel::prelude::*;
use tracing::debug;

use crate::db::{DbConn, Repository};
use crate::domain::error::Result;
use crate::domain::player::resource::{
    self, NewPlayerResource, PlayerResource, UpdatePlayerResource,
};
use crate::schema::player_resource::dsl::*;

#[derive(Debug)]
pub struct ResourcesRepository {}

impl Repository<PlayerResource, NewPlayerResource, &UpdatePlayerResource, resource::PK>
    for ResourcesRepository
{
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<PlayerResource>> {
        debug!("Getting all resources");
        let buildings = player_resource
            .select(PlayerResource::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(
        &self,
        connection: &mut DbConn,
        player_key: &resource::PK,
    ) -> Result<PlayerResource> {
        debug!("Getting resource by ID: {}", player_key);
        let resource = player_resource.find(player_key).first(connection)?;
        debug!("Got resource: {:?}", resource);
        Ok(resource)
    }

    fn create(&self, connection: &mut DbConn, entity: NewPlayerResource) -> Result<PlayerResource> {
        debug!("Creating resource: {:?}", entity);
        let resource = diesel::insert_into(player_resource)
            .values(entity)
            .returning(PlayerResource::as_returning())
            .get_result(connection)?;
        debug!("Created resource: {:?}", resource);
        Ok(resource)
    }

    fn update(
        &self,
        connection: &mut DbConn,
        changeset: &UpdatePlayerResource,
    ) -> Result<PlayerResource> {
        debug!(
            "Updating resource '{}': {:?}",
            changeset.player_id, changeset
        );
        let resource = diesel::update(player_resource)
            .set(changeset)
            .get_result(connection)?;
        debug!("Updated resource: {:?}", resource);
        Ok(resource)
    }

    fn delete(&self, connection: &mut DbConn, id: &resource::PK) -> Result<usize> {
        debug!("Deleting resource: {}", id);
        let res = diesel::delete(player_resource.find(id)).execute(connection)?;
        debug!("Deleted resource: {}", res);
        Ok(res)
    }
}

/// Deducts resources from a player: food, wood, stone, gold
pub type Deduction = (i32, i32, i32, i32);

impl ResourcesRepository {
    pub fn deduct(
        &self,
        connection: &mut DbConn,
        player_key: &resource::PK,
        amounts: &Deduction,
    ) -> Result<PlayerResource> {
        debug!(
            "Deducting resources {:?} from player {}",
            amounts, player_key
        );
        let res: PlayerResource = player_resource.find(player_key).first(connection)?;
        debug!("Current resources: {:?}", res);
        let updated_res = diesel::update(player_resource.find(player_key))
            .set((
                food.eq(food - amounts.0),
                wood.eq(wood - amounts.1),
                stone.eq(stone - amounts.2),
                gold.eq(gold - amounts.3),
            ))
            .get_result(connection)?;
        debug!("Updated resources: {:?}", updated_res);
        Ok(updated_res)
    }
}
