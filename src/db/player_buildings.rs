use diesel::prelude::*;
use tracing::info;

use crate::db::{DbConn, Repository};
use crate::domain::buildings::{Building, BuildingKey};
use crate::domain::error::Result;
use crate::domain::player::buildings::{
    NewPlayerBuilding, PlayerBuilding, PlayerBuildingKey, UpdatePlayerBuilding,
};
use crate::domain::player::PlayerKey;
use crate::schema::{building, player_building};

#[derive(Debug)]
pub struct PlayerBuildingRepository;

impl Repository<PlayerBuilding, NewPlayerBuilding, &UpdatePlayerBuilding, PlayerBuildingKey>
    for PlayerBuildingRepository
{
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<PlayerBuilding>> {
        let buildings = player_building::table
            .select(PlayerBuilding::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(&self, connection: &mut DbConn, id: &PlayerBuildingKey) -> Result<PlayerBuilding> {
        let building = player_building::table.find(id).first(connection)?;
        Ok(building)
    }

    fn create(&self, connection: &mut DbConn, entity: NewPlayerBuilding) -> Result<PlayerBuilding> {
        let building = diesel::insert_into(player_building::table)
            .values(entity)
            .returning(PlayerBuilding::as_returning())
            .get_result(connection)?;
        Ok(building)
    }

    fn update(
        &self,
        connection: &mut DbConn,
        entity: &UpdatePlayerBuilding,
    ) -> Result<PlayerBuilding> {
        let building = diesel::update(player_building::table)
            .set(entity)
            .get_result(connection)?;
        Ok(building)
    }

    fn delete(&self, connection: &mut DbConn, id: &PlayerBuildingKey) -> Result<usize> {
        let res = diesel::delete(player_building::table.find(id)).execute(connection)?;
        Ok(res)
    }
}

/**
* Tuple for upgrade information.  
*
* .0 - PlayerBuilding
* .1 - buildings.max_level
*/
type UpgradeTuple = (PlayerBuilding, Option<i32>);

impl PlayerBuildingRepository {
    pub fn can_construct(
        &self,
        connection: &mut DbConn,
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
            .first(connection)?;
        let count = player_building::table
            .filter(player_building::player_id.eq(player_id))
            .filter(player_building::building_id.eq(bld_id))
            .count()
            .get_result::<i64>(connection)?;
        info!(
            "Player {} has {} buildings of type {}. Maximum is {}",
            player_id, count, bld_id, bld.max_count
        );

        Ok(count < bld.max_count as i64)
    }

    pub fn get_upgrade_tuple(
        &self,
        connection: &mut DbConn,
        player_bld_id: &PlayerBuildingKey,
    ) -> Result<UpgradeTuple> {
        let upgrade_tuple = player_building::table
            .left_join(building::table)
            .filter(player_building::id.eq(player_bld_id))
            .select((PlayerBuilding::as_select(), building::max_level.nullable()))
            .first::<UpgradeTuple>(connection)?;
        Ok(upgrade_tuple)
    }

    pub fn set_upgrade_time(
        &self,
        connection: &mut DbConn,
        player_building_key: &PlayerBuildingKey,
        upgrade_time: Option<&str>,
    ) -> Result<PlayerBuilding> {
        let building = diesel::update(player_building::table.find(player_building_key))
            .set(player_building::upgrade_time.eq(upgrade_time))
            .returning(PlayerBuilding::as_returning())
            .get_result(connection)?;
        Ok(building)
    }

    /**
     * Increase the level of a building by one and resets the upgrade timer.
     */
    pub fn inc_level(
        &self,
        connection: &mut DbConn,
        id: &PlayerBuildingKey,
    ) -> Result<PlayerBuilding> {
        let building = diesel::update(player_building::table.find(id))
            .set((
                player_building::level.eq(player_building::level + 1),
                player_building::upgrade_time.eq(None::<String>),
            ))
            .returning(PlayerBuilding::as_returning())
            .get_result(connection)?;
        Ok(building)
    }
}
