// while in development, ignore dead code and unused variables warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::db::conn::get_connection_pool;
use crate::db::migrations::run_migrations;
use anyhow::Result;
use empire::setup_tracing;
use tracing::info;

mod db;
mod game;
mod models;
mod net;
mod rpc;
mod schema;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    setup_tracing()?;
    info!("Starting Empire server...");

    let pool = get_connection_pool();
    let mut conn = pool
        .get()
        .expect("Connection pool should return a connection");

    run_migrations(&mut conn).expect("Should execute pending migrations");

    Ok(())
}
