//! Database access layer for building-unit type mappings.
//!
//! This module provides operations for querying which buildings
//! can train which unit types.

use diesel::prelude::*;
use tracing::instrument;

use crate::Result;
use crate::db::DbConn;
use crate::domain::building::BuildingKey;
use crate::domain::building::unit_type::{BuildingUnitType, NewBuildingUnitType};
use crate::domain::unit::UnitType;
use crate::schema::building_unit_type::dsl::*;

/// Retrieves all unit types that can be trained by a specific building.
#[instrument(skip(conn))]
pub fn get_unit_types_for_building(
	conn: &mut DbConn,
	bld_key: &BuildingKey,
) -> Result<Vec<UnitType>> {
	let types = building_unit_type
		.filter(building_id.eq(bld_key))
		.select(unit_type)
		.load(conn)?;
	Ok(types)
}

/// Checks if a building can train a specific unit type.
#[instrument(skip(conn))]
pub fn can_train_unit(conn: &mut DbConn, bld_key: &BuildingKey, utype: &UnitType) -> Result<bool> {
	let count: i64 = building_unit_type
		.filter(building_id.eq(bld_key))
		.filter(unit_type.eq(utype))
		.count()
		.get_result(conn)?;
	Ok(count > 0)
}

/// Retrieves all building IDs that can train a specific unit type.
#[instrument(skip(conn))]
pub fn get_buildings_for_unit_type(
	conn: &mut DbConn,
	utype: &UnitType,
) -> Result<Vec<BuildingKey>> {
	let building_ids = building_unit_type
		.filter(unit_type.eq(utype))
		.select(building_id)
		.load(conn)?;
	Ok(building_ids)
}

/// Retrieves all building-unit type mappings.
#[instrument(skip(conn))]
pub fn get_all(conn: &mut DbConn) -> Result<Vec<BuildingUnitType>> {
	let mappings = building_unit_type
		.select(BuildingUnitType::as_select())
		.load(conn)?;
	Ok(mappings)
}

/// Creates a new building-unit type mapping.
#[instrument(skip(conn, entity))]
pub fn create(conn: &mut DbConn, entity: NewBuildingUnitType) -> Result<BuildingUnitType> {
	let result = diesel::insert_into(building_unit_type)
		.values(entity)
		.returning(BuildingUnitType::as_returning())
		.get_result(conn)?;
	Ok(result)
}
