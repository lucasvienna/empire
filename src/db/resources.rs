//! Database access layer for player resource entities.
//!
//! This module provides comprehensive CRUD operations for player resource management,
//! including standard database operations and specialized functionality for resource
//! deduction and player-specific queries.

use diesel::prelude::*;
use tracing::{debug, info, instrument, trace};

use crate::Result;
use crate::db::DbConn;
use crate::domain::player::PlayerKey;
use crate::domain::player::resource::{
	NewPlayerResource, PlayerResource, PlayerResourceKey, UpdatePlayerResource,
};
use crate::schema::player_resource::dsl::*;

/// Retrieves all player resources from the database.
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Returns
/// A Result containing a vector of all [`PlayerResource`] entities
#[instrument(skip(conn))]
pub fn get_all(conn: &mut DbConn) -> Result<Vec<PlayerResource>> {
	debug!("Starting get all resources");
	let resources = player_resource
		.select(PlayerResource::as_select())
		.load(conn)?;
	info!("Completed get all resources, count: {}", resources.len());
	Ok(resources)
}

/// Retrieves a single player resource by its ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_key` - The unique identifier of the player resource
///
/// # Returns
/// A Result containing the requested [`PlayerResource`] entity
#[instrument(skip(conn))]
pub fn get_by_id(conn: &mut DbConn, player_key: &PlayerResourceKey) -> Result<PlayerResource> {
	debug!("Starting get resource by ID: {}", player_key);
	let resource = player_resource.find(player_key).first(conn)?;
	trace!("Got resource details: {:?}", resource);
	info!("Completed get resource for ID: {}", player_key);
	Ok(resource)
}

/// Creates a new player resource in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `entity` - The [`NewPlayerResource`] entity to create
///
/// # Returns
/// A Result containing the created [`PlayerResource`] entity
#[instrument(skip(conn, entity))]
pub fn create(conn: &mut DbConn, entity: NewPlayerResource) -> Result<PlayerResource> {
	debug!("Starting create resource for player: {}", entity.player_id);
	let resource = diesel::insert_into(player_resource)
		.values(entity)
		.returning(PlayerResource::as_returning())
		.get_result(conn)?;
	trace!("Created resource details: {:?}", resource);
	info!(
		"Completed create resource for player: {}",
		resource.player_id
	);
	Ok(resource)
}

/// Updates an existing player resource in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `changeset` - The [`UpdatePlayerResource`] containing the changes
///
/// # Returns
/// A Result containing the updated [`PlayerResource`] entity
#[instrument(skip(conn, changeset))]
pub fn update(conn: &mut DbConn, changeset: &UpdatePlayerResource) -> Result<PlayerResource> {
	debug!(
		"Starting update resource for player: {}",
		changeset.player_id
	);
	let resource = diesel::update(player_resource)
		.set(changeset)
		.returning(PlayerResource::as_returning())
		.get_result(conn)?;
	trace!("Updated resource details: {:?}", resource);
	info!(
		"Completed update resource for player: {}",
		resource.player_id
	);
	Ok(resource)
}

/// Deletes a player resource from the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `resource_key` - The unique identifier of the player resource to delete
///
/// # Returns
/// A Result containing the number of deleted records
#[instrument(skip(conn))]
pub fn delete(conn: &mut DbConn, resource_key: &PlayerResourceKey) -> Result<usize> {
	debug!("Starting delete resource: {}", resource_key);
	let count = diesel::delete(player_resource.find(resource_key)).execute(conn)?;
	info!(
		"Completed delete resource: {}, count: {}",
		resource_key, count
	);
	Ok(count)
}

/// Represents resource amounts to be deducted from a player's resources.
/// The tuple contains amounts in the following order:
/// * food (i64)
/// * wood (i64)
/// * stone (i64)
/// * gold (i64)
pub type Deduction = (i64, i64, i64, i64);

/// Retrieves resource information for a specific player by their player ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_key` - The unique identifier of the player
///
/// # Returns
/// A Result containing the [`PlayerResource`] information if found
#[instrument(skip(conn))]
pub fn get_by_player_id(conn: &mut DbConn, player_key: &PlayerKey) -> Result<PlayerResource> {
	debug!("Getting player resources");
	let res = player_resource
		.select(PlayerResource::as_select())
		.filter(player_id.eq(player_key))
		.first(conn)?;
	trace!("Fetched player resource details: {:?}", res);
	Ok(res)
}

/// Deducts specified amounts of resources from a player's resource pool.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_key` - The unique identifier of the player
/// * `amounts` - The amounts to deduct as a tuple of (food, wood, stone, gold)
///
/// # Returns
/// A Result containing the updated [`PlayerResource`] after deduction
#[instrument(skip(conn))]
pub fn deduct(
	conn: &mut DbConn,
	player_key: &PlayerKey,
	amounts: &Deduction,
) -> Result<PlayerResource> {
	debug!(
		"Starting deduct resources from player {}: food={}, wood={}, stone={}, gold={}",
		player_key, amounts.0, amounts.1, amounts.2, amounts.3
	);
	let res: PlayerResource = player_resource
		.filter(player_id.eq(player_key))
		.first(conn)?;
	trace!("Current resources before deduction: {:?}", res);
	let updated_res = diesel::update(player_resource.filter(player_id.eq(player_key)))
		.set((
			food.eq(food - amounts.0),
			wood.eq(wood - amounts.1),
			stone.eq(stone - amounts.2),
			gold.eq(gold - amounts.3),
		))
		.returning(PlayerResource::as_returning())
		.get_result(conn)?;
	trace!("Updated resources after deduction: {:?}", updated_res);
	info!(
		"Completed deduct resources from player {}: food={}, wood={}, stone={}, gold={}",
		player_key, amounts.0, amounts.1, amounts.2, amounts.3
	);
	Ok(updated_res)
}
