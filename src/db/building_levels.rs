use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::models::{building, building_level};
use crate::models::building_level::{BuildingLevel, NewBuildingLevel};
use crate::models::error::EmpResult;
use crate::schema::building_levels;

pub struct BuildingLevelRepository {}

impl Repository<BuildingLevel, NewBuildingLevel<'_>, building_level::PK>
    for BuildingLevelRepository
{
    fn get_all(&self, connection: &mut DbConn) -> EmpResult<Vec<BuildingLevel>> {
        let buildings = building_levels::table
            .select(BuildingLevel::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(
        &self,
        connection: &mut DbConn,
        id: &building_level::PK,
    ) -> EmpResult<BuildingLevel> {
        let building = building_levels::table.find(id).first(connection)?;
        Ok(building)
    }

    fn create(
        &mut self,
        connection: &mut DbConn,
        entity: &NewBuildingLevel,
    ) -> EmpResult<BuildingLevel> {
        let building = diesel::insert_into(building_levels::table)
            .values(entity)
            .returning(BuildingLevel::as_returning())
            .get_result(connection)?;
        Ok(building)
    }

    fn update(
        &mut self,
        connection: &mut DbConn,
        entity: &BuildingLevel,
    ) -> EmpResult<BuildingLevel> {
        let building = diesel::update(building_levels::table.find(entity.id))
            .set(entity)
            .get_result(connection)?;
        Ok(building)
    }

    fn delete(&mut self, connection: &mut DbConn, id: &building_level::PK) -> EmpResult<()> {
        diesel::delete(building_levels::table.find(id)).execute(connection)?;
        Ok(())
    }
}

impl BuildingLevelRepository {
    pub fn get_by_building_id(
        &self,
        connection: &mut DbConn,
        building_id: &building::PK,
    ) -> EmpResult<Vec<BuildingLevel>> {
        let buildings = building_levels::table
            .filter(building_levels::building_id.eq(building_id))
            .order(building_levels::level.asc())
            .select(BuildingLevel::as_select())
            .load(connection)?;
        Ok(buildings)
    }
    pub fn get_next_upgrade(
        &self,
        connection: &mut DbConn,
        building_id: &building::PK,
        level: &i32,
    ) -> EmpResult<BuildingLevel> {
        let building = building_levels::table
            .filter(building_levels::building_id.eq(building_id))
            .filter(building_levels::level.eq(level + 1))
            .first(connection)?;
        Ok(building)
    }
}
