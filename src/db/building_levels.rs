use diesel::prelude::*;
use tracing::debug;

use crate::db::{DbConn, Repository};
use crate::domain::building_levels::{
    BuildingLevel, BuildingLevelKey, NewBuildingLevel, UpdateBuildingLevel,
};
use crate::domain::buildings;
use crate::domain::error::Result;
// can't glob import the dsl because it conflicts with the debug! macro
use crate::schema::building_level as bl;

#[derive(Debug)]
pub struct BuildingLevelRepository {}

impl Repository<BuildingLevel, NewBuildingLevel, &UpdateBuildingLevel, BuildingLevelKey>
    for BuildingLevelRepository
{
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<BuildingLevel>> {
        let buildings = bl::table
            .select(BuildingLevel::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(
        &self,
        connection: &mut DbConn,
        lvl_id: &BuildingLevelKey,
    ) -> Result<BuildingLevel> {
        let building = bl::table.find(lvl_id).first(connection)?;
        Ok(building)
    }

    fn create(&self, connection: &mut DbConn, entity: NewBuildingLevel) -> Result<BuildingLevel> {
        debug!("Creating building level {:?}", entity);
        let building = diesel::insert_into(bl::table)
            .values(entity)
            .returning(BuildingLevel::as_returning())
            .get_result(connection)?;
        debug!("Created building level: {:?}", building);
        Ok(building)
    }

    fn update(
        &self,
        connection: &mut DbConn,
        changeset: &UpdateBuildingLevel,
    ) -> Result<BuildingLevel> {
        debug!("Updating building level {}", changeset.id);
        let bld_level = diesel::update(bl::table)
            .set(changeset)
            .get_result(connection)?;
        debug!("Updated building level: {:?}", bld_level);
        Ok(bld_level)
    }

    fn delete(&self, connection: &mut DbConn, lvl_id: &BuildingLevelKey) -> Result<usize> {
        debug!("Deleting building level {}", lvl_id);
        let deleted_count = diesel::delete(bl::table.find(lvl_id)).execute(connection)?;
        debug!("Deleted {} building levels", deleted_count);
        Ok(deleted_count)
    }
}

impl BuildingLevelRepository {
    pub fn get_by_building_id(
        &self,
        connection: &mut DbConn,
        bld_id: &buildings::BuildingKey,
    ) -> Result<Vec<BuildingLevel>> {
        debug!("Getting levels for building {}", bld_id);
        let bld_levels = bl::table
            .filter(bl::building_id.eq(bld_id))
            .order(bl::level.asc())
            .load(connection)?;
        debug!("Levels: {:?}", bld_levels);
        Ok(bld_levels)
    }
    pub fn get_next_upgrade(
        &self,
        connection: &mut DbConn,
        bld_id: &buildings::BuildingKey,
        bld_level: &i32,
    ) -> Result<BuildingLevel> {
        debug!(
            "Getting next upgrade for building {} at level {}",
            bld_id, bld_level
        );
        let next_level: i32 = bld_level + 1;
        let building = bl::table
            .filter(bl::building_id.eq(bld_id))
            .filter(bl::level.eq(&next_level))
            .first(connection)?;
        debug!("Next upgrade: {:?}", building);
        Ok(building)
    }
}
