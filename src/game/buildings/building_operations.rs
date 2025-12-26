//! Building operations for the Empire game.
//!
//! This module provides core functionality for managing player buildings, including
//! construction, upgrades, and upgrade confirmation. It follows the functional programming
//! approach with direct function calls rather than service structs, enabling better
//! performance through single-connection-per-request optimization.
//!
//! All operations maintain transactional integrity and provide comprehensive validation
//! of resource requirements, building constraints, and timing requirements.

use std::ops::Add;

use chrono::TimeDelta;
use chrono::prelude::*;
use diesel::Connection;
use tracing::{debug, info, instrument, trace, warn};

use crate::db::{DbConn, building_levels, building_requirements, player_buildings, resources};
use crate::domain::building::BuildingKey;
use crate::domain::building::level::BuildingLevel;
use crate::domain::error::{Error, ErrorKind, Result};
use crate::domain::player::PlayerKey;
use crate::domain::player::buildings::{NewPlayerBuilding, PlayerBuilding, PlayerBuildingKey};
use crate::game::buildings::requirement_operations;

/// Constructs a new building for a player.
///
/// This function handles the complete building construction process, including resource
/// validation, constraint checking, and database operations. The construction is performed
/// within a database transaction to ensure atomicity.
///
/// # Arguments
///
/// * `conn` - Database connection for performing operations
/// * `player_id` - Unique identifier of the player constructing the building
/// * `bld_id` - Unique identifier of the building type to construct
///
/// # Returns
///
/// Returns the newly constructed `PlayerBuilding` on success, or an error if:
/// - Player lacks sufficient resources
/// - Maximum building count has been reached
/// - Database operation fails
///
/// # Errors
///
/// This function returns `ConstructBuildingError` variants for:
/// - Insufficient resources ("Not enough resources")
/// - Building count limit exceeded ("Max buildings reached")
/// - Transaction failure ("Failed to construct building")
#[instrument(skip(conn))]
pub fn construct_building(
	conn: &mut DbConn,
	player_id: &PlayerKey,
	bld_id: &BuildingKey,
) -> Result<PlayerBuilding> {
	debug!(
		"Starting construct building {} for player {}",
		bld_id, player_id
	);
	let bld_lvl = building_levels::get_next_upgrade(conn, bld_id, &0)?;
	trace!("Building level requirements: {:?}", bld_lvl);

	let reqs = building_requirements::get_for_bld_and_level(conn, bld_id, bld_lvl.building_level)?;
	let (bld, avail_data) = player_buildings::get_player_bld_count_level(conn, player_id, bld_id)?;
	let bld_avail = requirement_operations::gen_avail_data(bld, avail_data, reqs);
	trace!("Building availability: {:?}", bld_avail);

	if !bld_avail.buildable {
		trace!(
			"Cannot construct building, availability locks present: {:?}",
			bld_avail.locks
		);
		return Err(Error::from((
			ErrorKind::ConstructBuildingError,
			"Building has locks, cannot construct",
		)));
	}

	// check for resources
	if !has_enough_resources(conn, player_id, &bld_lvl)? {
		trace!(
			"Player {} doesn't have enough resources to build {}",
			player_id, bld_id
		);
		return Err(Error::from((
			ErrorKind::ConstructBuildingError,
			"Not enough resources",
		)));
	}

	let res: Result<PlayerBuilding> = conn.transaction(|connection| {
		info!("Initiating construction transaction");
		// deduct resources
		resources::deduct(
			connection,
			player_id,
			&(
				bld_lvl.req_food.unwrap_or(0),
				bld_lvl.req_wood.unwrap_or(0),
				bld_lvl.req_stone.unwrap_or(0),
				bld_lvl.req_gold.unwrap_or(0),
			),
		)?;
		trace!("Deducted resources");
		// construct building
		let upgrade_eta = Utc::now().add(TimeDelta::seconds(bld_lvl.upgrade_seconds));
		let player_bld = player_buildings::construct(
			connection,
			NewPlayerBuilding {
				player_id: *player_id,
				building_id: *bld_id,
				level: Some(0),
				upgrade_finishes_at: Some(upgrade_eta.to_rfc3339()),
			},
		)?;
		trace!("New player building details: {:#?}", player_bld);
		Ok(player_bld)
	});

	match res {
		Ok(player_bld) => {
			info!(
				"Successfully constructed building {} for player {}",
				bld_id, player_id
			);
			Ok(player_bld)
		}
		Err(e) => {
			warn!(
				"Failed to construct building {} for player {}: {}",
				bld_id, player_id, e
			);
			Err(Error::from((
				ErrorKind::ConstructBuildingError,
				"Failed to construct building",
			)))
		}
	}
}

