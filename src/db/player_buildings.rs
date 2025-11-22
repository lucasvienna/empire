//! Database access layer for player building entities.
//!
//! This module manages player-owned buildings, providing comprehensive CRUD operations
//! along with specialized functionality for building construction validation, upgrades,
//! and complex queries that join multiple tables to provide detailed building information.
//! It handles the relationship between players and their buildings, including level
//! progression and resource management.

use std::collections::HashMap;

use diesel::dsl::{count, max};
use diesel::prelude::*;
use tracing::info;

use crate::db::DbConn;
use crate::domain::building::level::BuildingLevel;
use crate::domain::building::resources::BuildingResource;
use crate::domain::building::{Building, BuildingKey};
use crate::domain::error::Result;
use crate::domain::factions::FactionCode;
use crate::domain::player::buildings::{
	NewPlayerBuilding, PlayerBuilding, PlayerBuildingKey, UpdatePlayerBuilding,
};
use crate::domain::player::{Player, PlayerKey};
use crate::game::buildings::requirement_operations::AvailabilityData;
use crate::schema::{building, player_building};

/// Retrieves all player buildings from the database.
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Returns
/// A Result containing a vector of all PlayerBuilding entities
pub fn get_all(conn: &mut DbConn) -> Result<Vec<PlayerBuilding>> {
	let buildings = player_building::table
		.select(PlayerBuilding::as_select())
		.load(conn)?;
	Ok(buildings)
}

/// Retrieves a single player building by its ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `id` - The unique identifier of the player building
///
/// # Returns
/// A Result containing the requested PlayerBuilding entity
pub fn get_by_id(conn: &mut DbConn, id: &PlayerBuildingKey) -> Result<PlayerBuilding> {
	let building = player_building::table.find(id).first(conn)?;
	Ok(building)
}

/// Creates a new player building in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `entity` - The NewPlayerBuilding entity to create
///
/// # Returns
/// A Result containing the created PlayerBuilding entity
pub fn create(conn: &mut DbConn, entity: NewPlayerBuilding) -> Result<PlayerBuilding> {
	let building = diesel::insert_into(player_building::table)
		.values(entity)
		.returning(PlayerBuilding::as_returning())
		.get_result(conn)?;
	Ok(building)
}

/// Updates an existing player building in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `entity` - The UpdatePlayerBuilding containing the changes
///
/// # Returns
/// A Result containing the updated PlayerBuilding entity
pub fn update(conn: &mut DbConn, entity: &UpdatePlayerBuilding) -> Result<PlayerBuilding> {
	let building = diesel::update(player_building::table)
		.set(entity)
		.get_result(conn)?;
	Ok(building)
}

/// Deletes a player building from the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `id` - The unique identifier of the player building to delete
///
/// # Returns
/// A Result containing the number of deleted records
pub fn delete(conn: &mut DbConn, id: &PlayerBuildingKey) -> Result<usize> {
	let res = diesel::delete(player_building::table.find(id)).execute(conn)?;
	Ok(res)
}

/// A tuple containing player building data and its maximum possible level.
///
/// - First element (`.0`) represents the PlayerBuilding instance
/// - Second element (`.1`) represents the maximum level from the buildings table
type UpgradeTuple = (PlayerBuilding, Option<i32>);

/// A tuple type alias representing a fully detailed building structure.
///
/// `FullBuildings` is a tuple that combines three elements:
/// 1. `PlayerBuilding` - The player's specific instance or association with the building,
///    which may include ownership details or custom modifications.
/// 2. `Building` - The core building type or structure, representing its general blueprint
///    or category.
/// 3. `BuildingLevel` - The current level or stage of progression of the building (e.g.,
///    upgrades or enhancements).
///
/// This type is useful for tying together these three distinct aspects into a single entity
/// to simplify data handling and representation in the context of player-related building
/// logic.
pub type FullBuilding = (PlayerBuilding, Building, BuildingLevel, BuildingResource);

/// Retrieves all buildings owned by a specific player.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_key` - The unique identifier of the player
///
/// # Returns
/// A vector containing all PlayerBuilding instances owned by the specified player
pub fn get_player_buildings(
	conn: &mut DbConn,
	player_key: &PlayerKey,
) -> Result<Vec<PlayerBuilding>> {
	use crate::schema::player_building::player_id;

	let player_blds: Vec<PlayerBuilding> = player_building::table
		.filter(player_id.eq(player_key))
		.get_results(conn)?;
	Ok(player_blds)
}

