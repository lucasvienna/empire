//! Database access layer for building requirement entities.
//!
//! This module manages building requirements, which represent different upgrade
//! pre-requisites for buildings in the game. It provides standard CRUD
//! operations along with specialized queries for retrieving reqs by building
//! ID and finding next upgrade prerequisites. All operations include debug
//! logging for troubleshooting.

use std::collections::HashMap;

use diesel::prelude::*;
use tracing::{debug, info, trace};

use crate::db::{DbConn, building_levels as bld_level};
use crate::domain::building::BuildingKey;
use crate::domain::building::level::BuildingLevelKey;
use crate::domain::building::requirement::{
	BuildingRequirement, BuildingRequirementKey, NewBuildingRequirement, UpdateBuildingRequirement,
};
use crate::domain::error::Result;
use crate::domain::factions::{FactionCode, FactionKey};
use crate::schema::building_requirement as br;
use crate::schema::building_requirement::dsl::building_requirement;

/// Retrieves all building requirements from the database.
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Returns
/// A [`Result`] containing a vector of [`BuildingRequirement`] entities if successful,
/// or an error if the database operation fails.
pub fn get_all(conn: &mut DbConn) -> Result<Vec<BuildingRequirement>> {
	let requirements = br::table
		.select(BuildingRequirement::as_select())
		.load(conn)?;
	Ok(requirements)
}

/// Retrieves a specific building requirement by its ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `req_id` - The unique identifier of the building requirement to retrieve
///
/// # Returns
/// A [`Result`] containing the requested [`BuildingRequirement`] if found,
/// or an error if the requirement doesn't exist or the operation fails.
pub fn get_by_id(
	conn: &mut DbConn,
	req_id: &BuildingRequirementKey,
) -> Result<BuildingRequirement> {
	let requirement = br::table.find(req_id).first(conn)?;
	Ok(requirement)
}

/// Creates a new building requirement in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `entity` - The [`NewBuildingRequirement`] entity to create
///
/// # Returns
/// A [`Result`] containing the newly created [`BuildingRequirement`] if successful,
/// or an error if the database operation fails.
pub fn create(conn: &mut DbConn, entity: NewBuildingRequirement) -> Result<BuildingRequirement> {
	debug!("Creating building requirement {:?}", entity);
	let requirement = diesel::insert_into(br::table)
		.values(entity)
		.returning(BuildingRequirement::as_returning())
		.get_result(conn)?;
	info!("Created building requirement: {:?}", requirement);
	Ok(requirement)
}

/// Updates an existing building requirement in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `changeset` - The [`UpdateBuildingRequirement`] containing the changes to apply
///
/// # Returns
/// A [`Result`] containing the updated [`BuildingRequirement`] if successful,
/// or an error if the requirement doesn't exist or the operation fails.
pub fn update(
	conn: &mut DbConn,
	changeset: &UpdateBuildingRequirement,
) -> Result<BuildingRequirement> {
	debug!("Updating building requirement {}", changeset.id);
	let requirement = diesel::update(br::table).set(changeset).get_result(conn)?;
	info!("Updated building requirement: {:?}", requirement);
	Ok(requirement)
}

/// Deletes a building requirement from the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `req_id` - The unique identifier of the building requirement to delete
///
/// # Returns
/// A [`Result`] containing the number of deleted records if successful,
/// or an error if the operation fails.
pub fn delete(conn: &mut DbConn, req_id: &BuildingRequirementKey) -> Result<usize> {
	debug!("Deleting building requirement {}", req_id);
	let deleted_count = diesel::delete(br::table.find(req_id)).execute(conn)?;
	info!("Deleted {} building requirements", deleted_count);
	Ok(deleted_count)
}

/// Retrieves all building requirements for a specific building level
///
/// # Arguments
///
/// * `conn` - Database connection
/// * `bld_level_id` - Building level identifier to get requirements for
///
/// # Returns
///
/// Returns a Result containing a vector of BuildingRequirement if successful,
/// or an error if the database query fails
pub fn get_for_level(
	conn: &mut DbConn,
	bld_level_id: &BuildingLevelKey,
) -> Result<Vec<BuildingRequirement>> {
	debug!("Getting requirements for building level {}", bld_level_id);
	let reqs = br::table
		.filter(br::building_level_id.eq(bld_level_id))
		.select(BuildingRequirement::as_select())
		.load(conn)?;
	trace!("Requirements: {:?}", reqs);
	Ok(reqs)
}

/// Retrieves all building requirements for a specific building and level
///
/// # Arguments
///
/// * `conn` - Database connection
/// * `bld_id` - Building identifier to get requirements for
/// * `bld_level` - Level of the building to get requirements for
///
/// # Returns
///
/// Returns a Result containing a vector of BuildingRequirement if successful,
/// or an error if the database query fails
pub fn get_for_bld_and_level(
	conn: &mut DbConn,
	bld_id: &BuildingKey,
	bld_level: i32,
) -> Result<Vec<BuildingRequirement>> {
	debug!(
		"Getting requirements for building {} at level {}",
		bld_id, bld_level
	);
	let bld_level = bld_level::get_by_bld_and_level(conn, bld_id, bld_level)?;
	let reqs = get_for_level(conn, &bld_level.id)?;
	Ok(reqs)
}

/// Retrieves all construction requirements for buildings available to a specific faction.
///
/// This function fetches the initial (level 1) building requirements for all buildings
/// that are either specific to the given faction or are neutral (available to all factions).
/// The requirements are organized in a HashMap where each building ID maps to a vector of
/// its requirements.
///
/// # Arguments
/// * `conn` - Database connection
/// * `faction_key` - The faction key to get requirements for
///
/// # Returns
/// A [`Result`] containing a [`HashMap`] where:
/// * Keys are [`BuildingKey`]s representing available buildings
/// * Values are vectors of [`BuildingRequirement`]s for each building
pub fn get_construction_reqs(
	conn: &mut DbConn,
	faction_key: &FactionKey,
) -> Result<HashMap<BuildingKey, Vec<BuildingRequirement>>> {
	use crate::schema::{building as bld, building_level as bld_level};

	let requirements_raw = bld::table
		.filter(
			bld::faction
				.eq(faction_key)
				.or(bld::faction.eq(FactionCode::Neutral)),
		)
		.inner_join(bld_level::table.inner_join(building_requirement))
		.filter(bld_level::level.eq(1))
		.select((bld::id, BuildingRequirement::as_select()))
		.load::<(BuildingKey, BuildingRequirement)>(conn)?;

	let requirements: HashMap<BuildingKey, Vec<BuildingRequirement>> = requirements_raw
		.into_iter()
		.fold(HashMap::new(), |mut map, (building_id, req)| {
			map.entry(building_id).or_default().push(req);
			map
		});

	Ok(requirements)
}