/// Initiates an upgrade for an existing player building.
///
/// This function starts the upgrade process for a player's building by validating
/// requirements, deducting resources, and setting the upgrade timer. The upgrade
/// is performed within a database transaction to maintain consistency.
///
/// # Arguments
///
/// * `conn` - Database connection for performing operations
/// * `player_bld_id` - Unique identifier of the player building to upgrade
///
/// # Returns
///
/// Returns the updated `PlayerBuilding` with upgrade timing set, or an error if:
/// - Player lacks sufficient resources for the upgrade
/// - Building is already at maximum level
/// - Database operation fails
///
/// # Errors
///
/// This function returns `UpgradeBuildingError` variants for:
/// - Insufficient resources ("Not enough resources")
/// - Maximum level reached ("Building is at max level")
/// - Transaction failure ("Failed to upgrade building")
#[instrument(skip(conn))]
pub fn upgrade_building(
	conn: &mut DbConn,
	player_bld_id: &PlayerBuildingKey,
) -> Result<PlayerBuilding> {
	debug!("Starting upgrade building: {}", player_bld_id);
	let (player_bld, max_level) = player_buildings::get_upgrade_tuple(conn, player_bld_id)?;
	trace!(
		"Player building details: {:?}, max level: {:?}",
		player_bld, max_level
	);
	let bld_id = &player_bld.building_id;
	let player_id = &player_bld.player_id;

	let bld_lvl = building_levels::get_next_upgrade(conn, bld_id, &player_bld.level)?;
	trace!("Next building level details: {:?}", bld_lvl);

	let reqs = building_requirements::get_for_bld_and_level(conn, bld_id, bld_lvl.building_level)?;
	let (bld, avail_data) = player_buildings::get_player_bld_count_level(conn, player_id, bld_id)?;
	let bld_avail = requirement_operations::gen_avail_data(bld, avail_data, reqs);
	trace!("Building availability: {:?}", bld_avail);

	if !bld_avail.buildable {
		debug!(
			"Building {} cannot be upgraded, locks present: {:?}",
			player_bld.building_id, bld_avail.locks
		);
		return Err(Error::from((
			ErrorKind::UpgradeBuildingError,
			"Building has locks",
		)));
	}

	// check for resources
	if !has_enough_resources(conn, &player_bld.player_id, &bld_lvl)? {
		debug!(
			"Player {} doesn't have enough resources for upgrade",
			player_bld.player_id
		);
		return Err(Error::from((
			ErrorKind::UpgradeBuildingError,
			"Not enough resources",
		)));
	}

	let res: Result<PlayerBuilding> = conn.transaction(|connection| {
		info!("Initiating upgrade transaction");
		// deduct resources
		resources::deduct(
			connection,
			&player_bld.player_id,
			&(
				bld_lvl.req_food.unwrap_or(0),
				bld_lvl.req_wood.unwrap_or(0),
				bld_lvl.req_stone.unwrap_or(0),
				bld_lvl.req_gold.unwrap_or(0),
			),
		)?;
		trace!("Deducted resources");
		// upgrade building
		let upgrade_eta = Utc::now().add(TimeDelta::seconds(bld_lvl.upgrade_seconds));
		let player_bld = player_buildings::set_upgrade_eta(
			connection,
			player_bld_id,
			Some(&upgrade_eta.to_rfc3339()),
		)?;
		debug!("Building upgrade started: {:?}", player_bld);
		Ok(player_bld)
	});

	match res {
		Ok(player_bld) => {
			info!(
				"Successfully started building {} upgrade for player {}",
				player_bld_id, player_bld.player_id
			);
			Ok(player_bld)
		}
		Err(e) => {
			warn!("Failed to start building {} upgrade: {}", player_bld_id, e);
			Err(Error::from((
				ErrorKind::UpgradeBuildingError,
				"Failed to upgrade building",
			)))
		}
	}
}

