use std::fmt;
use std::sync::Arc;

use axum::extract::FromRef;
use diesel::prelude::*;
use diesel::sql_types::Int4;
use diesel::QueryDsl;
use tracing::{debug, instrument};

use crate::db::resources::ResourcesRepository;
use crate::db::Repository;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::player::resource::PlayerResource;
use crate::domain::player::PlayerKey;
use crate::game::service::ApiService;
use crate::schema::{player_accumulator as acc, player_resource as rsc};
use crate::Result;

define_sql_function! {
    #[sql_name = "LEAST"]
    fn least(a: Int4, b: Int4) -> Int4
}

pub struct ResourceService {
    pool: AppPool,
    res_repo: ResourcesRepository,
}

impl fmt::Debug for ResourceService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResourceService")
    }
}

impl FromRef<AppState> for ResourceService {
    fn from_ref(state: &AppState) -> Self {
        ResourceService::new(&state.db_pool)
    }
}

impl ApiService for ResourceService {
    fn new(pool: &AppPool) -> Self {
        ResourceService {
            pool: Arc::clone(pool),
            res_repo: ResourcesRepository::new(pool),
        }
    }
}

impl ResourceService {
    /// Collects resources for a player by transferring the maximum possible amount from their
    /// resource accumulator to their resource storage, constrained by the storage capacity limits.
    ///
    /// # Parameters
    ///
    /// - `player_id`: Reference to the primary key of the player whose resources will be collected.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the updated `Resource` structure representing the player's
    /// updated resources or an error if the operation fails.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following conditions occur:
    /// - The database query to calculate the collectible resource amounts fails.
    /// - The transaction to update both the accumulator and the resource storage fails.
    #[instrument(skip(self))]
    pub fn collect_resources(&self, player_id: &PlayerKey) -> Result<PlayerResource> {
        let mut connection = self.pool.get()?;
        let (food, wood, stone, gold) = acc::table
            .inner_join(rsc::table.on(acc::player_id.eq(rsc::player_id)))
            .filter(acc::player_id.nullable().eq(player_id))
            .select((
                least(acc::food, rsc::food_cap - rsc::food),
                least(acc::wood, rsc::wood_cap - rsc::wood),
                least(acc::stone, rsc::stone_cap - rsc::stone),
                least(acc::gold, rsc::gold_cap - rsc::gold),
            ))
            .first::<(i32, i32, i32, i32)>(&mut connection)?;
        debug!("Deltas: {:?}", (food, wood, stone, gold));

        let res: Result<PlayerResource> = connection.transaction(|conn| {
            // drain accumulator first
            let updated_rows = diesel::update(acc::table.filter(acc::player_id.eq(player_id)))
                .set((
                    acc::food.eq(acc::food - food),
                    acc::wood.eq(acc::wood - wood),
                    acc::stone.eq(acc::stone - stone),
                    acc::gold.eq(acc::gold - gold),
                ))
                .execute(conn)?;

            // then increase resources
            let res = diesel::update(rsc::table.filter(rsc::player_id.eq(player_id)))
                .set((
                    rsc::food.eq(rsc::food + food),
                    rsc::wood.eq(rsc::wood + wood),
                    rsc::stone.eq(rsc::stone + stone),
                    rsc::gold.eq(rsc::gold + gold),
                ))
                .returning(PlayerResource::as_returning())
                .get_result(conn)?;

            Ok(res)
        });

        res
    }
}
