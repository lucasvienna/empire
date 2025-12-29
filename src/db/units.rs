//! Database access layer for unit entities.
//!
//! This module provides CRUD operations for unit definitions,
//! which represent the various military unit types that can be trained.

use diesel::prelude::*;
use tracing::instrument;

use crate::Result;
use crate::db::DbConn;
use crate::domain::unit::{NewUnit, Unit, UnitKey, UnitType, UpdateUnit};
use crate::schema::unit::dsl::*;

/// Retrieves all units from the database.
#[instrument(skip(conn))]
pub fn get_all(conn: &mut DbConn) -> Result<Vec<Unit>> {
	let unit_list = unit.select(Unit::as_select()).load(conn)?;
	Ok(unit_list)
}

/// Retrieves a single unit by its ID.
#[instrument(skip(conn))]
pub fn get_by_id(conn: &mut DbConn, unit_id: &UnitKey) -> Result<Unit> {
	let result = unit.find(unit_id).first(conn)?;
	Ok(result)
}

/// Retrieves all matching units by ID.
#[instrument(skip(conn))]
pub fn get_all_by_id(conn: &mut DbConn, unit_ids: &[UnitKey]) -> Result<Vec<Unit>> {
	let result = unit.filter(id.eq_any(unit_ids)).get_results(conn)?;
	Ok(result)
}

/// Retrieves all units of a specific type.
#[instrument(skip(conn))]
pub fn get_by_type(conn: &mut DbConn, unit_type_filter: &UnitType) -> Result<Vec<Unit>> {
	let unit_list = unit
		.filter(unit_type.eq(unit_type_filter))
		.select(Unit::as_select())
		.load(conn)?;
	Ok(unit_list)
}

/// Creates a new unit in the database.
#[instrument(skip(conn, entity))]
pub fn create(conn: &mut DbConn, entity: NewUnit) -> Result<Unit> {
	let result = diesel::insert_into(unit)
		.values(entity)
		.returning(Unit::as_returning())
		.get_result(conn)?;
	Ok(result)
}

/// Updates an existing unit in the database.
#[instrument(skip(conn, changeset))]
pub fn update(conn: &mut DbConn, changeset: &UpdateUnit) -> Result<Unit> {
	let result = diesel::update(unit).set(changeset).get_result(conn)?;
	Ok(result)
}

/// Deletes a unit from the database.
#[instrument(skip(conn))]
pub fn delete(conn: &mut DbConn, unit_id: &UnitKey) -> Result<usize> {
	let deleted_count = diesel::delete(unit.find(unit_id)).execute(conn)?;
	Ok(deleted_count)
}