/// Retrieves a list of the full set of buildings associated with a given player.
///
/// This function fetches a collection of `FullBuildings` for the specified player, including information
/// about the player's buildings, their current levels, required resources for the next level,
/// and other related details. It performs database queries using JOINs across multiple tables to construct
/// the final result.
///
/// # Arguments
///
/// * `conn` - Database connection
/// * `player_key` - The unique identifier (`PlayerKey`) of the player whose buildings are to be retrieved.
///
/// # Returns
///
/// This function returns a `Result`:
/// * On success: A vector containing `FullBuildings` objects, where each object represents the detailed
///   state of a single building belonging to the player.
/// * On failure: An error if the database query fails or if there is an issue with the connection to the
///   database pool.
pub fn get_game_buildings(conn: &mut DbConn, player_key: &PlayerKey) -> Result<Vec<FullBuilding>> {
	use crate::schema::building::dsl as b;
	use crate::schema::building_level::dsl as bl;
	use crate::schema::building_resource::dsl as br;
	use crate::schema::player_building::dsl as pb;

	let results = pb::player_building
		.filter(pb::player_id.eq(player_key))
		.inner_join(b::building.on(pb::building_id.eq(b::id)))
		.inner_join(
			bl::building_level.on(pb::building_id
				.eq(bl::building_id)
				.and(bl::level.eq(pb::level + 1))),
		)
		.inner_join(
			br::building_resource.on(pb::building_id
				.eq(br::building_id)
				.and(pb::level.eq(br::building_level))),
		)
		.get_results::<FullBuilding>(conn)?;

	Ok(results)
}

/// Retrieves detailed information about a specific building for a player in the game.
///
/// This function queries the database to fetch a specific building's information,
/// including its level and associated resources, for a given player. It joins multiple
/// tables in the database schema (`player_building`, `building`, `building_level`,
/// and `building_resource`) to construct a complete view of the building's details.
///
/// # Arguments
///
/// * `conn` - Database connection
/// * `player_key` - A reference to the key representing the player for whom the building
///   information is being fetched. This key is used to filter player-specific data.
/// * `bld_key` - A reference to the key representing the specific building for which the
///   details are needed.
///
/// # Returns
///
/// Returns a `Result` containing the `FullBuilding` object if the building is found
/// successfully. If the query fails or no matching building data is found, an error of
/// type `diesel::result::Error` is returned.
pub fn get_game_building(
	conn: &mut DbConn,
	player_key: &PlayerBuildingKey,
	bld_key: &PlayerBuildingKey,
) -> Result<FullBuilding> {
	use crate::schema::building::dsl as b;
	use crate::schema::building_level::dsl as bl;
	use crate::schema::building_resource::dsl as br;
	use crate::schema::player_building::dsl as pb;

	let results = pb::player_building
		.filter(pb::player_id.eq(player_key).and(pb::id.eq(bld_key)))
		.inner_join(b::building.on(pb::building_id.eq(b::id)))
		.inner_join(
			bl::building_level.on(pb::building_id
				.eq(bl::building_id)
				.and(bl::level.eq(pb::level + 1))),
		)
		.inner_join(
			br::building_resource.on(pb::building_id
				.eq(br::building_id)
				.and(pb::level.eq(br::building_level))),
		)
		.first::<FullBuilding>(conn)?;

	Ok(results)
}

/// Constructs a new building for a player in the database.
///
/// # Arguments
/// * `conn` - Database connection
/// * `new_building` - The new building data to be inserted
///
/// # Returns
/// The newly created PlayerBuilding instance
pub fn construct(conn: &mut DbConn, new_building: NewPlayerBuilding) -> Result<PlayerBuilding> {
	let new_building = diesel::insert_into(player_building::table)
		.values(new_building)
		.returning(PlayerBuilding::as_returning())
		.get_result(conn)?;
	Ok(new_building)
}

/// Checks if a player can construct a specific building.
///
/// Validates if the player hasn't reached the maximum allowed number of buildings
/// for the specified type.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_id` - The unique identifier of the player
/// * `bld_id` - The unique identifier of the building type
///
/// # Returns
/// `true` if the player can construct the building, `false` otherwise
pub fn can_construct(
	conn: &mut DbConn,
	player_id: &PlayerKey,
	bld_id: &BuildingKey,
) -> Result<bool> {
	info!(
		"Checking if player {} can construct building: {}",
		player_id, bld_id
	);
	let bld = building::table
		.find(bld_id)
		.select(Building::as_select())
		.first(conn)?;
	let count = player_building::table
		.filter(player_building::player_id.eq(player_id))
		.filter(player_building::building_id.eq(bld_id))
		.count()
		.get_result::<i64>(conn)?;
	info!(
		"Player {} has {} buildings of type {}. Maximum is {}",
		player_id, count, bld_id, bld.max_count
	);

	Ok(count < bld.max_count as i64)
}

