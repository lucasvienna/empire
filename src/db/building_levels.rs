//! Database access layer for building level entities.
//!
//! This module manages building levels, which represent different upgrade tiers
//! for buildings in the game. It provides standard CRUD operations along with
//! specialized queries for retrieving levels by building ID and finding next
//! available upgrades. All operations include debug logging for troubleshooting.

use diesel::prelude::*;
use tracing::debug;

use crate::db::DbConn;
use crate::domain::building::BuildingKey;
use crate::domain::building::level::{
	BuildingLevel, BuildingLevelKey, NewBuildingLevel, UpdateBuildingLevel,
};
use crate::domain::error::Result;
// can't glob import the dsl because it conflicts with the debug! macro
use crate::schema::building_level as bl;

/// Retrieves all building levels from the database.
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Returns
/// A [`Result`] containing a vector of [`BuildingLevel`] entities if successful,
/// or an error if the database operation fails.
pub fn get_all(conn: &mut DbConn) -> Result<Vec<BuildingLevel>> {
	let buildings = bl::table.select(BuildingLevel::as_select()).load(conn)?;
	Ok(buildings)
}

/// Retrieves a specific building level by its ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `lvl_id` - The unique identifier of the building level to retrieve
///
/// # Returns
/// A [`Result`] containing the requested [`BuildingLevel`] if found,
/// or an error if the level doesn't exist or the operation fails.
pub fn get_by_id(conn: &mut DbConn, lvl_id: &BuildingLevelKey) -> Result<BuildingLevel> {
	let building = bl::table.find(lvl_id).first(conn)?;
	Ok(building)
}

/// Creates a new building level in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `entity` - The [`NewBuildingLevel`] entity to create
///
/// # Returns
/// A [`Result`] containing the newly created [`BuildingLevel`] if successful,
/// or an error if the database operation fails.
pub fn create(conn: &mut DbConn, entity: NewBuildingLevel) -> Result<BuildingLevel> {
	debug!("Creating building level {:?}", entity);
	let building = diesel::insert_into(bl::table)
		.values(entity)
		.returning(BuildingLevel::as_returning())
		.get_result(conn)?;
	debug!("Created building level: {:?}", building);
	Ok(building)
}

/// Updates an existing building level in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `changeset` - The [`UpdateBuildingLevel`] containing the changes to apply
///
/// # Returns
/// A [`Result`] containing the updated [`BuildingLevel`] if successful,
/// or an error if the level doesn't exist or the operation fails.
pub fn update(conn: &mut DbConn, changeset: &UpdateBuildingLevel) -> Result<BuildingLevel> {
	debug!("Updating building level {}", changeset.id);
	let bld_level = diesel::update(bl::table).set(changeset).get_result(conn)?;
	debug!("Updated building level: {:?}", bld_level);
	Ok(bld_level)
}

/// Deletes a building level from the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `lvl_id` - The unique identifier of the building level to delete
///
/// # Returns
/// A [`Result`] containing the number of deleted records if successful,
/// or an error if the operation fails.
pub fn delete(conn: &mut DbConn, lvl_id: &BuildingLevelKey) -> Result<usize> {
	debug!("Deleting building level {}", lvl_id);
	let deleted_count = diesel::delete(bl::table.find(lvl_id)).execute(conn)?;
	debug!("Deleted {} building levels", deleted_count);
	Ok(deleted_count)
}

/// Retrieves all building levels for a specific building.
///
/// # Arguments
/// * `conn` - Database connection
/// * `bld_id` - The unique identifier of the building
///
/// # Returns
/// A Result containing a vector of BuildingLevel entities ordered by level
pub fn get_by_building_id(conn: &mut DbConn, bld_id: &BuildingKey) -> Result<Vec<BuildingLevel>> {
	debug!("Getting levels for building {}", bld_id);
	let bld_levels = bl::table
		.filter(bl::building_id.eq(bld_id))
		.order(bl::level.asc())
		.load(conn)?;
	debug!("Levels: {:?}", bld_levels);
	Ok(bld_levels)
}

/// Retrieves a specific building level.
///
/// # Arguments
/// * `conn` - Database connection
/// * `bld_id` - The unique identifier of the building
/// * `bld_level` - The level of the building to retrieve
///
/// # Returns
/// A Result containing a vector of BuildingLevel entities ordered by level
pub fn get_by_bld_and_level(
	conn: &mut DbConn,
	bld_id: &BuildingKey,
	bld_level: i32,
) -> Result<BuildingLevel> {
	debug!("Getting level {} for building {}", bld_level, bld_id,);
	let bld_level = bl::table
		.filter(bl::building_id.eq(bld_id).and(bl::level.eq(bld_level)))
		.select(BuildingLevel::as_select())
		.get_result(conn)?;
	debug!("Level: {:?}", bld_level);
	Ok(bld_level)
}

/// Retrieves the next upgrade level for a building.
///
/// # Arguments
/// * `conn` - Database connection
/// * `bld_id` - The unique identifier of the building
/// * `bld_level` - The current level of the building
///
/// # Returns
/// A Result containing the next BuildingLevel upgrade if available
pub fn get_next_upgrade(
	conn: &mut DbConn,
	bld_id: &BuildingKey,
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
		.first(conn)?;
	debug!("Next upgrade: {:?}", building);
	Ok(building)
}
