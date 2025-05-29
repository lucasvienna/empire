use std::env;

use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use secrecy::ExposeSecret;
use tracing::{debug, error, info, instrument, trace};

use crate::configuration::DatabaseSettings;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

/// Initializes the database connection pool using the provided `DatabaseSettings`.
///
/// If a `pool_size` is specified in the settings, it will create a pool with the given size.
/// Otherwise, it defaults to creating a pool with no size limit.
///
/// Exits the process if the pool cannot be created.
#[instrument(skip(settings))]
pub fn initialize_pool(settings: &DatabaseSettings) -> DbPool {
    debug!("Initializing database pool...");
    trace!(
        "Connecting to {:#?}",
        format!(
            "postgres://{}:{:?}@{}:{}/{}",
            settings.username,
            settings.password,
            settings.host,
            settings.port,
            settings.database_name
        )
    );
    let pool = match settings.pool_size {
        Some(size) => {
            debug!("Creating connection pool with size: {}", size);
            create_pool_with_size(settings.connection_string().expose_secret(), Some(size))
        }
        None => {
            debug!("Creating connection pool with default size");
            create_pool(settings.connection_string().expose_secret())
        }
    };
    pool.unwrap_or_else(|err| {
        error!("Failed to initialize database pool: {}", err);
        std::process::exit(1);
    })
}

/// Initializes a database connection pool using environment variables.
///
/// This function optionally loads environment variables from a specified file.
/// It retrieves the `DATABASE_URL` from the environment and uses it to
/// establish the connection pool. If the environment variable or connection
/// pool creation fails, the process will exit with an error message.
///
/// # Arguments
///
/// * `filename` - An optional path to a `.env` file to load environment variables from.
///
/// # Panics
///
/// This function will terminate and exit the process if the `DATABASE_URL` is missing
/// or the connection pool initialization fails.
#[instrument]
pub fn initialize_pool_from_env(filename: Option<&str>) -> DbPool {
    debug!("Creating connection from environment variables");
    load_environment(filename);
    let database_url = env::var_os("DATABASE_URL").unwrap_or_else(|| "".into());
    create_pool(database_url.to_string_lossy()).unwrap_or_else(|err| {
        error!("Failed to initialize database pool: {}", err);
        std::process::exit(1);
    })
}

/// Creates a new database pool using the provided database URL.
fn create_pool<S: Into<String>>(database_url: S) -> Result<DbPool> {
    create_pool_with_size(database_url, None)
}

/// Creates a new database pool with an optional pool size.
fn create_pool_with_size<S: Into<String>>(
    database_url: S,
    pool_size: Option<usize>,
) -> Result<DbPool> {
    assert_ne!(pool_size, Some(0), "r2d2 pool size must be greater than 0");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let builder = Pool::builder().test_on_check_out(true);
    let pool = match pool_size {
        Some(size) => builder.max_size(size as u32).build(manager)?,
        None => builder.build(manager)?,
    };
    debug!("Connection pool created: {:?}", pool.state());
    info!("Database pool ready");

    Ok(pool)
}

/// Loads environment variables from the specified filename or defaults.
fn load_environment(filename: Option<&str>) {
    match filename {
        Some(file) => dotenvy::from_filename(file).ok(),
        None => dotenvy::dotenv().ok(),
    };
}
