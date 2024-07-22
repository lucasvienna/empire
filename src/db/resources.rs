use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::models::error::EmpResult;
use crate::models::resource;
use crate::models::resource::{NewResource, Resource};
use crate::schema::resources;

pub struct ResourcesRepository {}

impl Repository<Resource, NewResource, resource::PK> for ResourcesRepository {
    fn get_all(&self, connection: &mut DbConn) -> EmpResult<Vec<Resource>> {
        let buildings = resources::table
            .select(Resource::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(&self, connection: &mut DbConn, user_id: &resource::PK) -> EmpResult<Resource> {
        let building = resources::table.find(user_id).first(connection)?;
        Ok(building)
    }

    fn create(&mut self, connection: &mut DbConn, entity: &NewResource) -> EmpResult<Resource> {
        let building = diesel::insert_into(resources::table)
            .values(entity)
            .returning(Resource::as_returning())
            .get_result(connection)?;
        Ok(building)
    }

    fn update(&mut self, connection: &mut DbConn, entity: &Resource) -> EmpResult<Resource> {
        let building = diesel::update(resources::table.find(entity.user_id))
            .set(entity)
            .get_result(connection)?;
        Ok(building)
    }

    fn delete(&mut self, connection: &mut DbConn, id: &resource::PK) -> EmpResult<()> {
        diesel::delete(resources::table.find(id)).execute(connection)?;
        Ok(())
    }
}
