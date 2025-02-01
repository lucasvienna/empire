use std::env;
use std::fmt::Display;

use crate::configuration::DatabaseSettings;
use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use tracing::{debug, info};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

fn new<S: Display>(database_url: S) -> Result<DbPool> {
    new_with_pool_size(database_url, None)
}

pub fn new_from_env(filename: Option<&str>) -> Result<DbPool> {
    match filename {
        Some(filename) => dotenvy::from_filename(filename).ok(),
        None => dotenvy::dotenv().ok(),
    };

    let database_url = env::var_os("DATABASE_URL").unwrap_or_else(|| "".into());
    new(database_url.to_string_lossy())
}

fn new_with_pool_size<S: Display>(database_url: S, pool_size: Option<u32>) -> Result<DbPool> {
    assert_ne!(pool_size, Some(0), "r2d2 pool size must be greater than 0");

    let database_url = format!("{}", database_url);
    info!("Connecting to database at: {}", database_url);

    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    let builder = Pool::builder().test_on_check_out(true);

    let pool = match pool_size {
        Some(pool_size) => builder.max_size(pool_size).build(manager)?,
        None => builder.build(manager)?,
    };

    Ok(pool)
}

pub fn get_pool(settings: DatabaseSettings) -> DbPool {
    match settings.pool_size {
        Some(pool_size) => {
            debug!("Creating connection pool with size: {}", pool_size);
            new_with_pool_size(settings.connection_string(), Some(pool_size)).unwrap()
        }
        None => new_from_env(None).unwrap(),
    }
}

pub fn get_test_pool() -> DbPool {
    new_from_env(Some(".env.test")).unwrap()
}
