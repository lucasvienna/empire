//! Database access layer for active modifier entities.
//!
//! This module manages active modifiers, which represent temporary or applied
//! modifications to game entities (typically players). It provides standard CRUD
//! operations along with specialized queries for retrieving modifiers by player ID.

use diesel::prelude::*;

use crate::Result;
use crate::db::DbConn;
use crate::domain::modifier::active_modifier::{
	ActiveModifier, ActiveModifierKey, NewActiveModifier, UpdateActiveModifier,
};
use crate::domain::player::PlayerKey;
use crate::schema::active_modifiers::dsl::*;

/// Retrieves all active modifiers from the database.
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Returns
/// * `Result<Vec<ActiveModifier>>` - List of all active modifiers or an error
pub fn get_all(conn: &mut DbConn) -> Result<Vec<ActiveModifier>> {
	let mod_list = active_modifiers
		.select(ActiveModifier::as_select())
		.load(conn)?;
	Ok(mod_list)
}

/// Retrieves a single active modifier by its ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `mod_id` - The [`ActiveModifierKey`] of the modifier to retrieve
///
/// # Returns
/// * `Result<ActiveModifier>` - The requested modifier or an error
pub fn get_by_id(conn: &mut DbConn, mod_id: &ActiveModifierKey) -> Result<ActiveModifier> {
	let modifier = active_modifiers.find(mod_id).first(conn)?;
	Ok(modifier)
}

/// Creates a new active modifier in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `entity` - The [`NewActiveModifier`] to create
///
/// # Returns
/// * `Result<ActiveModifier>` - The created modifier or an error
pub fn create(conn: &mut DbConn, entity: NewActiveModifier) -> Result<ActiveModifier> {
	let modifier = diesel::insert_into(active_modifiers)
		.values(entity)
		.returning(ActiveModifier::as_returning())
		.get_result(conn)?;
	Ok(modifier)
}

/// Updates an existing active modifier in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `changeset` - Reference to the [`UpdateActiveModifier`] containing the changes
///
/// # Returns
/// * `Result<ActiveModifier>` - The updated modifier or an error
pub fn update(conn: &mut DbConn, changeset: &UpdateActiveModifier) -> Result<ActiveModifier> {
	let modifier = diesel::update(active_modifiers)
		.set(changeset)
		.get_result(conn)?;
	Ok(modifier)
}

/// Deletes an active modifier from the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `mod_id` - The [`ActiveModifierKey`] of the modifier to delete
///
/// # Returns
/// * `Result<usize>` - The number of deleted rows or an error
pub fn delete(conn: &mut DbConn, mod_id: &ActiveModifierKey) -> Result<usize> {
	let deleted_count = diesel::delete(active_modifiers.find(mod_id)).execute(conn)?;
	Ok(deleted_count)
}

/// Retrieves all active modifiers for a specific player.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_key` - Reference to the [`PlayerKey`] to filter modifiers by
///
/// # Returns
/// * `Result<Vec<ActiveModifier>>` - List of active modifiers for the player or an error
pub fn get_by_player_id(conn: &mut DbConn, player_key: &PlayerKey) -> Result<Vec<ActiveModifier>> {
	let active_mods = active_modifiers
		.filter(player_id.eq(player_key))
		.get_results(conn)?;
	Ok(active_mods)
}
