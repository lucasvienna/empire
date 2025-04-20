use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::domain::buildings::{Building, BuildingKey, NewBuilding, UpdateBuilding};
use crate::domain::error::Result;
use crate::schema::building::dsl::*;

#[derive(Debug)]
pub struct BuildingRepository {}

impl Repository<Building, NewBuilding, &UpdateBuilding, BuildingKey> for BuildingRepository {
    fn get_all(&self, conn: &mut DbConn) -> Result<Vec<Building>> {
        let bld_list = building.select(Building::as_select()).load(conn)?;
        Ok(bld_list)
    }

    fn get_by_id(&self, conn: &mut DbConn, bld_id: &BuildingKey) -> Result<Building> {
        let bld = building.find(bld_id).first(conn)?;
        Ok(bld)
    }

    fn create(&self, conn: &mut DbConn, entity: NewBuilding) -> Result<Building> {
        let bld = diesel::insert_into(building)
            .values(entity)
            .returning(Building::as_returning())
            .get_result(conn)?;
        Ok(bld)
    }

    fn update(
        &self,
        conn: &mut DbConn,
        changeset: &UpdateBuilding,
    ) -> Result<Building> {
        let bld = diesel::update(building).set(changeset).get_result(conn)?;
        Ok(bld)
    }

    fn delete(&self, conn: &mut DbConn, bld_id: &BuildingKey) -> Result<usize> {
        let deleted_count = diesel::delete(building.find(bld_id)).execute(conn)?;
        Ok(deleted_count)
    }
}
