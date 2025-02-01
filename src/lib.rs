// while in development, ignore dead code and unused variables warnings
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod configuration;
pub mod controllers;
pub mod db;
pub mod game;
pub mod models;
pub mod net;
pub mod schema;
pub mod startup;
pub mod telemetry;

// re-export for ease of use in other private crates
pub use models::error::{Error, ErrorKind, Result};
