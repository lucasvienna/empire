use std::fmt;
use std::sync::Arc;

use axum::extract::FromRef;
use diesel::prelude::*;
use tracing::info;
use uuid::Uuid;

use crate::db::{DbConn, Repository};
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::building::level::BuildingLevel;
use crate::domain::building::resources::BuildingResource;
use crate::domain::building::{Building, BuildingKey};
use crate::domain::error::Result;
use crate::domain::player::buildings::{
    NewPlayerBuilding, PlayerBuilding, PlayerBuildingKey, UpdatePlayerBuilding,
};
use crate::domain::player::PlayerKey;
use crate::schema::{building, player_building};

/// Repository for managing player building data in the database.
///
/// Provides CRUD operations and additional functionality for managing player buildings,
/// including construction validation, upgrades, and level management.
///
/// # Fields
/// * `pool` - Thread-safe connection pool of type [`AppPool`] for database access
pub struct PlayerBuildingRepository {
    pool: AppPool,
}

impl fmt::Debug for PlayerBuildingRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlayerBuildingRepository")
    }
}

impl FromRef<AppState> for PlayerBuildingRepository {
    fn from_ref(state: &AppState) -> Self {
        Self::new(&state.db_pool)
    }
}

impl Repository<PlayerBuilding, NewPlayerBuilding, &UpdatePlayerBuilding, PlayerBuildingKey>
    for PlayerBuildingRepository
{
    fn new(pool: &AppPool) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    fn get_all(&self) -> Result<Vec<PlayerBuilding>> {
        let mut conn = self.pool.get()?;
        let buildings = player_building::table
            .select(PlayerBuilding::as_select())
            .load(&mut conn)?;
        Ok(buildings)
    }

    fn get_by_id(&self, id: &PlayerBuildingKey) -> Result<PlayerBuilding> {
        let mut conn = self.pool.get()?;
        let building = player_building::table.find(id).first(&mut conn)?;
        Ok(building)
    }

    fn create(&self, entity: NewPlayerBuilding) -> Result<PlayerBuilding> {
        let mut conn = self.pool.get()?;
        let building = diesel::insert_into(player_building::table)
            .values(entity)
            .returning(PlayerBuilding::as_returning())
            .get_result(&mut conn)?;
        Ok(building)
    }

    fn update(&self, entity: &UpdatePlayerBuilding) -> Result<PlayerBuilding> {
        let mut conn = self.pool.get()?;
        let building = diesel::update(player_building::table)
            .set(entity)
            .get_result(&mut conn)?;
        Ok(building)
    }

    fn delete(&self, id: &PlayerBuildingKey) -> Result<usize> {
        let mut conn = self.pool.get()?;
        let res = diesel::delete(player_building::table.find(id)).execute(&mut conn)?;
        Ok(res)
    }
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

impl PlayerBuildingRepository {
    /// Retrieves all buildings owned by a specific player.
    ///
    /// # Arguments
    /// * `player_key` - The unique identifier of the player
    ///
    /// # Returns
    /// A vector containing all PlayerBuilding instances owned by the specified player
    pub fn get_player_buildings(&self, player_key: &PlayerKey) -> Result<Vec<PlayerBuilding>> {
        use crate::schema::player_building::player_id;
        let mut conn = self.pool.get()?;

        let player_blds: Vec<PlayerBuilding> = player_building::table
            .filter(player_id.eq(player_key))
            .get_results(&mut conn)?;
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
    /// * `player_key` - The unique identifier (`PlayerKey`) of the player whose buildings are to be retrieved.
    ///
    /// # Returns
    ///
    /// This function returns a `Result`:
    /// * On success: A vector containing `FullBuildings` objects, where each object represents the detailed
    ///   state of a single building belonging to the player.
    /// * On failure: An error if the database query fails or if there is an issue with the connection to the
    ///   database pool.
    ///
    /// # SQL Query
    ///
    /// The function builds and executes a SQL query for retrieving building data using the following JOINs:
    /// * `player_building` table filtered by `player_id`.
    /// * Inner joined with the `building` table based on `building_id`.
    /// * Inner joined with the `building_level` table to retrieve information for the next building level
    ///   (i.e. `level + 1`).
    /// * Inner joined with the `building_resource` table to fetch resource details corresponding to
    ///   the player's current building level.
    ///
    /// # Errors
    ///
    /// The function may return an error in the following scenarios:
    /// * Failure to acquire a connection from the database pool.
    /// * Issues encountered while executing the query (e.g., missing data or mismatched relations between tables).
    pub fn get_game_buildings(&self, player_key: &PlayerKey) -> Result<Vec<FullBuilding>> {
        use crate::schema::building::dsl as b;
        use crate::schema::building_level::dsl as bl;
        use crate::schema::building_resource::dsl as br;
        use crate::schema::player_building::dsl as pb;

        let mut conn = self.pool.get()?;
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
            .get_results::<FullBuilding>(&mut conn)?;

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
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// * The database connection cannot be obtained.
    /// * The query fails due to a schema constraint or unexpected condition.
    /// * No matching building for the given `player_key` and `bld_key` is found.
    pub fn get_game_building(
        &self,
        player_key: &PlayerBuildingKey,
        bld_key: &PlayerBuildingKey,
    ) -> Result<FullBuilding> {
        use crate::schema::building::dsl as b;
        use crate::schema::building_level::dsl as bl;
        use crate::schema::building_resource::dsl as br;
        use crate::schema::player_building::dsl as pb;

        let mut conn = self.pool.get()?;
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
            .first::<FullBuilding>(&mut conn)?;

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
    pub fn construct(
        &self,
        conn: &mut DbConn,
        new_building: NewPlayerBuilding,
    ) -> Result<PlayerBuilding> {
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
    /// * `player_id` - The unique identifier of the player
    /// * `bld_id` - The unique identifier of the building type
    ///
    /// # Returns
    /// `true` if the player can construct the building, `false` otherwise
    pub fn can_construct(
        &self,
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
    /// * `player_bld_id` - The unique identifier of the player's building
    pub fn get_upgrade_tuple(
        &self,
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

    /// Sets or clears the upgrade time for a player's building.
    ///
    /// # Arguments
    /// * `player_building_key` - The unique identifier of the player's building
    /// * `upgrade_time` - The upgrade completion time as string, or None to clear it
    ///
    /// # Returns
    /// Updated PlayerBuilding instance
    pub fn set_upgrade_time(
        &self,
        conn: &mut DbConn,
        player_building_key: &PlayerBuildingKey,
        upgrade_time: Option<&str>,
    ) -> Result<PlayerBuilding> {
        let building = diesel::update(player_building::table.find(player_building_key))
            .set(player_building::upgrade_time.eq(upgrade_time))
            .returning(PlayerBuilding::as_returning())
            .get_result(conn)?;
        Ok(building)
    }

    /// Increases the level of a building by one and resets the upgrade timer.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the player's building
    ///
    /// # Returns
    /// Updated PlayerBuilding instance with incremented level
    pub fn inc_level(&self, conn: &mut DbConn, id: &PlayerBuildingKey) -> Result<PlayerBuilding> {
        let building = diesel::update(player_building::table.find(id))
            .set((
                player_building::level.eq(player_building::level + 1),
                player_building::upgrade_time.eq(None::<String>),
            ))
            .returning(PlayerBuilding::as_returning())
            .get_result(conn)?;
        Ok(building)
    }

    /// Diesel version of the resource_generation view
    fn res_gen_view(&self, conn: &mut DbConn, player_key: &PlayerKey) -> Result<()> {
        use bigdecimal::BigDecimal;
        use diesel::dsl::sum;

        use crate::schema::{building_resource as br, player_building as pb};

        let something = pb::table
            .left_join(
                br::table.on(pb::building_id
                    .eq(br::building_id)
                    .and(pb::level.eq(br::building_level))),
            )
            .group_by(pb::player_id)
            .filter(pb::player_id.eq(player_key))
            .select((
                pb::player_id,
                sum(br::population).assume_not_null(),
                sum(br::food).assume_not_null(),
                sum(br::wood).assume_not_null(),
                sum(br::stone).assume_not_null(),
                sum(br::gold).assume_not_null(),
            ))
            .first::<(
                Uuid,
                BigDecimal,
                BigDecimal,
                BigDecimal,
                BigDecimal,
                BigDecimal,
            )>(conn)?;

        Ok(())
    }
}