/// Retrieves building information needed for upgrade operations.
///
/// Returns a tuple containing the player building data and its maximum possible level.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_bld_id` - The unique identifier of the player's building
pub fn get_upgrade_tuple(
	conn: &mut DbConn,
	player_bld_id: &PlayerBuildingKey,
) -> Result<UpgradeTuple> {
	use crate::schema::building::columns as bld;
	use crate::schema::player_building::columns as pb;

	let upgrade_tuple = player_building::table
		.left_join(building::table)
		.filter(pb::id.eq(player_bld_id))
		.select((PlayerBuilding::as_select(), bld::max_level.nullable()))
		.first::<UpgradeTuple>(conn)?;
	Ok(upgrade_tuple)
}

/// Sets or clears the upgrade completion time for a player's building.
///
/// # Arguments
/// * `conn` - Database connection
/// * `player_building_key` - The unique identifier of the player's building
/// * `upgrade_eta` - The upgrade completion time as string, or None to clear it
///
/// # Returns
/// Updated PlayerBuilding instance
pub fn set_upgrade_eta(
	conn: &mut DbConn,
	player_building_key: &PlayerBuildingKey,
	upgrade_eta: Option<&str>,
) -> Result<PlayerBuilding> {
	let building = diesel::update(player_building::table.find(player_building_key))
		.set(player_building::upgrade_finishes_at.eq(upgrade_eta))
		.returning(PlayerBuilding::as_returning())
		.get_result(conn)?;
	Ok(building)
}

/// Increases the level of a building by one and resets the upgrade timer.
///
/// # Arguments
/// * `conn` - Database connection
/// * `id` - The unique identifier of the player's building
///
/// # Returns
/// Updated PlayerBuilding instance with incremented level
pub fn inc_level(conn: &mut DbConn, id: &PlayerBuildingKey) -> Result<PlayerBuilding> {
	let building = diesel::update(player_building::table.find(id))
		.set((
			player_building::level.eq(player_building::level + 1),
			player_building::upgrade_finishes_at.eq(None::<String>),
		))
		.returning(PlayerBuilding::as_returning())
		.get_result(conn)?;
	Ok(building)
}

/// Retrieves building counts and maximum levels for all buildings available to a player.
///
/// This function returns a map containing counts and maximum levels of buildings for a given player,
/// filtered by the player's faction. It includes both faction-specific buildings and neutral buildings
/// that are available to all factions.
///
/// # Arguments
///
/// * `conn` - Database connection
/// * `player` - Reference to the Player entity whose building information is being queried
///
/// # Returns
///
/// Returns a Result containing a `HashMap` where:
/// * Key is the BuildingKey (unique identifier for each building type)
/// * Value is a tuple containing:
///   * First element (`i64`): Count of how many instances of this building the player has
///   * Second element (`Option<i32>`): Maximum level achieved across all instances of this building,
///     or None if the player has no instances of this building
///
/// The HashMap includes entries for all buildings available to the player's faction, even if they
/// haven't built any instances of some building types (in which case the count will be 0 and the
/// level will be `None`).
pub fn get_player_bld_counts_levels(
	conn: &mut DbConn,
	player: &Player,
) -> Result<(
	HashMap<BuildingKey, Building>,
	HashMap<BuildingKey, AvailabilityData>,
)> {
	let building_counts: Vec<(Building, i64, Option<i32>)> = building::table
		.filter(
			building::faction
				.eq(&player.faction)
				.or(building::faction.eq(FactionCode::Neutral)),
		)
		.left_join(
			player_building::table.on(building::id
				.eq(player_building::building_id)
				.and(player_building::player_id.eq(&player.id))),
		)
		.group_by(building::id)
		.select((
			Building::as_select(),
			count(player_building::id).assume_not_null(),
			max(player_building::level).nullable(),
		))
		.load(conn)?;

	let mut buildings: HashMap<BuildingKey, Building> = HashMap::new();
	let player_building_levels = building_counts.into_iter().fold(
		HashMap::new(),
		|mut map, (building, count, max_level)| {
			map.insert(building.id, (count, building.max_count, max_level));
			buildings.insert(building.id, building);
			map
		},
	);
	Ok((buildings, player_building_levels))
}
