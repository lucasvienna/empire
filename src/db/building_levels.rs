use std::fmt;
use std::sync::Arc;

use diesel::prelude::*;
use tracing::debug;

use crate::db::{DbConn, Repository};
use crate::domain::app_state::AppPool;
use crate::domain::building::level::{
	BuildingLevel, BuildingLevelKey, NewBuildingLevel, UpdateBuildingLevel,
};
use crate::domain::building::BuildingKey;
use crate::domain::error::Result;
// can't glob import the dsl because it conflicts with the debug! macro
use crate::schema::building_level as bl;

/// Repository for managing active modifiers in the database.
///
/// Provides CRUD operations and functionality to manage active player modifiers,
/// including querying by player and handling modifier expirations.
///
/// # Fields
/// * `pool` - Thread-safe connection pool of type [`AppPool`] for database access
pub struct BuildingLevelRepository {
	pool: AppPool,
}

impl fmt::Debug for BuildingLevelRepository {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "BuildingLevelRepository")
	}
}

impl Repository<BuildingLevel, NewBuildingLevel, &UpdateBuildingLevel, BuildingLevelKey>
	for BuildingLevelRepository
{
	fn new(pool: &AppPool) -> Self {
		Self {
			pool: Arc::clone(pool),
		}
	}

	/// Retrieves all building levels from the database.
	///
	/// # Returns
	/// A [`Result`] containing a vector of [`BuildingLevel`] entities if successful,
	/// or an error if the database operation fails.
	fn get_all(&self) -> Result<Vec<BuildingLevel>> {
		let mut conn = self.pool.get()?;
		let buildings = bl::table
			.select(BuildingLevel::as_select())
			.load(&mut conn)?;
		Ok(buildings)
	}

	/// Retrieves a specific building level by its ID.
	///
	/// # Arguments
	/// * `lvl_id` - The unique identifier of the building level to retrieve
	///
	/// # Returns
	/// A [`Result`] containing the requested [`BuildingLevel`] if found,
	/// or an error if the level doesn't exist or the operation fails.
	fn get_by_id(&self, lvl_id: &BuildingLevelKey) -> Result<BuildingLevel> {
		let mut conn = self.pool.get()?;
		let building = bl::table.find(lvl_id).first(&mut conn)?;
		Ok(building)
	}

	/// Creates a new building level in the database.
	///
	/// # Arguments
	/// * `entity` - The [`NewBuildingLevel`] entity to create
	///
	/// # Returns
	/// A [`Result`] containing the newly created [`BuildingLevel`] if successful,
	/// or an error if the database operation fails.
	fn create(&self, entity: NewBuildingLevel) -> Result<BuildingLevel> {
		debug!("Creating building level {:?}", entity);
		let mut conn = self.pool.get()?;
		let building = diesel::insert_into(bl::table)
			.values(entity)
			.returning(BuildingLevel::as_returning())
			.get_result(&mut conn)?;
		debug!("Created building level: {:?}", building);
		Ok(building)
	}

	/// Updates an existing building level in the database.
	///
	/// # Arguments
	/// * `changeset` - The [`UpdateBuildingLevel`] containing the changes to apply
	///
	/// # Returns
	/// A [`Result`] containing the updated [`BuildingLevel`] if successful,
	/// or an error if the level doesn't exist or the operation fails.
	fn update(&self, changeset: &UpdateBuildingLevel) -> Result<BuildingLevel> {
		debug!("Updating building level {}", changeset.id);
		let mut conn = self.pool.get()?;
		let bld_level = diesel::update(bl::table)
			.set(changeset)
			.get_result(&mut conn)?;
		debug!("Updated building level: {:?}", bld_level);
		Ok(bld_level)
	}

	/// Deletes a building level from the database.
	///
	/// # Arguments
	/// * `lvl_id` - The unique identifier of the building level to delete
	///
	/// # Returns
	/// A [`Result`] containing the number of deleted records if successful,
	/// or an error if the operation fails.
	fn delete(&self, lvl_id: &BuildingLevelKey) -> Result<usize> {
		debug!("Deleting building level {}", lvl_id);
		let mut conn = self.pool.get()?;
		let deleted_count = diesel::delete(bl::table.find(lvl_id)).execute(&mut conn)?;
		debug!("Deleted {} building levels", deleted_count);
		Ok(deleted_count)
	}
}

impl BuildingLevelRepository {
	/// Retrieves all active modifiers for a specific player.
	///
	/// # Arguments
	/// * `player_id` - The unique identifier of the player
	///
	/// # Returns
	/// A Result containing a vector of ActiveModifier entities
	pub fn get_by_building_id(
		&self,
		conn: &mut DbConn,
		bld_id: &BuildingKey,
	) -> Result<Vec<BuildingLevel>> {
		debug!("Getting levels for building {}", bld_id);
		let bld_levels = bl::table
			.filter(bl::building_id.eq(bld_id))
			.order(bl::level.asc())
			.load(conn)?;
		debug!("Levels: {:?}", bld_levels);
		Ok(bld_levels)
	}

	pub fn get_next_upgrade(
		&self,
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
}
