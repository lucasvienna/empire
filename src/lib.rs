// while in development, ignore dead code and unused variables warnings
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod configuration;
pub mod controllers;
pub mod custom_schema;
pub mod db;
pub mod domain;
pub mod game;
pub mod job_queue;
pub mod net;
pub mod schema;
pub mod services;
pub mod startup;
pub mod telemetry;

// re-export for ease of use in other private crates
pub use domain::error::{Error, ErrorKind, Result};
