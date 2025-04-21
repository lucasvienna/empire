use std::fmt;

use diesel::prelude::*;
use tracing::info;

use crate::db::{DbConn, DbPool, Repository};
use crate::domain::buildings::{Building, BuildingKey};
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
/// * `connection` - Database connection pool
pub struct PlayerBuildingRepository {
    connection: DbConn,
}

impl fmt::Debug for PlayerBuildingRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlayerBuildingRepository")
    }
}

impl Repository<PlayerBuilding, NewPlayerBuilding, &UpdatePlayerBuilding, PlayerBuildingKey>
    for PlayerBuildingRepository
{
    fn try_from_pool(pool: &DbPool) -> Result<Self> {
        Ok(Self {
            connection: pool.get()?,
        })
    }

    fn from_connection(connection: DbConn) -> Self {
        Self { connection }
    }

    fn get_all(&mut self) -> Result<Vec<PlayerBuilding>> {
        let buildings = player_building::table
            .select(PlayerBuilding::as_select())
            .load(&mut self.connection)?;
        Ok(buildings)
    }

    fn get_by_id(&mut self, id: &PlayerBuildingKey) -> Result<PlayerBuilding> {
        let building = player_building::table
            .find(id)
            .first(&mut self.connection)?;
        Ok(building)
    }

    fn create(&mut self, entity: NewPlayerBuilding) -> Result<PlayerBuilding> {
        let building = diesel::insert_into(player_building::table)
            .values(entity)
            .returning(PlayerBuilding::as_returning())
            .get_result(&mut self.connection)?;
        Ok(building)
    }

    fn update(&mut self, entity: &UpdatePlayerBuilding) -> Result<PlayerBuilding> {
        let building = diesel::update(player_building::table)
            .set(entity)
            .get_result(&mut self.connection)?;
        Ok(building)
    }

    fn delete(&mut self, id: &PlayerBuildingKey) -> Result<usize> {
        let res = diesel::delete(player_building::table.find(id)).execute(&mut self.connection)?;
        Ok(res)
    }
}

/// A tuple containing player building data and its maximum possible level.
///
/// - First element (`.0`) represents the PlayerBuilding instance
/// - Second element (`.1`) represents the maximum level from the buildings table
type UpgradeTuple = (PlayerBuilding, Option<i32>);

impl PlayerBuildingRepository {
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
    pub fn can_construct(&mut self, player_id: &PlayerKey, bld_id: &BuildingKey) -> Result<bool> {
        info!(
            "Checking if player {} can construct building: {}",
            player_id, bld_id
        );
        let bld = building::table
            .find(bld_id)
            .select(Building::as_select())
            .first(&mut self.connection)?;
        let count = player_building::table
            .filter(player_building::player_id.eq(player_id))
            .filter(player_building::building_id.eq(bld_id))
            .count()
            .get_result::<i64>(&mut self.connection)?;
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
    pub fn get_upgrade_tuple(&mut self, player_bld_id: &PlayerBuildingKey) -> Result<UpgradeTuple> {
        use crate::schema::building::columns as bld;
        use crate::schema::player_building::columns as pb;

        let upgrade_tuple = player_building::table
            .left_join(building::table)
            .filter(pb::id.eq(player_bld_id))
            .select((PlayerBuilding::as_select(), bld::max_level.nullable()))
            .first::<UpgradeTuple>(&mut self.connection)?;
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
        &mut self,
        player_building_key: &PlayerBuildingKey,
        upgrade_time: Option<&str>,
    ) -> Result<PlayerBuilding> {
        let building = diesel::update(player_building::table.find(player_building_key))
            .set(player_building::upgrade_time.eq(upgrade_time))
            .returning(PlayerBuilding::as_returning())
            .get_result(&mut self.connection)?;
        Ok(building)
    }

    /// Increases the level of a building by one and resets the upgrade timer.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the player's building
    ///
    /// # Returns
    /// Updated PlayerBuilding instance with incremented level
    pub fn inc_level(&mut self, id: &PlayerBuildingKey) -> Result<PlayerBuilding> {
        let building = diesel::update(player_building::table.find(id))
            .set((
                player_building::level.eq(player_building::level + 1),
                player_building::upgrade_time.eq(None::<String>),
            ))
            .returning(PlayerBuilding::as_returning())
            .get_result(&mut self.connection)?;
        Ok(building)
    }
}
