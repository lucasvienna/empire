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
        log::debug!("Creating building level {:?}", entity);
        let building = diesel::insert_into(building_levels::table)
            .values(entity)
            .returning(BuildingLevel::as_returning())
            .get_result(connection)?;
        log::debug!("Created building level: {:?}", building);
        Ok(building)
    }

    fn update(
        &mut self,
        connection: &mut DbConn,
        entity: &BuildingLevel,
    ) -> EmpResult<BuildingLevel> {
        log::debug!("Updating building level {}", entity.id);
        let building = diesel::update(building_levels::table.find(entity.id))
            .set(entity)
            .get_result(connection)?;
        log::debug!("Updated building level: {:?}", building);
        Ok(building)
    }

    fn delete(&mut self, connection: &mut DbConn, id: &building_level::PK) -> EmpResult<usize> {
        log::debug!("Deleting building level {}", id);
        let res = diesel::delete(building_levels::table.find(id)).execute(connection)?;
        log::debug!("Deleted {} building levels", res);
        Ok(res)
    }
}

impl BuildingLevelRepository {
    pub fn get_by_building_id(
        &self,
        connection: &mut DbConn,
        building_id: &building::PK,
    ) -> EmpResult<Vec<BuildingLevel>> {
        log::debug!("Getting levels for building {}", building_id);
        let buildings = building_levels::table
            .filter(building_levels::building_id.eq(building_id))
            .order(building_levels::level.asc())
            .load(connection)?;
        log::debug!("Levels: {:?}", buildings);
        Ok(buildings)
    }
    pub fn get_next_upgrade(
        &self,
        connection: &mut DbConn,
        building_id: &building::PK,
        level: &i32,
    ) -> EmpResult<BuildingLevel> {
        log::debug!(
            "Getting next upgrade for building {} at level {}",
            building_id,
            level
        );
        let building = building_levels::table
            .filter(building_levels::building_id.eq(building_id))
            .filter(building_levels::level.eq(level + 1))
            .first(connection)?;
        log::debug!("Next upgrade: {:?}", building);
        Ok(building)
    }
}
