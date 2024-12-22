use std::str::FromStr;

use chrono::prelude::*;
use diesel::Connection;

use crate::db::building_levels::BuildingLevelRepository;
use crate::db::buildings::BuildingRepository;
use crate::db::resources::ResourcesRepository;
use crate::db::user_buildings::UserBuildingRepository;
use crate::db::{DbConn, Repository};
use crate::models::building_level::BuildingLevel;
use crate::models::error::{EmpError, EmpResult, ErrorKind};
use crate::models::user_building::{NewUserBuilding, UserBuilding};
use crate::models::{building, user, user_building};

pub struct BuildingService<'a> {
    connection: &'a mut DbConn,
    bld_repo: BuildingRepository,
    usr_bld_repo: UserBuildingRepository,
    bld_lvl_repo: BuildingLevelRepository,
    res_repo: ResourcesRepository,
}

impl BuildingService<'_> {
    pub fn new(connection: &mut DbConn) -> BuildingService {
        BuildingService {
            connection,
            bld_repo: BuildingRepository {},
            usr_bld_repo: UserBuildingRepository {},
            bld_lvl_repo: BuildingLevelRepository {},
            res_repo: ResourcesRepository {},
        }
    }

    pub fn construct_building(
        &mut self,
        usr_id: &user::PK,
        bld_id: &building::PK,
    ) -> EmpResult<UserBuilding> {
        log::info!("Constructing building: {} for user: {}", bld_id, usr_id);
        let bld_lvl = self
            .bld_lvl_repo
            .get_next_upgrade(&mut *self.connection, bld_id, &0)?;

        // check for resources
        if !self.has_enough_resources(usr_id, &bld_lvl)? {
            return Err(EmpError::from((
                ErrorKind::ConstructBuildingError,
                "Not enough resources",
            )));
        }

        // verify if maximum number of buildings was reached
        if !self
            .usr_bld_repo
            .can_construct(&mut *self.connection, usr_id, bld_id)?
        {
            return Err(EmpError::from((
                ErrorKind::ConstructBuildingError,
                "Max buildings reached",
            )));
        }

        let res: EmpResult<UserBuilding> = (&mut self.connection).transaction(|connection| {
            log::info!("Initiating construction transaction");
            // deduct resources
            self.res_repo.deduct(
                connection,
                usr_id,
                &(
                    bld_lvl.req_food.unwrap_or(0),
                    bld_lvl.req_wood.unwrap_or(0),
                    bld_lvl.req_stone.unwrap_or(0),
                    bld_lvl.req_gold.unwrap_or(0),
                ),
            )?;
            // construct building
            let usr_bld = self.usr_bld_repo.create(
                connection,
                &NewUserBuilding {
                    user_id: *usr_id,
                    building_id: *bld_id,
                    level: Some(0),
                    upgrade_time: Some(bld_lvl.upgrade_time.as_str()),
                },
            )?;
            log::info!("Building constructed: {:?}", usr_bld);
            Ok(usr_bld)
        });

        match res {
            Ok(usr_bld) => Ok(usr_bld),
            Err(_) => Err(EmpError::from((
                ErrorKind::ConstructBuildingError,
                "Failed to construct building",
            ))),
        }
    }

    pub fn upgrade_building(&mut self, usr_bld_id: &user_building::PK) -> EmpResult<()> {
        let (usr_bld, max_level) = self
            .usr_bld_repo
            .get_upgrade_tuple(&mut *self.connection, usr_bld_id)?;
        let bld_lvl = self.bld_lvl_repo.get_next_upgrade(
            &mut *self.connection,
            &usr_bld.building_id,
            &usr_bld.level,
        )?;

        // check for resources
        if !self.has_enough_resources(&usr_bld.user_id, &bld_lvl)? {
            return Err(EmpError::from((
                ErrorKind::UpgradeBuildingError,
                "Not enough resources",
            )));
        }

        // check for max level constraints
        if usr_bld.level >= max_level.unwrap_or(0) {
            return Err(EmpError::from((
                ErrorKind::UpgradeBuildingError,
                "Building is at max level",
            )));
        }

        let res: EmpResult<UserBuilding> = (&mut self.connection).transaction(|connection| {
            log::info!("Initiating upgrade transaction");
            // deduct resources
            self.res_repo.deduct(
                connection,
                &usr_bld.user_id,
                &(
                    bld_lvl.req_food.unwrap_or(0),
                    bld_lvl.req_wood.unwrap_or(0),
                    bld_lvl.req_stone.unwrap_or(0),
                    bld_lvl.req_gold.unwrap_or(0),
                ),
            )?;
            // upgrade building
            let usr_bld = self.usr_bld_repo.set_upgrade_time(
                connection,
                usr_bld_id,
                Some(&bld_lvl.upgrade_time.as_str()),
            )?;
            log::info!("Building upgrade started: {:?}", usr_bld);
            Ok(usr_bld)
        });

        match usr_bld.upgrade_time {
            Some(_) => Ok(()),
            None => Err(EmpError::from((
                ErrorKind::UpgradeBuildingError,
                "Failed to upgrade building",
            ))),
        }
    }

    pub fn confirm_upgrade(&mut self, id: &user_building::PK) -> EmpResult<()> {
        let usr_bld = self.usr_bld_repo.get_by_id(&mut *self.connection, id)?;
        match usr_bld.upgrade_time {
            None => Err(EmpError::from((
                ErrorKind::ConfirmUpgradeError,
                "Building is not upgrading",
            ))),
            Some(t) => {
                let time = DateTime::<Utc>::from_str(&t.as_str()).map_err(|_| {
                    EmpError::from((ErrorKind::ConfirmUpgradeError, "Invalid time format"))
                })?;
                if time <= Utc::now() {
                    self.usr_bld_repo.inc_level(&mut *self.connection, id)?;
                    Ok(())
                } else {
                    Err(EmpError::from((
                        ErrorKind::ConfirmUpgradeError,
                        "Upgrade time has not passed",
                    )))
                }
            }
        }
    }

    fn has_enough_resources(
        &mut self,
        user_id: &user::PK,
        bld_lvl: &BuildingLevel,
    ) -> EmpResult<bool> {
        log::debug!("Checking resources for user: {}", user_id);
        let res = self.res_repo.get_by_id(&mut *self.connection, user_id)?;
        let has_enough_food = res.food >= bld_lvl.req_food.unwrap_or(0);
        let has_enough_wood = res.wood >= bld_lvl.req_wood.unwrap_or(0);
        let has_enough_stone = res.stone >= bld_lvl.req_stone.unwrap_or(0);
        let has_enough_gold = res.gold >= bld_lvl.req_gold.unwrap_or(0);
        log::debug!(
            "Has enough resources: food({}) wood({}) stone({}) gold({})",
            has_enough_food,
            has_enough_wood,
            has_enough_stone,
            has_enough_gold
        );
        Ok(has_enough_food && has_enough_wood && has_enough_stone && has_enough_gold)
    }
}