/// Confirms completion of a building upgrade.
///
/// This function processes the completion of a building upgrade by verifying that
/// the upgrade time has elapsed and incrementing the building level. It validates
/// both the upgrade state and timing constraints.
///
/// # Arguments
///
/// * `conn` - Database connection for performing operations
/// * `id` - Unique identifier of the player building to confirm upgrade
///
/// # Returns
///
/// Returns `Ok(())` on successful upgrade confirmation, or an error if:
/// - Building is not in upgrading state
/// - Upgrade time has not yet elapsed
/// - Time format is invalid
/// - Database operation fails
///
/// # Errors
///
/// This function returns `ConfirmUpgradeError` variants for:
/// - Invalid state ("Building is not upgrading")
/// - Premature confirmation ("Upgrade time has not passed")
/// - Invalid time format ("Invalid time format")
#[instrument(skip(conn))]
pub fn confirm_upgrade(conn: &mut DbConn, id: &PlayerBuildingKey) -> Result<PlayerBuilding> {
	debug!("Starting confirm upgrade for building {}", id);
	let player_bld = player_buildings::get_by_id(conn, id)?;
	trace!("Player building details: {:?}", player_bld);
	match player_bld.upgrade_finishes_at {
		None => {
			debug!("Building {} is not in upgrading state", id);
			Err(Error::from((
				ErrorKind::ConfirmUpgradeError,
				"Building is not upgrading",
			)))
		}
		Some(eta) => {
			let upgrade_finishes_at = DateTime::parse_from_rfc3339(&eta).map_err(|_| {
				Error::from((ErrorKind::ConfirmUpgradeError, "Invalid time format"))
			})?;
			if Utc::now() >= upgrade_finishes_at.to_utc() {
				debug!("Upgrade time has passed, incrementing building level");
				let bld = player_buildings::inc_level(conn, id)?;
				info!("Successfully confirmed upgrade for building {}", id);
				trace!(?bld, "Updated player building details");
				Ok(bld)
			} else {
				debug!(
					"Upgrade time has not passed yet: current={}, finishes_at={}",
					Utc::now(),
					upgrade_finishes_at
				);
				Err(Error::from((
					ErrorKind::ConfirmUpgradeError,
					"Upgrade time has not passed",
				)))
			}
		}
	}
}

/// Validates whether a player has sufficient resources for a building operation.
///
/// This internal utility function checks all four resource types (food, wood, stone, gold)
/// against the requirements specified in the building level configuration. It provides
/// detailed logging of resource availability for debugging purposes.
///
/// # Arguments
///
/// * `conn` - Database connection for querying player resources
/// * `player_id` - Unique identifier of the player whose resources to check
/// * `bld_lvl` - Building level configuration containing resource requirements
///
/// # Returns
///
/// Returns `true` if the player has sufficient resources for all required types,
/// `false` if any resource is insufficient, or an error if the database query fails.
///
/// # Errors
///
/// This function may return database-related errors if the player resource query fails.
#[instrument(skip(conn, bld_lvl))]
fn has_enough_resources(
	conn: &mut DbConn,
	player_id: &PlayerKey,
	bld_lvl: &BuildingLevel,
) -> Result<bool> {
	debug!("Checking resources for player: {}", player_id);
	trace!(
		"Required resources: food={}, wood={}, stone={}, gold={}",
		bld_lvl.req_food.unwrap_or(0),
		bld_lvl.req_wood.unwrap_or(0),
		bld_lvl.req_stone.unwrap_or(0),
		bld_lvl.req_gold.unwrap_or(0)
	);
	let res = resources::get_by_player_id(conn, player_id)?;
	let has_enough_food = res.food >= bld_lvl.req_food.unwrap_or(0);
	let has_enough_wood = res.wood >= bld_lvl.req_wood.unwrap_or(0);
	let has_enough_stone = res.stone >= bld_lvl.req_stone.unwrap_or(0);
	let has_enough_gold = res.gold >= bld_lvl.req_gold.unwrap_or(0);
	debug!(
		"Has enough of resource: food({}) wood({}) stone({}) gold({})",
		has_enough_food, has_enough_wood, has_enough_stone, has_enough_gold
	);
	Ok(has_enough_food && has_enough_wood && has_enough_stone && has_enough_gold)
}
