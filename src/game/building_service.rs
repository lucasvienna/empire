use std::fmt;
use std::str::FromStr;

use chrono::prelude::*;
use diesel::Connection;
use tracing::{debug, info, instrument};

use crate::db::building_levels::BuildingLevelRepository;
use crate::db::buildings::BuildingRepository;
use crate::db::player_buildings::PlayerBuildingRepository;
use crate::db::resources::ResourcesRepository;
use crate::db::{DbPool, Repository};
use crate::domain::building_levels::BuildingLevel;
use crate::domain::buildings::BuildingKey;
use crate::domain::error::{Error, ErrorKind, Result};
use crate::domain::player::buildings::{NewPlayerBuilding, PlayerBuilding};
use crate::domain::player::{buildings, PlayerKey};

pub struct BuildingService {
    db_pool: DbPool,
    bld_repo: BuildingRepository,
    player_bld_repo: PlayerBuildingRepository,
    bld_lvl_repo: BuildingLevelRepository,
    res_repo: ResourcesRepository,
}

impl fmt::Debug for BuildingService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BuildingService")
    }
}

impl BuildingService {
    pub fn new(pool: DbPool) -> Result<BuildingService> {
        Ok(BuildingService {
            bld_repo: BuildingRepository::try_from_pool(&pool)?,
            player_bld_repo: PlayerBuildingRepository::try_from_pool(&pool)?,
            bld_lvl_repo: BuildingLevelRepository::try_from_pool(&pool)?,
            res_repo: ResourcesRepository::try_from_pool(&pool)?,
            db_pool: pool,
        })
    }

    #[instrument(skip(self))]
    pub fn construct_building(
        &mut self,
        player_id: &PlayerKey,
        bld_id: &BuildingKey,
    ) -> Result<PlayerBuilding> {
        info!(
            "Constructing building: {} for player: {}",
            bld_id, player_id
        );
        let bld_lvl = self.bld_lvl_repo.get_next_upgrade(bld_id, &0)?;

        // check for resources
        if !self.has_enough_resources(player_id, &bld_lvl)? {
            return Err(Error::from((
                ErrorKind::ConstructBuildingError,
                "Not enough resources",
            )));
        }

        // verify if maximum number of buildings was reached
        if !self.player_bld_repo.can_construct(player_id, bld_id)? {
            return Err(Error::from((
                ErrorKind::ConstructBuildingError,
                "Max buildings reached",
            )));
        }

        let mut conn = self.db_pool.get()?;
        let res: Result<PlayerBuilding> = conn.transaction(|connection| {
            info!("Initiating construction transaction");
            // deduct resources
            self.res_repo.deduct(
                player_id,
                &(
                    bld_lvl.req_food.unwrap_or(0),
                    bld_lvl.req_wood.unwrap_or(0),
                    bld_lvl.req_stone.unwrap_or(0),
                    bld_lvl.req_gold.unwrap_or(0),
                ),
            )?;
            // construct building
            let player_bld = self.player_bld_repo.create(NewPlayerBuilding {
                player_id: *player_id,
                building_id: *bld_id,
                level: Some(0),
                upgrade_time: Some(bld_lvl.upgrade_time),
            })?;
            info!("Building constructed: {:?}", player_bld);
            Ok(player_bld)
        });

        match res {
            Ok(player_bld) => Ok(player_bld),
            Err(_) => Err(Error::from((
                ErrorKind::ConstructBuildingError,
                "Failed to construct building",
            ))),
        }
    }

    #[instrument(skip(self))]
    pub fn upgrade_building(&mut self, player_bld_id: &buildings::PlayerBuildingKey) -> Result<()> {
        let (player_bld, max_level) = self.player_bld_repo.get_upgrade_tuple(player_bld_id)?;
        let bld_lvl = self
            .bld_lvl_repo
            .get_next_upgrade(&player_bld.building_id, &player_bld.level)?;

        // check for resources
        if !self.has_enough_resources(&player_bld.player_id, &bld_lvl)? {
            return Err(Error::from((
                ErrorKind::UpgradeBuildingError,
                "Not enough resources",
            )));
        }

        // check for max level constraints
        if player_bld.level >= max_level.unwrap_or(0) {
            return Err(Error::from((
                ErrorKind::UpgradeBuildingError,
                "Building is at max level",
            )));
        }

        let mut conn = self.db_pool.get()?;
        let res: Result<PlayerBuilding> = conn.transaction(|connection| {
            info!("Initiating upgrade transaction");
            // deduct resources
            self.res_repo.deduct(
                &player_bld.player_id,
                &(
                    bld_lvl.req_food.unwrap_or(0),
                    bld_lvl.req_wood.unwrap_or(0),
                    bld_lvl.req_stone.unwrap_or(0),
                    bld_lvl.req_gold.unwrap_or(0),
                ),
            )?;
            // upgrade building
            let player_bld = self
                .player_bld_repo
                .set_upgrade_time(player_bld_id, Some(bld_lvl.upgrade_time.as_str()))?;
            info!("Building upgrade started: {:?}", player_bld);
            Ok(player_bld)
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
    pub fn confirm_upgrade(&mut self, id: &buildings::PlayerBuildingKey) -> Result<()> {
        let player_bld = self.player_bld_repo.get_by_id(id)?;
        match player_bld.upgrade_time {
            None => Err(Error::from((
                ErrorKind::ConfirmUpgradeError,
                "Building is not upgrading",
            ))),
            Some(t) => {
                let time = DateTime::<Utc>::from_str(t.as_str()).map_err(|_| {
                    Error::from((ErrorKind::ConfirmUpgradeError, "Invalid time format"))
                })?;
                if time <= Utc::now() {
                    self.player_bld_repo.inc_level(id)?;
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
        player_id: &PlayerKey,
        bld_lvl: &BuildingLevel,
    ) -> Result<bool> {
        debug!("Checking resources for player: {}", player_id);
        let res = self.res_repo.get_by_id(player_id)?;
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
