// while in development, ignore dead code and unused variables warnings
#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate rocket;

use log4rs;

use crate::db::conn::get_connection_pool;
use crate::db::migrations::run_migrations;

mod db;
mod game;
mod models;
mod net;
mod rpc;
mod schema;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default())
        .expect("Logger should be initialized from file");
    log::debug!(
        "Starting Empire server with log level: {}",
        log::max_level()
    );
    log::info!("Starting Empire server...");

    let pool = get_connection_pool();
    let mut conn = pool
        .get()
        .expect("Connection pool should return a connection");

    run_migrations(&mut conn).expect("Should execute pending migrations");

}
