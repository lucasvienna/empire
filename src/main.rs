#[macro_use]
extern crate rocket;

use log4rs;

use crate::data::hydrate::initialize_database;
use crate::db::conn::get_connection_pool;
use crate::db::migrations::run_migrations;

mod data;
mod db;
mod game;
mod models;
mod net;
mod rpc;
mod schema;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    log::info!("Starting Empire server...");

    let pool = get_connection_pool();
    let mut conn = pool.get().unwrap();
    let res = run_migrations(&mut conn);
    res.expect("Should execute pending migrations");
    initialize_database(&mut conn).expect("Should initialize database");

    rpc::receiver::start().expect("Should start receiver");

    // let server = net::server::start().launch();
}
