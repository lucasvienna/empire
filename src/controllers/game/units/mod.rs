//! Units controller module for unit training operations.
//!
//! Provides REST API endpoints for:
//! - Listing available units for training at a building
//! - Starting unit training
//! - Viewing the training queue with progress
//! - Cancelling training with resource refunds
//! - Viewing the player's unit inventory

mod handlers;
mod models;
mod routes;

pub use routes::*;
