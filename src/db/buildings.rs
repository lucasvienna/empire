use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::domain::building::{Building, NewBuilding, UpdateBuilding, PK as BuildingKey};
use crate::domain::error::Result;
use crate::schema::buildings::dsl::*;

#[derive(Debug)]
pub struct BuildingRepository {}

impl Repository<Building, NewBuilding, UpdateBuilding, BuildingKey> for BuildingRepository {
    fn get_all(&self, conn: &mut DbConn) -> Result<Vec<Building>> {
        let bld_list = buildings.select(Building::as_select()).load(conn)?;
        Ok(bld_list)
    }

    fn get_by_id(&self, conn: &mut DbConn, bld_id: &BuildingKey) -> Result<Building> {
        let building = buildings.find(bld_id).first(conn)?;
        Ok(building)
    }

    fn create(&self, conn: &mut DbConn, entity: NewBuilding) -> Result<Building> {
        let building = diesel::insert_into(buildings)
            .values(entity)
            .returning(Building::as_returning())
            .get_result(conn)?;
        Ok(building)
    }

    fn update(
        &self,
        conn: &mut DbConn,
        bld_id: &BuildingKey,
        changeset: UpdateBuilding,
    ) -> Result<Building> {
        let building = diesel::update(buildings.find(bld_id))
            .set(changeset)
            .get_result(conn)?;
        Ok(building)
    }

    fn delete(&self, conn: &mut DbConn, bld_id: &BuildingKey) -> Result<usize> {
        let res = diesel::delete(buildings.find(bld_id)).execute(conn)?;
        Ok(res)
    }
}
