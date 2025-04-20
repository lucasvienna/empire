use diesel::prelude::*;
use tracing::info;

use crate::db::{DbConn, Repository};
use crate::domain::buildings::Building;
use crate::domain::error::Result;
use crate::domain::user_building::{NewUserBuilding, UpdateUserBuilding, UserBuilding};
use crate::domain::{buildings, user, user_building};
use crate::schema::{building, user_buildings};

#[derive(Debug)]
pub struct UserBuildingRepository {}

impl Repository<UserBuilding, NewUserBuilding, UpdateUserBuilding, user_building::PK>
    for UserBuildingRepository
{
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<UserBuilding>> {
        let buildings = user_buildings::table
            .select(UserBuilding::as_select())
            .load(connection)?;
        Ok(buildings)
    }

    fn get_by_id(&self, connection: &mut DbConn, id: &user_building::PK) -> Result<UserBuilding> {
        let building = user_buildings::table.find(id).first(connection)?;
        Ok(building)
    }

    fn create(&self, connection: &mut DbConn, entity: NewUserBuilding) -> Result<UserBuilding> {
        let building = diesel::insert_into(user_buildings::table)
            .values(entity)
            .returning(UserBuilding::as_returning())
            .get_result(connection)?;
        Ok(building)
    }

    fn update(
        &self,
        connection: &mut DbConn,
        id: &user_building::PK,
        entity: UpdateUserBuilding,
    ) -> Result<UserBuilding> {
        let building = diesel::update(user_buildings::table.find(id))
            .set(entity)
            .get_result(connection)?;
        Ok(building)
    }

    fn delete(&self, connection: &mut DbConn, id: &user_building::PK) -> Result<usize> {
        let res = diesel::delete(user_buildings::table.find(id)).execute(connection)?;
        Ok(res)
    }
}

/**
* Tuple for upgrade information.  
*
* .0 - UserBuilding
* .1 - buildings.max_level
*/
type UpgradeTuple = (UserBuilding, Option<i32>);

impl UserBuildingRepository {
    pub fn can_construct(
        &self,
        connection: &mut DbConn,
        usr_id: &user::PK,
        bld_id: &buildings::BuildingKey,
    ) -> Result<bool> {
        info!(
            "Checking if user {} can construct building: {}",
            usr_id, bld_id
        );
        let bld = building::table
            .find(bld_id)
            .select(Building::as_select())
            .first(connection)?;
        let count = user_buildings::table
            .filter(user_buildings::user_id.eq(usr_id))
            .filter(user_buildings::building_id.eq(bld_id))
            .count()
            .get_result::<i64>(connection)?;
        info!(
            "User {} has {} buildings of type {}. Maximum is {}",
            usr_id, count, bld_id, bld.max_count
        );

        Ok(count < bld.max_count as i64)
    }

    pub fn get_upgrade_tuple(
        &self,
        connection: &mut DbConn,
        usr_bld_id: &user_building::PK,
    ) -> Result<UpgradeTuple> {
        let upgrade_tuple = user_buildings::table
            .left_join(building::table)
            .filter(user_buildings::id.eq(usr_bld_id))
            .select((UserBuilding::as_select(), building::max_level.nullable()))
            .first::<UpgradeTuple>(connection)?;
        Ok(upgrade_tuple)
    }

    pub fn set_upgrade_time(
        &self,
        connection: &mut DbConn,
        pk: &user_building::PK,
        upgrade_time: Option<&str>,
    ) -> Result<UserBuilding> {
        let building = diesel::update(user_buildings::table.find(pk))
            .set(user_buildings::upgrade_time.eq(upgrade_time))
            .returning(UserBuilding::as_returning())
            .get_result(connection)?;
        Ok(building)
    }

    /**
     * Increase the level of a building by one and resets the upgrade timer.
     */
    pub fn inc_level(
        &self,
        connection: &mut DbConn,
        id: &user_building::PK,
    ) -> Result<UserBuilding> {
        let building = diesel::update(user_buildings::table.find(id))
            .set((
                user_buildings::level.eq(user_buildings::level + 1),
                user_buildings::upgrade_time.eq(None::<String>),
            ))
            .returning(UserBuilding::as_returning())
            .get_result(connection)?;
        Ok(building)
    }
}
