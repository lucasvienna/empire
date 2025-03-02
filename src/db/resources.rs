use diesel::prelude::*;
use tracing::debug;

use crate::db::{DbConn, Repository};
use crate::domain::error::Result;
use crate::domain::resource::{self, NewResource, UpdateResource, UserResource};
use crate::schema::user_resources::dsl::*;

#[derive(Debug)]
pub struct ResourcesRepository {}

impl Repository<UserResource, NewResource, UpdateResource, resource::PK> for ResourcesRepository {
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<UserResource>> {
        debug!("Getting all resources");
        let buildings = user_resources
            .select(UserResource::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(&self, connection: &mut DbConn, user_pk: &resource::PK) -> Result<UserResource> {
        debug!("Getting resource by ID: {}", user_pk);
        let resource = user_resources.find(user_pk).first(connection)?;
        debug!("Got resource: {:?}", resource);
        Ok(resource)
    }

    fn create(&self, connection: &mut DbConn, entity: NewResource) -> Result<UserResource> {
        debug!("Creating resource: {:?}", entity);
        let resource = diesel::insert_into(user_resources)
            .values(entity)
            .returning(UserResource::as_returning())
            .get_result(connection)?;
        debug!("Created resource: {:?}", resource);
        Ok(resource)
    }

    fn update(
        &self,
        connection: &mut DbConn,
        id: &resource::PK,
        changeset: UpdateResource,
    ) -> Result<UserResource> {
        debug!("Updating resource '{}': {:?}", id, changeset);
        let resource = diesel::update(user_resources.find(id))
            .set(changeset)
            .get_result(connection)?;
        debug!("Updated resource: {:?}", resource);
        Ok(resource)
    }

    fn delete(&self, connection: &mut DbConn, id: &resource::PK) -> Result<usize> {
        debug!("Deleting resource: {}", id);
        let res = diesel::delete(user_resources.find(id)).execute(connection)?;
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
        user_pk: &resource::PK,
        amounts: &Deduction,
    ) -> Result<UserResource> {
        debug!("Deducting resources {:?} from user {}", amounts, user_pk);
        let res: UserResource = user_resources.find(user_pk).first(connection)?;
        debug!("Current resources: {:?}", res);
        let updated_res = diesel::update(user_resources.find(user_pk))
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
