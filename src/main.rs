#[macro_use]
extern crate rocket;

use log4rs;

use crate::data::hydrate::initialize_database;
use crate::db::conn::get_connection_pool;
use crate::db::migrations::run_migrations;

pub mod data;
pub mod db;
pub mod models;
pub mod net;
pub mod rpc;
pub mod schema;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    log::info!("Starting Empire server...");

    let pool = get_connection_pool();
    let res = run_migrations(&mut pool.get().unwrap());
    res.expect("Should execute pending migrations");
    initialize_database(&mut pool.get().unwrap());

    rpc::receiver::start().expect("Should start receiver");

    // let server = net::server::start().launch();
}
