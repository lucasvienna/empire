use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::domain::building;
use crate::domain::building::{Building, NewBuilding};
use crate::domain::error::Result;
use crate::schema::buildings;

#[derive(Debug)]
pub struct BuildingRepository {}

impl Repository<Building, NewBuilding<'_>, building::PK> for BuildingRepository {
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<Building>> {
        let buildings = buildings::table
            .select(Building::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(&self, connection: &mut DbConn, id: &building::PK) -> Result<Building> {
        let building = buildings::table.find(&id).first(connection)?;
        Ok(building)
    }

    fn create(&self, connection: &mut DbConn, entity: &NewBuilding<'_>) -> Result<Building> {
        let building = diesel::insert_into(buildings::table)
            .values(entity)
            .returning(Building::as_returning())
            .get_result(connection)?;
        Ok(building)
    }

    fn update(&self, connection: &mut DbConn, entity: &Building) -> Result<Building> {
        let building = diesel::update(buildings::table.find(entity.id))
            .set(entity)
            .get_result(connection)?;
        Ok(building)
    }

    fn delete(&self, connection: &mut DbConn, id: &building::PK) -> Result<usize> {
        let res = diesel::delete(buildings::table.find(&id)).execute(connection)?;
        Ok(res)
    }
}
