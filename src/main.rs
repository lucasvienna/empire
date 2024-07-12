use log4rs;

use crate::data::hydrate::initialize_database;
use crate::db::conn::get_connection_pool;
use crate::db::migrations::run_migrations;

mod data;
mod db;
mod models;
mod schema;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    log::info!("Starting Empire server...");

    let pool = get_connection_pool();
    let res = run_migrations(&mut pool.get().unwrap());
    res.expect("Should execute pending migrations");
    initialize_database(&mut pool.get().unwrap());
}
