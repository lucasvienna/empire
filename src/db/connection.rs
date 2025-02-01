use std::env;
use std::fmt::Display;

use crate::configuration::{get_configuration, DatabaseSettings};
use crate::db::migrations::run_migrations;
use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{sql_query, Connection, PgConnection, RunQueryDsl};
use tracing::{debug, info};
use uuid::Uuid;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

fn new<S: Display>(database_url: S) -> Result<DbPool> {
    new_with_pool_size(database_url, None)
}

pub fn new_from_settings(settings: DatabaseSettings) -> Result<DbPool> {
    new_with_pool_size(settings.connection_string(), settings.pool_size)
}

pub fn new_from_env(filename: Option<&str>) -> Result<DbPool> {
    debug!(
        "Creating connection with environment variables from: {:?}",
        filename
    );
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
    debug!("Connection pool created. {:#?}", pool.state());

    Ok(pool)
}

pub fn get_pool(settings: &DatabaseSettings) -> DbPool {
    match settings.pool_size {
        Some(pool_size) => {
            debug!("Creating connection pool with size: {}", pool_size);
            new_with_pool_size(settings.connection_string(), Some(pool_size)).unwrap()
        }
        None => {
            debug!("Creating connection pool with default size");
            new(settings.connection_string()).unwrap()
        }
    }
}

pub fn get_test_pool() -> DbPool {
    let mut config = get_configuration()
        .expect("Failed to read configuration.")
        .database;
    config.database_name = Uuid::now_v7().to_string();

    let db_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: "password".to_string(),
        pool_size: Some(1),
        ..config
    };

    let mut conn = PgConnection::establish(&db_settings.connection_string())
        .expect("Failed to connect to database.");

    sql_query(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .execute(&mut conn)
        .expect("Failed to create test schema");

    run_migrations(&mut conn).expect("Failed to run migrations.");

    new_from_settings(db_settings).unwrap()
}
