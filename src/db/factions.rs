//! Database access layer for faction entities.
//!
//! This module provides comprehensive CRUD operations for faction management,
//! including standard database operations (create, read, update, delete) and
//! specialized queries for retrieving faction-specific bonuses and modifiers.

use diesel::prelude::*;

use crate::db::DbConn;
use crate::domain::error::Result;
use crate::domain::factions::{Faction, FactionKey, NewFaction, UpdateFaction};
use crate::domain::modifier::Modifier;
use crate::schema::faction::dsl::*;

/// Retrieves all [`Faction`] entities from the database.
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Returns
/// A Result containing a vector of all faction entities
pub fn get_all(conn: &mut DbConn) -> Result<Vec<Faction>> {
	let fac_list = faction.select(Faction::as_select()).load(conn)?;
	Ok(fac_list)
}

/// Retrieves a single [`Faction`] by its ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `faction_id` - The unique identifier of the faction to retrieve
///
/// # Returns
/// A Result containing the requested faction
pub fn get_by_id(conn: &mut DbConn, faction_id: &FactionKey) -> Result<Faction> {
	let fac = faction.find(faction_id).first(conn)?;
	Ok(fac)
}

/// Creates a new [`Faction`] in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `entity` - The new faction entity to create
///
/// # Returns
/// A Result containing the created faction
pub fn create(conn: &mut DbConn, entity: NewFaction) -> Result<Faction> {
	let created_faction = diesel::insert_into(faction)
		.values(entity)
		.returning(Faction::as_returning())
		.get_result(conn)?;
	Ok(created_faction)
}

/// Updates an existing [`Faction`] in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `changeset` - The changes to apply to the faction
///
/// # Returns
/// A Result containing the updated faction
pub fn update(conn: &mut DbConn, changeset: &UpdateFaction) -> Result<Faction> {
	let updated_faction = diesel::update(faction).set(changeset).get_result(conn)?;
	Ok(updated_faction)
}

/// Deletes a [`Faction`] from the database by its ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `faction_id` - The unique identifier of the faction to delete
///
/// # Returns
/// A Result containing the number of rows deleted
pub fn delete(conn: &mut DbConn, faction_id: &FactionKey) -> Result<usize> {
	let rows_deleted = diesel::delete(faction.find(faction_id)).execute(conn)?;
	Ok(rows_deleted)
}

/// Retrieves faction-specific bonuses (modifiers) from the database.
///
/// # Arguments
/// * `faction_key` - Optional faction identifier to filter bonuses for a specific faction
pub fn get_bonuses(conn: &mut DbConn, faction_key: Option<&FactionKey>) -> Result<Vec<Modifier>> {
	let bonuses = {
		use crate::schema::modifiers as md;
		let mut query = md::table
			.select(Modifier::as_select())
			.filter(md::stacking_group.similar_to("faction_%"))
			.into_boxed();

		if let Some(faction_key) = faction_key {
			let ilike = format!("{faction_key}_%");
			query = query.filter(md::name.ilike(ilike));
		}

		query.get_results(conn)?
	};

	Ok(bonuses)
}
