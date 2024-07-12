use std::env;
use std::fmt::Display;

use anyhow::Result;
use diesel::connection::SimpleConnection;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, CustomizeConnection, Pool};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use log;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
struct Customizer;

impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for Customizer {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        conn.batch_execute(
            "\
                    PRAGMA journal_mode = WAL;\
                    PRAGMA busy_timeout = 1000;\
                    PRAGMA foreign_keys = ON;\
                ",
        )
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
        log::info!("using database at: {}", database_url);

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

pub fn get_connection_pool() -> DbPool {
    let executor = DbExecutor::from_env();
    executor.unwrap().pool
}
