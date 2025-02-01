use std::env;
use std::fmt::Display;

use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use tracing::info;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

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
        assert_ne!(pool_size, Some(0), "r2d2 pool size must be greater than 0");

        let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
        let builder = Pool::builder().test_on_check_out(true);

        let pool = match pool_size {
            Some(pool_size) => builder
                .max_size(pool_size)
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
}

pub fn get_env_pool() -> DbPool {
    let executor = DbExecutor::from_env();
    executor.unwrap().pool
}
pub fn get_test_pool() -> DbPool {
    DbExecutor::from_env().unwrap().pool
}
