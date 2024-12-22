use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::models::building;
use crate::models::building::{Building, NewBuilding};
use crate::models::error::EmpResult;
use crate::schema::buildings;

#[derive(Debug)]
pub struct BuildingRepository {}

impl Repository<Building, NewBuilding<'static>, building::PK> for BuildingRepository {
    fn get_all(&self, connection: &mut DbConn) -> EmpResult<Vec<Building>> {
        let buildings = buildings::table
            .select(Building::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(&self, connection: &mut DbConn, id: &building::PK) -> EmpResult<Building> {
        let building = buildings::table.find(&id).first(connection)?;
        Ok(building)
    }

    fn create(
        &mut self,
        connection: &mut DbConn,
        entity: &NewBuilding<'static>,
    ) -> EmpResult<Building> {
        let building = diesel::insert_into(buildings::table)
            .values(entity)
            .returning(Building::as_returning())
            .get_result(connection)?;
        Ok(building)
    }

    fn update(&mut self, connection: &mut DbConn, entity: &Building) -> EmpResult<Building> {
        let building = diesel::update(buildings::table.find(entity.id))
            .set(entity)
            .get_result(connection)?;
        Ok(building)
    }

    fn delete(&mut self, connection: &mut DbConn, id: &building::PK) -> EmpResult<usize> {
        let res = diesel::delete(buildings::table.find(&id)).execute(connection)?;
        Ok(res)
    }
}
