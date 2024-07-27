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
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    log::info!("Starting Empire server...");

    let pool = get_connection_pool();
    let mut conn = pool.get().unwrap();
    let res = run_migrations(&mut conn);
    res.expect("Should execute pending migrations");
}
