use std::fmt;

use diesel::prelude::*;
use diesel::sql_types::Int4;
use diesel::QueryDsl;
use tracing::{debug, instrument};

use crate::db::resources::ResourcesRepository;
use crate::db::DbConn;
use crate::domain::resource::UserResource;
use crate::domain::user;
use crate::schema::{user_accumulator as acc, user_resources as rsc};
use crate::Result;

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
    pub fn collect_resources(&mut self, user_id: &user::PK) -> Result<UserResource> {
        let (food, wood, stone, gold) = acc::table
            .inner_join(rsc::table.on(acc::user_id.eq(rsc::user_id)))
            .filter(acc::user_id.eq(user_id))
            .select((
                least(acc::food, rsc::food_cap - rsc::food),
                least(acc::wood, rsc::wood_cap - rsc::wood),
                least(acc::stone, rsc::stone_cap - rsc::stone),
                least(acc::gold, rsc::gold_cap - rsc::gold),
            ))
            .first::<(i32, i32, i32, i32)>(&mut self.connection)?;
        debug!("Deltas: {:?}", (food, wood, stone, gold));

        let res: Result<UserResource> = self.connection.transaction(|conn| {
            // drain accumulator first
            let updated_rows = diesel::update(acc::table.filter(acc::user_id.eq(user_id)))
                .set((
                    acc::food.eq(acc::food - food),
                    acc::wood.eq(acc::wood - wood),
                    acc::stone.eq(acc::stone - stone),
                    acc::gold.eq(acc::gold - gold),
                ))
                .execute(conn)?;

            // then increase resources
            let res = diesel::update(rsc::table.filter(rsc::user_id.eq(user_id)))
                .set((
                    rsc::food.eq(rsc::food + food),
                    rsc::wood.eq(rsc::wood + wood),
                    rsc::stone.eq(rsc::stone + stone),
                    rsc::gold.eq(rsc::gold + gold),
                ))
                .returning(UserResource::as_returning())
                .get_result(conn)?;

            Ok(res)
        });

        res
    }
}
