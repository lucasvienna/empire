use std::time::Duration;

use diesel::sql_types::Integer;
use diesel::RunQueryDsl;
use tokio::task::JoinHandle;
use tokio::{task, time};
use tracing::{debug, error, info, instrument};

use crate::db::DbConn;
use crate::game::TICK_RATE;

pub static RES_GEN_QUERY: &str = "UPDATE resources_accumulator acc
    SET food  = GREATEST(acc.food + rg.food / $1, rg.food_acc_cap),
        wood  = GREATEST(acc.wood + rg.wood / $1, rg.wood_acc_cap),
        stone = GREATEST(acc.stone + rg.stone / $1, rg.stone_acc_cap),
        gold  = GREATEST(acc.gold + rg.gold / $1, rg.gold_acc_cap)
    FROM resource_generation rg
    WHERE acc.user_id = rg.user_id;";

/// Initializes a background task responsible for periodic resource generation.
///
/// This function runs an asynchronous task that periodically updates the resource values
/// in the database. The resource amounts are incremented by values from the `resource_generation`
/// view, using a SQL query to perform batch updates for all users.
///
/// # Parameters
/// - `conn`: A database connection object (`DbConn`) used to execute the SQL query.
///
/// # Returns
/// A `JoinHandle` representing the spawned asynchronous task. The task will continue to run
/// until the application shuts down or the `JoinHandle` is dropped.
///
/// # Behavior
/// - Updates the `resources` table every 60 seconds.
/// - Increments the `food`, `wood`, `stone`, and `gold` columns for each user by their
///   respective resource generation rates stored in the `resource_generation` table.
/// - Logs any errors encountered during the update operation.
/// - Logs the number of users whose resources were updated successfully.
#[instrument(skip(conn))]
pub fn init_res_gen(mut conn: DbConn) -> JoinHandle<()> {
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(TICK_RATE as u64));
        // 3600/x gives the number of ticks in one hour, so 60s -> 60 ticks, 30s -> 120 ticks and so on
        // the rate is per hour on the DB, so dividing that by this quotient gives the exact number of
        // generated resources per tick
        let gen_quot = 3600 / TICK_RATE;
        loop {
            interval.tick().await;

            info!("Incrementing resources...");
            let updated = diesel::sql_query(RES_GEN_QUERY)
                .bind::<Integer, _>(gen_quot)
                .execute(&mut conn)
                .inspect_err(|err| error!("Failed to generate resources: {}", err.to_string()))
                .unwrap_or_default();
            debug!("Incremented resources for {} users", updated);
        }
    })
}
