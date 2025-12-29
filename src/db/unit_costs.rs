//! Database access layer for unit cost entities.
//!
//! This module provides operations for retrieving unit training costs,
//! which represent the resource requirements for training units.

use std::collections::HashMap;

use diesel::prelude::*;
use tracing::instrument;

use crate::Result;
use crate::db::DbConn;
use crate::domain::unit::UnitKey;
use crate::domain::unit::cost::{NewUnitCost, UnitCost};
use crate::schema::unit_cost as uc;

/// Retrieves all costs for a specific unit.
#[instrument(skip(conn))]
pub fn get_by_unit(conn: &mut DbConn, unit_key: &UnitKey) -> Result<Vec<UnitCost>> {
	let costs = uc::table
		.filter(uc::unit_id.eq(unit_key))
		.select(UnitCost::as_select())
		.load(conn)?;
	Ok(costs)
}

// Retrieves all matching units by ID.
#[instrument(skip(conn))]
pub fn get_all_by_unit(conn: &mut DbConn, unit_ids: &[UnitKey]) -> Result<Vec<UnitCost>> {
	let result = uc::table
		.filter(uc::unit_id.eq_any(unit_ids))
		.get_results(conn)?;
	Ok(result)
}

/// Retrieves all unit costs grouped by unit ID.
///
/// Returns a HashMap where keys are unit IDs and values are vectors of costs.
#[instrument(skip(conn))]
pub fn get_all_costs(conn: &mut DbConn) -> Result<HashMap<UnitKey, Vec<UnitCost>>> {
	let costs_raw = uc::table.select(UnitCost::as_select()).load(conn)?;

	let costs: HashMap<UnitKey, Vec<UnitCost>> =
		costs_raw.into_iter().fold(HashMap::new(), |mut map, cost| {
			map.entry(cost.unit_id).or_default().push(cost);
			map
		});

	Ok(costs)
}

/// Creates a new unit cost in the database.
#[instrument(skip(conn, entity))]
pub fn create(conn: &mut DbConn, entity: NewUnitCost) -> Result<UnitCost> {
	let cost = diesel::insert_into(uc::table)
		.values(entity)
		.returning(UnitCost::as_returning())
		.get_result(conn)?;
	Ok(cost)
}
