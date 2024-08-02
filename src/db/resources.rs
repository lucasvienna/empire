use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::models::error::EmpResult;
use crate::models::resource;
use crate::models::resource::{NewResource, Resource};
use crate::schema::resources;

pub struct ResourcesRepository {}

impl Repository<Resource, NewResource, resource::PK> for ResourcesRepository {
    fn get_all(&self, connection: &mut DbConn) -> EmpResult<Vec<Resource>> {
        log::debug!("Getting all resources");
        let buildings = resources::table
            .select(Resource::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(&self, connection: &mut DbConn, user_id: &resource::PK) -> EmpResult<Resource> {
        log::debug!("Getting resource by ID: {}", user_id);
        let resource = resources::table.find(user_id).first(connection)?;
        log::debug!("Got resource: {:?}", resource);
        Ok(resource)
    }

    fn create(&mut self, connection: &mut DbConn, entity: &NewResource) -> EmpResult<Resource> {
        log::debug!("Creating resource: {:?}", entity);
        let resource = diesel::insert_into(resources::table)
            .values(entity)
            .returning(Resource::as_returning())
            .get_result(connection)?;
        log::debug!("Created resource: {:?}", resource);
        Ok(resource)
    }

    fn update(&mut self, connection: &mut DbConn, entity: &Resource) -> EmpResult<Resource> {
        log::debug!("Updating resource: {:?}", entity);
        let resource = diesel::update(resources::table.find(entity.user_id))
            .set(entity)
            .get_result(connection)?;
        log::debug!("Updated resource: {:?}", resource);
        Ok(resource)
    }

    fn delete(&mut self, connection: &mut DbConn, id: &resource::PK) -> EmpResult<usize> {
        log::debug!("Deleting resource: {}", id);
        let res = diesel::delete(resources::table.find(id)).execute(connection)?;
        log::debug!("Deleted resource: {}", res);
        Ok(res)
    }
}
