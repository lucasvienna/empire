use crate::db::{DbConn, Repository};
use crate::domain::error::Result;
use crate::domain::resource;
use crate::domain::resource::{NewResource, Resource};
use crate::schema::resources;
use diesel::prelude::*;
use tracing::debug;

#[derive(Debug)]
pub struct ResourcesRepository {}

impl Repository<Resource, NewResource, resource::PK> for ResourcesRepository {
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<Resource>> {
        debug!("Getting all resources");
        let buildings = resources::table
            .select(Resource::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(&self, connection: &mut DbConn, user_id: &resource::PK) -> Result<Resource> {
        debug!("Getting resource by ID: {}", user_id);
        let resource = resources::table.find(user_id).first(connection)?;
        debug!("Got resource: {:?}", resource);
        Ok(resource)
    }

    fn create(&self, connection: &mut DbConn, entity: &NewResource) -> Result<Resource> {
        debug!("Creating resource: {:?}", entity);
        let resource = diesel::insert_into(resources::table)
            .values(entity)
            .returning(Resource::as_returning())
            .get_result(connection)?;
        debug!("Created resource: {:?}", resource);
        Ok(resource)
    }

    fn update(&self, connection: &mut DbConn, entity: &Resource) -> Result<Resource> {
        debug!("Updating resource: {:?}", entity);
        let resource = diesel::update(resources::table.find(entity.user_id))
            .set(entity)
            .get_result(connection)?;
        debug!("Updated resource: {:?}", resource);
        Ok(resource)
    }

    fn delete(&self, connection: &mut DbConn, id: &resource::PK) -> Result<usize> {
        debug!("Deleting resource: {}", id);
        let res = diesel::delete(resources::table.find(id)).execute(connection)?;
        debug!("Deleted resource: {}", res);
        Ok(res)
    }
}

/// Deducts resources from a user: food, wood, stone, gold
pub type Deduction = (i32, i32, i32, i32);

impl ResourcesRepository {
    pub fn deduct(
        &self,
        connection: &mut DbConn,
        user_id: &resource::PK,
        amounts: &Deduction,
    ) -> Result<Resource> {
        debug!("Deducting resources {:?} from user {}", amounts, user_id);
        let res: Resource = resources::table.find(user_id).first(connection)?;
        debug!("Current resources: {:?}", res);
        let updated_res = diesel::update(resources::table.find(user_id))
            .set((
                resources::food.eq(resources::food - amounts.0),
                resources::wood.eq(resources::wood - amounts.1),
                resources::stone.eq(resources::stone - amounts.2),
                resources::gold.eq(resources::gold - amounts.3),
            ))
            .get_result(connection)?;
        debug!("Updated resources: {:?}", updated_res);
        Ok(updated_res)
    }
}
