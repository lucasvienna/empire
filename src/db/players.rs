//! Database access layer for player entities.
//!
//! This module provides comprehensive CRUD operations for player management,
//! including standard database operations and specialized functionality for
//! finding players by name and checking existence.

use diesel::prelude::*;

use crate::db::DbConn;
use crate::domain::error::Result;
use crate::domain::player::{NewPlayer, Player, PlayerKey, UpdatePlayer};
use crate::schema::player::dsl::*;

/// Retrieves all players from the database.
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Returns
/// A Result containing a vector of all [`Player`] entities
pub fn get_all(conn: &mut DbConn) -> Result<Vec<Player>> {
	let player_list = player.select(Player::as_select()).load(conn)?;
	Ok(player_list)
}

/// Retrieves a single player by their ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_id` - The unique identifier of the player
///
/// # Returns
/// A Result containing the requested [`Player`] entity
pub fn get_by_id(conn: &mut DbConn, player_id: &PlayerKey) -> Result<Player> {
	let player_ = player.find(player_id).first(conn)?;
	Ok(player_)
}

/// Creates a new player in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `entity` - The [`NewPlayer`] entity to create
///
/// # Returns
/// A Result containing the created [`Player`] entity
pub fn create(conn: &mut DbConn, entity: NewPlayer) -> Result<Player> {
	let player_ = diesel::insert_into(player)
		.values(entity)
		.returning(Player::as_returning())
		.get_result(conn)?;
	Ok(player_)
}

/// Updates an existing player in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `changeset` - The [`UpdatePlayer`] containing the changes
///
/// # Returns
/// A Result containing the updated [`Player`] entity
pub fn update(conn: &mut DbConn, changeset: &UpdatePlayer) -> Result<Player> {
	let player_ = diesel::update(player).set(changeset).get_result(conn)?;
	Ok(player_)
}

/// Deletes a player from the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_id` - The unique identifier of the player to delete
///
/// # Returns
/// A Result containing the number of deleted records
pub fn delete(conn: &mut DbConn, player_id: &PlayerKey) -> Result<usize> {
	let deleted_count = diesel::delete(player.find(player_id)).execute(conn)?;
	Ok(deleted_count)
}

/// Attempts to find a player by their ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_id` - The unique identifier of the player
///
/// # Returns
/// * `Ok(Some(`[`Player`]`))` if the player is found
/// * `Ok(None)` if no player exists with the given ID
/// * `Err` if a database error occurs
pub fn find_by_id(conn: &mut DbConn, player_id: &PlayerKey) -> Result<Option<Player>> {
	let player_: Option<Player> = player.find(player_id).first(conn).optional()?;
	Ok(player_)
}

/// Retrieves a player by their name.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_name` - The name of the player to find
///
/// # Returns
/// * `Ok(`[`Player`]`)` if the player is found
/// * `Err` if no player exists with the given name or a database error occurs
pub fn get_by_name(conn: &mut DbConn, player_name: impl AsRef<str>) -> Result<Player> {
	let player_ = player.filter(name.eq(player_name.as_ref())).first(conn)?;
	Ok(player_)
}

/// Attempts to find a player by their name.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_name` - The name of the player to find
///
/// # Returns
/// * `Ok(Some(`[`Player`]`))` if the player is found
/// * `Ok(None)` if no player exists with the given name
/// * `Err` if a database error occurs
pub fn find_by_name(conn: &mut DbConn, player_name: impl AsRef<str>) -> Result<Option<Player>> {
	let player_: Option<Player> = player
		.filter(name.eq(player_name.as_ref()))
		.first(conn)
		.optional()?;
	Ok(player_)
}

/// Checks if a player with the given name exists.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_name` - The name to check for existence
///
/// # Returns
/// * `Ok(true)` if a player with the given name exists
/// * `Ok(false)` if no player exists with the given name
/// * `Err` if a database error occurs
pub fn exists_by_name(conn: &mut DbConn, player_name: impl AsRef<str>) -> Result<bool> {
	let player_ = find_by_name(conn, player_name)?;
	Ok(player_.is_some())
}
