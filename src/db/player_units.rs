//! Database access layer for player unit ownership entities.
//!
//! This module provides operations for managing player unit quantities,
//! including retrieving owned units and updating quantities.

use diesel::prelude::*;
use diesel::upsert::excluded;
use tracing::{debug, instrument, trace};

use crate::Result;
use crate::db::DbConn;
use crate::domain::player::PlayerKey;
use crate::domain::unit::UnitKey;
use crate::domain::unit::player_unit::{NewPlayerUnit, PlayerUnit};
use crate::schema::player_unit as pu;

/// Retrieves all units owned by a player.
#[instrument(skip(conn))]
pub fn get_for_player(conn: &mut DbConn, player_key: &PlayerKey) -> Result<Vec<PlayerUnit>> {
	let units = pu::table
		.filter(pu::player_id.eq(player_key))
		.select(PlayerUnit::as_select())
		.load(conn)?;
	Ok(units)
}

/// Gets the quantity of a specific unit owned by a player.
///
/// Returns 0 if the player doesn't own any of this unit type.
#[instrument(skip(conn))]
pub fn get_player_unit_count(
	conn: &mut DbConn,
	player_key: &PlayerKey,
	unit_key: &UnitKey,
) -> Result<i32> {
	let count: Option<i32> = pu::table
		.filter(pu::player_id.eq(player_key))
		.filter(pu::unit_id.eq(unit_key))
		.select(pu::quantity)
		.first(conn)
		.optional()?;
	Ok(count.unwrap_or(0))
}

/// Updates the quantity of a specific unit for a player by a delta amount.
///
/// The delta can be positive (adding units) or negative (removing units).
#[instrument(skip(conn))]
pub fn update_quantity(
	conn: &mut DbConn,
	player_key: &PlayerKey,
	unit_key: &UnitKey,
	delta: i32,
) -> Result<PlayerUnit> {
	debug!(
		"Updating player {} unit {} quantity by {}",
		player_key, unit_key, delta
	);
	let updated = diesel::update(
		pu::table
			.filter(pu::player_id.eq(player_key))
			.filter(pu::unit_id.eq(unit_key)),
	)
	.set(pu::quantity.eq(pu::quantity + delta))
	.returning(PlayerUnit::as_returning())
	.get_result(conn)?;
	trace!("Updated player unit: {:?}", updated);
	Ok(updated)
}

/// Creates a new player unit entry or updates the quantity if it already exists.
///
/// Uses PostgreSQL's ON CONFLICT to upsert the record.
#[instrument(skip(conn, entity))]
pub fn create_or_update(conn: &mut DbConn, entity: NewPlayerUnit) -> Result<PlayerUnit> {
	debug!(
		"Upserting player {} unit {} with quantity {}",
		entity.player_id, entity.unit_id, entity.quantity
	);
	let result = diesel::insert_into(pu::table)
		.values(&entity)
		.on_conflict((pu::player_id, pu::unit_id))
		.do_update()
		.set(pu::quantity.eq(pu::quantity + excluded(pu::quantity)))
		.returning(PlayerUnit::as_returning())
		.get_result(conn)?;
	trace!("Upserted player unit: {:?}", result);
	Ok(result)
}

/// Creates a new player unit entry.
#[instrument(skip(conn, entity))]
pub fn create(conn: &mut DbConn, entity: NewPlayerUnit) -> Result<PlayerUnit> {
	let result = diesel::insert_into(pu::table)
		.values(entity)
		.returning(PlayerUnit::as_returning())
		.get_result(conn)?;
	Ok(result)
}

/// Adds units to a player's inventory.
///
/// Creates a new entry if the player doesn't own this unit type,
/// or adds to the existing quantity if they do.
#[instrument(skip(conn))]
pub fn add_units(
	conn: &mut DbConn,
	player_key: &PlayerKey,
	unit_key: &UnitKey,
	quantity: i32,
) -> Result<PlayerUnit> {
	debug!(
		"Adding {} units of {} to player {}",
		quantity, unit_key, player_key
	);
	let entity = NewPlayerUnit {
		player_id: *player_key,
		unit_id: *unit_key,
		quantity,
	};
	create_or_update(conn, entity)
}
