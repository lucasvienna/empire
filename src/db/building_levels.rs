use diesel::prelude::*;
use tracing::debug;

use crate::db::{DbConn, Repository};
use crate::domain::building_level::{BuildingLevel, NewBuildingLevel, UpdateBuildingLevel};
use crate::domain::error::Result;
use crate::domain::{building, building_level};
use crate::schema::building_levels;

#[derive(Debug)]
pub struct BuildingLevelRepository {}

impl Repository<BuildingLevel, NewBuildingLevel, UpdateBuildingLevel, building_level::PK>
    for BuildingLevelRepository
{
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<BuildingLevel>> {
        let buildings = building_levels::table
            .select(BuildingLevel::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(&self, connection: &mut DbConn, id: &building_level::PK) -> Result<BuildingLevel> {
        let building = building_levels::table.find(id).first(connection)?;
        Ok(building)
    }

    fn create(&self, connection: &mut DbConn, entity: NewBuildingLevel) -> Result<BuildingLevel> {
        debug!("Creating building level {:?}", entity);
        let building = diesel::insert_into(building_levels::table)
            .values(entity)
            .returning(BuildingLevel::as_returning())
            .get_result(connection)?;
        debug!("Created building level: {:?}", building);
        Ok(building)
    }

    fn update(
        &self,
        connection: &mut DbConn,
        id: &building_level::PK,
        changeset: UpdateBuildingLevel,
    ) -> Result<BuildingLevel> {
        debug!("Updating building level {}", id);
        let building = diesel::update(building_levels::table.find(id))
            .set(changeset)
            .get_result(connection)?;
        debug!("Updated building level: {:?}", building);
        Ok(building)
    }

    fn delete(&self, connection: &mut DbConn, id: &building_level::PK) -> Result<usize> {
        debug!("Deleting building level {}", id);
        let res = diesel::delete(building_levels::table.find(id)).execute(connection)?;
        debug!("Deleted {} building levels", res);
        Ok(res)
    }
}

impl BuildingLevelRepository {
    pub fn get_by_building_id(
        &self,
        connection: &mut DbConn,
        building_id: &building::PK,
    ) -> Result<Vec<BuildingLevel>> {
        debug!("Getting levels for building {}", building_id);
        let buildings = building_levels::table
            .filter(building_levels::building_id.eq(building_id))
            .order(building_levels::level.asc())
            .load(connection)?;
        debug!("Levels: {:?}", buildings);
        Ok(buildings)
    }
    pub fn get_next_upgrade(
        &self,
        connection: &mut DbConn,
        building_id: &building::PK,
        level: &i32,
    ) -> Result<BuildingLevel> {
        debug!(
            "Getting next upgrade for building {} at level {}",
            building_id, level
        );
        let next_level: i32 = level + 1;
        let building = building_levels::table
            .filter(building_levels::building_id.eq(building_id))
            .filter(building_levels::level.eq(&next_level))
            .first(connection)?;
        debug!("Next upgrade: {:?}", building);
        Ok(building)
    }
}
