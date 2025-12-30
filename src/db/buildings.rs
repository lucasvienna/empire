//! Database access layer for building entities.
//!
//! This module provides comprehensive CRUD operations for building management,
//! including standard database operations for creating, reading, updating, and
//! deleting building records.

use diesel::prelude::*;

use crate::db::DbConn;
use crate::domain::building::level::BuildingLevel;
use crate::domain::building::resources::BuildingResource;
use crate::domain::building::{Building, BuildingKey, NewBuilding, UpdateBuilding};
use crate::domain::error::Result;
use crate::domain::factions::{FactionCode, FactionKey};
use crate::schema::building::dsl::*;

/// Retrieves all buildings from the database.
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Returns
/// * `Result<Vec<Building>>` - Vector of all [`Building`] entities
pub fn get_all(conn: &mut DbConn) -> Result<Vec<Building>> {
	let bld_list = building.select(Building::as_select()).load(conn)?;
	Ok(bld_list)
}

/// Retrieves a single building by its ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `bld_id` - Reference to [`BuildingKey`] identifying the building
///
/// # Returns
/// * `Result<Building>` - The requested [`Building`] entity
pub fn get_by_id(conn: &mut DbConn, bld_id: &BuildingKey) -> Result<Building> {
	let bld = building.find(bld_id).first(conn)?;
	Ok(bld)
}

/// Creates a new building in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `entity` - [`NewBuilding`] struct containing the building data
///
/// # Returns
/// * `Result<Building>` - The newly created [`Building`] entity
pub fn create(conn: &mut DbConn, entity: NewBuilding) -> Result<Building> {
	let created_building = diesel::insert_into(building)
		.values(entity)
		.returning(Building::as_returning())
		.get_result(conn)?;
	Ok(created_building)
}

/// Updates an existing building in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `changeset` - Reference to [`UpdateBuilding`] containing the changes
///
/// # Returns
/// * `Result<Building>` - The updated [`Building`] entity
pub fn update(conn: &mut DbConn, changeset: &UpdateBuilding) -> Result<Building> {
	let updated_building = diesel::update(building).set(changeset).get_result(conn)?;
	Ok(updated_building)
}

/// Deletes a building from the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `bld_id` - Reference to [`BuildingKey`] identifying the building to delete
///
/// # Returns
/// * `Result<usize>` - Number of deleted records
pub fn delete(conn: &mut DbConn, bld_id: &BuildingKey) -> Result<usize> {
	let deleted_count = diesel::delete(building.find(bld_id)).execute(conn)?;
	Ok(deleted_count)
}

/// Tuple containing building, level, and resource data for a specific level
pub type BuildingLevelData = (Building, BuildingLevel, BuildingResource);

/// Retrieves all buildings for a faction with their levels and resource data.
///
/// Returns buildings that match the specified faction OR are neutral.
/// Each building is joined with all its levels and corresponding resource data.
///
/// # Arguments
/// * `conn` - Database connection
/// * `faction_key` - The faction to filter buildings for
///
/// # Returns
/// * `Result<Vec<BuildingLevelData>>` - Vector of (Building, BuildingLevel, BuildingResource) tuples
///   ordered by building_id and level
pub fn get_faction_building_definitions(
	conn: &mut DbConn,
	faction_key: &FactionKey,
) -> Result<Vec<BuildingLevelData>> {
	use crate::schema::{building as bld, building_level as bl, building_resource as br};

	let results = bld::table
		.filter(
			bld::faction
				.eq(faction_key)
				.or(bld::faction.eq(FactionCode::Neutral)),
		)
		.inner_join(bl::table.on(bld::id.eq(bl::building_id)))
		.inner_join(
			br::table.on(bl::building_id
				.eq(br::building_id)
				.and(bl::level.eq(br::building_level))),
		)
		.select((
			Building::as_select(),
			BuildingLevel::as_select(),
			BuildingResource::as_select(),
		))
		.order((bld::id.asc(), bl::level.asc()))
		.load::<BuildingLevelData>(conn)?;

	Ok(results)
}
