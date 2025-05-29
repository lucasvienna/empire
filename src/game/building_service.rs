use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::FromRef;
use chrono::prelude::*;
use diesel::Connection;
use tracing::{debug, info, instrument, trace, warn};

use crate::db::building_levels::BuildingLevelRepository;
use crate::db::buildings::BuildingRepository;
use crate::db::player_buildings::PlayerBuildingRepository;
use crate::db::resources::ResourcesRepository;
use crate::db::Repository;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::building::level::BuildingLevel;
use crate::domain::building::BuildingKey;
use crate::domain::error::{Error, ErrorKind, Result};
use crate::domain::player::buildings::{NewPlayerBuilding, PlayerBuilding, PlayerBuildingKey};
use crate::domain::player::PlayerKey;
use crate::game::service::ApiService;

pub struct BuildingService {
    db_pool: AppPool,
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

impl FromRef<AppState> for BuildingService {
    fn from_ref(state: &AppState) -> Self {
        BuildingService::new(&state.db_pool)
    }
}

impl ApiService for BuildingService {
    fn new(pool: &AppPool) -> Self {
        BuildingService {
            db_pool: Arc::clone(pool),
            bld_repo: BuildingRepository::new(pool),
            player_bld_repo: PlayerBuildingRepository::new(pool),
            bld_lvl_repo: BuildingLevelRepository::new(pool),
            res_repo: ResourcesRepository::new(pool),
        }
    }
}

impl BuildingService {
    #[instrument(skip(self))]
    pub fn construct_building(
        &self,
        player_id: &PlayerKey,
        bld_id: &BuildingKey,
    ) -> Result<PlayerBuilding> {
        debug!(
            "Starting construct building {} for player {}",
            bld_id, player_id
        );
        let mut conn = self.db_pool.get()?;
        let bld_lvl = self.bld_lvl_repo.get_next_upgrade(&mut conn, bld_id, &0)?;
        trace!("Building level requirements: {:?}", bld_lvl);

        // check for resources
        if !self.has_enough_resources(player_id, &bld_lvl)? {
            trace!(
                "Player {} doesn't have enough resources to build {}",
                player_id,
                bld_id
            );
            return Err(Error::from((
                ErrorKind::ConstructBuildingError,
                "Not enough resources",
            )));
        }

        // verify if the maximum number of buildings was reached
        if !self
            .player_bld_repo
            .can_construct(&mut conn, player_id, bld_id)?
        {
            trace!(
                "Player {} has reached max buildings for building #{}",
                player_id,
                bld_id
            );
            return Err(Error::from((
                ErrorKind::ConstructBuildingError,
                "Max buildings reached",
            )));
        }

        let res: Result<PlayerBuilding> = conn.transaction(|connection| {
            info!("Initiating construction transaction");
            // deduct resources
            self.res_repo.deduct(
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
            let player_bld = self.player_bld_repo.construct(
                connection,
                NewPlayerBuilding {
                    player_id: *player_id,
                    building_id: *bld_id,
                    level: Some(0),
                    upgrade_time: Some(bld_lvl.upgrade_time),
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

    #[instrument(skip(self))]
    pub fn upgrade_building(&self, player_bld_id: &PlayerBuildingKey) -> Result<PlayerBuilding> {
        debug!("Starting upgrade building: {}", player_bld_id);
        let mut conn = self.db_pool.get()?;
        let (player_bld, max_level) = self
            .player_bld_repo
            .get_upgrade_tuple(&mut conn, player_bld_id)?;
        trace!(
            "Player building details: {:?}, max level: {:?}",
            player_bld,
            max_level
        );
        let bld_lvl = self.bld_lvl_repo.get_next_upgrade(
            &mut conn,
            &player_bld.building_id,
            &player_bld.level,
        )?;
        trace!("Next building level details: {:?}", bld_lvl);

        // check for resources
        if !self.has_enough_resources(&player_bld.player_id, &bld_lvl)? {
            debug!(
                "Player {} doesn't have enough resources for upgrade",
                player_bld.player_id
            );
            return Err(Error::from((
                ErrorKind::UpgradeBuildingError,
                "Not enough resources",
            )));
        }

        // check for max level constraints
        if player_bld.level >= max_level.unwrap_or(0) {
            debug!(
                "Building {} is already at max level {} for player {}",
                player_bld.building_id, player_bld.level, player_bld.player_id
            );
            return Err(Error::from((
                ErrorKind::UpgradeBuildingError,
                "Building is at max level",
            )));
        }

        let res: Result<PlayerBuilding> = conn.transaction(|connection| {
            info!("Initiating upgrade transaction");
            // deduct resources
            self.res_repo.deduct(
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
            let player_bld = self.player_bld_repo.set_upgrade_time(
                connection,
                player_bld_id,
                Some(bld_lvl.upgrade_time.as_str()),
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

    #[instrument(skip(self))]
    pub fn confirm_upgrade(&self, id: &PlayerBuildingKey) -> Result<()> {
        debug!("Starting confirm upgrade for building {}", id);
        let mut conn = self.db_pool.get()?;
        let player_bld = self.player_bld_repo.get_by_id(id)?;
        trace!("Player building details: {:?}", player_bld);
        match player_bld.upgrade_time {
            None => {
                debug!("Building {} is not in upgrading state", id);
                Err(Error::from((
                    ErrorKind::ConfirmUpgradeError,
                    "Building is not upgrading",
                )))
            }
            Some(t) => {
                let time = DateTime::<Utc>::from_str(t.as_str()).map_err(|_| {
                    Error::from((ErrorKind::ConfirmUpgradeError, "Invalid time format"))
                })?;
                if time <= Utc::now() {
                    debug!("Upgrade time has passed, incrementing building level");
                    self.player_bld_repo.inc_level(&mut conn, id)?;
                    info!("Successfully confirmed upgrade for building {}", id);
                    Ok(())
                } else {
                    debug!(
                        "Upgrade time has not passed yet: current={}, upgrade_time={}",
                        Utc::now(),
                        time
                    );
                    Err(Error::from((
                        ErrorKind::ConfirmUpgradeError,
                        "Upgrade time has not passed",
                    )))
                }
            }
        }
    }

    #[instrument(skip(self, bld_lvl))]
    fn has_enough_resources(&self, player_id: &PlayerKey, bld_lvl: &BuildingLevel) -> Result<bool> {
        debug!("Checking resources for player: {}", player_id);
        trace!(
            "Required resources: food={}, wood={}, stone={}, gold={}",
            bld_lvl.req_food.unwrap_or(0),
            bld_lvl.req_wood.unwrap_or(0),
            bld_lvl.req_stone.unwrap_or(0),
            bld_lvl.req_gold.unwrap_or(0)
        );
        let res = self.res_repo.get_by_player_id(player_id)?;
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
