use std::env;
use std::fmt::Display;

use anyhow::Result;
use diesel::query_dsl::RunQueryDsl;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, CustomizeConnection, Pool};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use tracing::info;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
struct Customizer;

impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for Customizer {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        // see https://fractaledmind.github.io/2023/09/07/enhancing-rails-sqlite-fine-tuning/
        // sleep if the database is busy
        // this corresponds to 2 seconds
        // if we ever see errors regarding busy_timeout in production
        // we might want to consider to increase this time
        diesel::sql_query("PRAGMA busy_timeout = 2000;")
            .execute(conn)
            .map_err(r2d2::Error::QueryError)?;
        // better write-concurrency
        diesel::sql_query("PRAGMA journal_mode = WAL;")
            .execute(conn)
            .map_err(r2d2::Error::QueryError)?;
        // fsync only in critical moments
        diesel::sql_query("PRAGMA synchronous = NORMAL;")
            .execute(conn)
            .map_err(r2d2::Error::QueryError)?;
        // write WAL changes back every 1000 pages, for an in average 1MB WAL file. May affect readers if number is increased
        diesel::sql_query("PRAGMA wal_autocheckpoint = 1000;")
            .execute(conn)
            .map_err(r2d2::Error::QueryError)?;
        // free some space by truncating possibly massive WAL files from the last run
        diesel::sql_query("PRAGMA wal_checkpoint(TRUNCATE);")
            .execute(conn)
            .map_err(r2d2::Error::QueryError)?;
        // maximum size of the WAL file, corresponds to 64MB
        diesel::sql_query("PRAGMA journal_size_limit = 67108864;")
            .execute(conn)
            .map_err(r2d2::Error::QueryError)?;
        // maximum size of the internal mmap pool. Corresponds to 128MB, matches postgres default settings
        diesel::sql_query("PRAGMA mmap_size = 134217728;")
            .execute(conn)
            .map_err(r2d2::Error::QueryError)?;
        // maximum number of database disk pages that will be hold in memory. Corresponds to ~8MB
        diesel::sql_query("PRAGMA cache_size = 2000;")
            .execute(conn)
            .map_err(r2d2::Error::QueryError)?;
        //enforce foreign keys
        diesel::sql_query("PRAGMA foreign_keys = ON;")
            .execute(conn)
            .map_err(r2d2::Error::QueryError)?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct DbExecutor {
    pub pool: DbPool,
}

impl DbExecutor {
    pub fn new<S: Display>(database_url: S) -> Result<Self> {
        DbExecutor::new_with_pool_size(database_url, None)
    }

    fn new_with_pool_size<S: Display>(database_url: S, pool_size: Option<u32>) -> Result<Self> {
        let database_url = format!("{}", database_url);
        info!("Connecting to database at: {}", database_url);

        let manager = ConnectionManager::<SqliteConnection>::new(database_url.clone());
        let builder = Pool::builder()
            .connection_customizer(Box::new(Customizer))
            .test_on_check_out(true);

        let pool = match pool_size {
            // Sqlite doesn't handle connections from multiple threads well.
            Some(pool_size) => builder
                .max_size(pool_size)
                .idle_timeout(None)
                .max_lifetime(None)
                .build(manager)?,
            None => builder.build(manager)?,
        };

        Ok(DbExecutor { pool })
    }

    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        let database_url = env::var_os("DATABASE_URL").unwrap_or_else(|| "".into());
        Self::new(database_url.to_string_lossy())
    }

    pub fn in_memory(name: &str) -> Result<Self> {
        Self::new_with_pool_size(format!("file:{}?mode=memory&cache=shared", name), Some(1))
    }
}

pub fn get_env_pool() -> DbPool {
    let executor = DbExecutor::from_env();
    executor.unwrap().pool
}
