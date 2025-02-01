use crate::configuration::DatabaseSettings;
use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;
use std::fmt::Display;
use tracing::{debug, info};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Creates a new database pool using the provided database URL.
fn create_pool<S: Display>(database_url: S) -> Result<DbPool> {
    create_pool_with_size(database_url, None)
}

/// Creates a new database pool from the given `DatabaseSettings`.
pub fn create_pool_from_settings(settings: DatabaseSettings) -> Result<DbPool> {
    create_pool_with_size(settings.connection_string(), settings.pool_size)
}

/// Creates a new database pool using environment variables.
/// Optionally loads variables from a specified filename.
pub fn create_pool_from_env(filename: Option<&str>) -> Result<DbPool> {
    debug!(
        "Creating connection with environment variables from: {:?}",
        filename
    );
    load_environment(filename);
    let database_url = env::var_os("DATABASE_URL").unwrap_or_else(|| "".into());
    create_pool(database_url.to_string_lossy())
}

/// Creates a new database pool with an optional pool size.
fn create_pool_with_size<S: Display>(database_url: S, pool_size: Option<u32>) -> Result<DbPool> {
    assert_ne!(pool_size, Some(0), "r2d2 pool size must be greater than 0");

    let database_url = format!("{}", database_url);
    info!("Connecting to database at: {}", database_url);

    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    let builder = Pool::builder().test_on_check_out(true);
    let pool = match pool_size {
        Some(size) => builder.max_size(size).build(manager)?,
        None => builder.build(manager)?,
    };

    debug!("Connection pool created. {:#?}", pool.state());
    Ok(pool)
}

/// Initializes the database pool based on the provided settings.
pub fn initialize_pool(settings: &DatabaseSettings) -> DbPool {
    match settings.pool_size {
        Some(size) => {
            debug!("Creating connection pool with size: {}", size);
            create_pool_with_size(settings.connection_string(), Some(size)).unwrap()
        }
        None => {
            debug!("Creating connection pool with default size");
            create_pool(settings.connection_string()).unwrap()
        }
    }
}

/// Loads environment variables from the specified filename or defaults.
fn load_environment(filename: Option<&str>) {
    match filename {
        Some(file) => dotenvy::from_filename(file).ok(),
        None => dotenvy::dotenv().ok(),
    };
}
