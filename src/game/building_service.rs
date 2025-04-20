use std::fmt;
use std::str::FromStr;

use chrono::prelude::*;
use diesel::Connection;
use tracing::{debug, info, instrument};

use crate::db::building_levels::BuildingLevelRepository;
use crate::db::buildings::BuildingRepository;
use crate::db::resources::ResourcesRepository;
use crate::db::user_buildings::UserBuildingRepository;
use crate::db::{DbConn, Repository};
use crate::domain::building_levels::BuildingLevel;
use crate::domain::error::{Error, ErrorKind, Result};
use crate::domain::user_building::{NewUserBuilding, UserBuilding};
use crate::domain::{buildings, user, user_building};

pub struct BuildingService {
    connection: DbConn,
    bld_repo: BuildingRepository,
    usr_bld_repo: UserBuildingRepository,
    bld_lvl_repo: BuildingLevelRepository,
    res_repo: ResourcesRepository,
}

impl fmt::Debug for BuildingService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BuildingService")
    }
}

impl BuildingService {
    pub fn new(connection: DbConn) -> BuildingService {
        BuildingService {
            connection,
            bld_repo: BuildingRepository {},
            usr_bld_repo: UserBuildingRepository {},
            bld_lvl_repo: BuildingLevelRepository {},
            res_repo: ResourcesRepository {},
        }
    }

    #[instrument(skip(self))]
    pub fn construct_building(
        &mut self,
        usr_id: &user::UserKey,
        bld_id: &buildings::BuildingKey,
    ) -> Result<UserBuilding> {
        info!("Constructing building: {} for user: {}", bld_id, usr_id);
        let bld_lvl = self
            .bld_lvl_repo
            .get_next_upgrade(&mut self.connection, bld_id, &0)?;

        // check for resources
        if !self.has_enough_resources(usr_id, &bld_lvl)? {
            return Err(Error::from((
                ErrorKind::ConstructBuildingError,
                "Not enough resources",
            )));
        }

        // verify if maximum number of buildings was reached
        if !self
            .usr_bld_repo
            .can_construct(&mut self.connection, usr_id, bld_id)?
        {
            return Err(Error::from((
                ErrorKind::ConstructBuildingError,
                "Max buildings reached",
            )));
        }

        let res: Result<UserBuilding> = self.connection.transaction(|connection| {
            info!("Initiating construction transaction");
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
                NewUserBuilding {
                    user_id: *usr_id,
                    building_id: *bld_id,
                    level: Some(0),
                    upgrade_time: Some(bld_lvl.upgrade_time),
                },
            )?;
            info!("Building constructed: {:?}", usr_bld);
            Ok(usr_bld)
        });

        match res {
            Ok(usr_bld) => Ok(usr_bld),
            Err(_) => Err(Error::from((
                ErrorKind::ConstructBuildingError,
                "Failed to construct building",
            ))),
        }
    }

    #[instrument(skip(self))]
    pub fn upgrade_building(&mut self, usr_bld_id: &user_building::UserBuildingKey) -> Result<()> {
        let (usr_bld, max_level) = self
            .usr_bld_repo
            .get_upgrade_tuple(&mut self.connection, usr_bld_id)?;
        let bld_lvl = self.bld_lvl_repo.get_next_upgrade(
            &mut self.connection,
            &usr_bld.building_id,
            &usr_bld.level,
        )?;

        // check for resources
        if !self.has_enough_resources(&usr_bld.user_id, &bld_lvl)? {
            return Err(Error::from((
                ErrorKind::UpgradeBuildingError,
                "Not enough resources",
            )));
        }

        // check for max level constraints
        if usr_bld.level >= max_level.unwrap_or(0) {
            return Err(Error::from((
                ErrorKind::UpgradeBuildingError,
                "Building is at max level",
            )));
        }

        let res: Result<UserBuilding> = self.connection.transaction(|connection| {
            info!("Initiating upgrade transaction");
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
                Some(bld_lvl.upgrade_time.as_str()),
            )?;
            info!("Building upgrade started: {:?}", usr_bld);
            Ok(usr_bld)
        });

        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from((
                ErrorKind::UpgradeBuildingError,
                "Failed to upgrade building",
            ))),
        }
    }

    #[instrument(skip(self))]
    pub fn confirm_upgrade(&mut self, id: &user_building::UserBuildingKey) -> Result<()> {
        let usr_bld = self.usr_bld_repo.get_by_id(&mut self.connection, id)?;
        match usr_bld.upgrade_time {
            None => Err(Error::from((
                ErrorKind::ConfirmUpgradeError,
                "Building is not upgrading",
            ))),
            Some(t) => {
                let time = DateTime::<Utc>::from_str(t.as_str()).map_err(|_| {
                    Error::from((ErrorKind::ConfirmUpgradeError, "Invalid time format"))
                })?;
                if time <= Utc::now() {
                    self.usr_bld_repo.inc_level(&mut self.connection, id)?;
                    Ok(())
                } else {
                    Err(Error::from((
                        ErrorKind::ConfirmUpgradeError,
                        "Upgrade time has not passed",
                    )))
                }
            }
        }
    }

    fn has_enough_resources(
        &mut self,
        user_id: &user::UserKey,
        bld_lvl: &BuildingLevel,
    ) -> Result<bool> {
        debug!("Checking resources for user: {}", user_id);
        let res = self.res_repo.get_by_id(&mut self.connection, user_id)?;
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
}
