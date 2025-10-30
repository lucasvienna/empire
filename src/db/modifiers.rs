//! Database access layer for modifier entities.
//!
//! This module provides comprehensive CRUD operations for game modifiers,
//! which represent various effects, bonuses, or penalties that can be applied
//! to game entities.

use diesel::prelude::*;

use crate::Result;
use crate::db::DbConn;
use crate::domain::modifier::{Modifier, ModifierKey, NewModifier, UpdateModifier};
use crate::schema::modifiers::dsl::*;

/// Retrieves all modifiers from the database.
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Returns
/// A Result containing a vector of [`Modifier`] entities
pub fn get_all(conn: &mut DbConn) -> Result<Vec<Modifier>> {
	let mod_list = modifiers.select(Modifier::as_select()).load(conn)?;
	Ok(mod_list)
}

/// Retrieves a single modifier by its ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `modifier_id` - The unique identifier of the modifier
///
/// # Returns
/// A Result containing the found [`Modifier`]
pub fn get_by_id(conn: &mut DbConn, modifier_id: &ModifierKey) -> Result<Modifier> {
	let modifier = modifiers.find(modifier_id).first(conn)?;
	Ok(modifier)
}

/// Creates a new modifier in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `entity` - The [`NewModifier`] to create
///
/// # Returns
/// A Result containing the created [`Modifier`]
pub fn create(conn: &mut DbConn, entity: NewModifier) -> Result<Modifier> {
	let modifier = diesel::insert_into(modifiers)
		.values(entity)
		.returning(Modifier::as_returning())
		.get_result(conn)?;
	Ok(modifier)
}

/// Updates an existing modifier in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `changeset` - The [`UpdateModifier`] containing the changes
///
/// # Returns
/// A Result containing the updated [`Modifier`]
pub fn update(conn: &mut DbConn, changeset: &UpdateModifier) -> Result<Modifier> {
	let modifier = diesel::update(modifiers).set(changeset).get_result(conn)?;
	Ok(modifier)
}

/// Deletes a modifier from the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `modifier_id` - The unique identifier of the modifier to delete
///
/// # Returns
/// A Result containing the number of affected rows
pub fn delete(conn: &mut DbConn, modifier_id: &ModifierKey) -> Result<usize> {
	let deleted_count = diesel::delete(modifiers.find(modifier_id)).execute(conn)?;
	Ok(deleted_count)
}
