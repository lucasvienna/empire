use diesel::prelude::*;
use diesel::sql_types::Int4;
use diesel::QueryDsl;
use std::fmt;
use tracing::{debug, instrument};

use crate::db::resources::ResourcesRepository;
use crate::db::DbConn;
use crate::domain::resource::Resource;
use crate::domain::user;
use crate::schema::resources as rr;
use crate::schema::resources_accumulator as ra;
use crate::Result;

#[derive(Debug)]
pub enum ResourceType {
    Food,
    Wood,
    Stone,
    Gold,
}

pub struct ResourceService {
    connection: DbConn,
    res_repo: ResourcesRepository,
}

impl fmt::Debug for ResourceService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResourceService")
    }
}

define_sql_function! {
    #[sql_name = "LEAST"]
    fn least(a: Int4, b: Int4) -> Int4
}

impl ResourceService {
    pub fn new(connection: DbConn) -> ResourceService {
        ResourceService {
            connection,
            res_repo: ResourcesRepository {},
        }
    }

    /// Collects resources for a user by transferring the maximum possible amount from their
    /// resource accumulator to their resource storage, constrained by the storage capacity limits.
    ///
    /// # Parameters
    ///
    /// - `user_id`: Reference to the primary key of the user whose resources will be collected.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the updated `Resource` structure representing the user's
    /// updated resources or an error if the operation fails.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following conditions occur:
    /// - The database query to calculate the collectible resource amounts fails.
    /// - The transaction to update both the accumulator and the resource storage fails.
    #[instrument(skip(self))]
    pub fn collect_resources(&mut self, user_id: &user::PK) -> Result<Resource> {
        let (food, wood, stone, gold) = ra::table
            .inner_join(rr::table.on(ra::user_id.eq(rr::user_id)))
            .filter(ra::user_id.eq(user_id))
            .select((
                least(ra::food, rr::food_cap - rr::food),
                least(ra::wood, rr::wood_cap - rr::wood),
                least(ra::stone, rr::stone_cap - rr::stone),
                least(ra::gold, rr::gold_cap - rr::gold),
            ))
            .first::<(i32, i32, i32, i32)>(&mut self.connection)?;
        debug!("Deltas: {:?}", (food, wood, stone, gold));

        let res: Result<Resource> = self.connection.transaction(|conn| {
            // drain accumulator first
            let updated_rows = diesel::update(ra::table.filter(ra::user_id.eq(user_id)))
                .set((
                    ra::food.eq(ra::food - food),
                    ra::wood.eq(ra::wood - wood),
                    ra::stone.eq(ra::stone - stone),
                    ra::gold.eq(ra::gold - gold),
                ))
                .execute(conn)?;

            // then increase resources
            let res = diesel::update(rr::table.filter(rr::user_id.eq(user_id)))
                .set((
                    rr::food.eq(rr::food + food),
                    rr::wood.eq(rr::wood + wood),
                    rr::stone.eq(rr::stone + stone),
                    rr::gold.eq(rr::gold + gold),
                ))
                .returning(Resource::as_returning())
                .get_result(conn)?;

            Ok(res)
        });

        res
    }
}
