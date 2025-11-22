//! Module for handling building availability and requirement checks.
//!
//! This module provides functionality to determine which buildings are available
//! for construction based on requirements and restrictions like building levels,
//! tech nodes, and maximum counts.

use std::collections::HashMap;

use serde::Serialize;
use uuid::Uuid;

use crate::domain::building::requirement::BuildingRequirement;
use crate::domain::building::{Building, BuildingKey};

/// Represents the availability status of a building, including build restrictions and current state
#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
pub struct BuildingAvailability {
	/// The building definition
	building: Building,
	/// Whether the building can currently be constructed
	buildable: bool,
	/// Number of instances of this building currently owned
	current_count: i64,
	/// Maximum number of instances allowed
	max_count: i32,
	/// List of restrictions preventing construction
	locks: Vec<BuildingLock>,
}

/// Represents different types of restrictions that can prevent a building from being constructed
#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum BuildingLock {
	/// Maximum allowed number of building instances has been reached
	MaxCountReached,
	/// Another building needs to reach a specific level
	BuildingLevelRequired {
		/// The building that needs upgrading
		building: BuildingKey,
		/// Current level of the required building
		current: i32,
		/// Level that needs to be reached
		required: i32,
	},
	/// A specific technology node needs to be researched
	TechNodeRequired {
		/// ID of the required technology node
		node_id: Uuid,
	},
}

/// Represents the count of buildings constructed
type BuildingCount = i64;
/// Represents the maximum number of instances of the same building the player can build
type MaxBuildingCount = i32;
/// Represents the maximum level a building can reach
type MaxBuildingLevel = i32;
/// Tuple containing current count, maximum count and maximum level for a building
pub type AvailabilityData = (BuildingCount, MaxBuildingCount, Option<MaxBuildingLevel>);

/// Generates an availability map for buildings based on their current counts and requirements
///
/// # Arguments
/// * `blds` - Map of building definitions by their keys
/// * `bld_data` - Map containing current counts and maximum limits for each building
/// * `requirements` - Map of building requirements indexed by building keys
///
/// # Returns
/// Vector of [BuildingAvailability] containing availability status for each building
pub fn gen_avail_list(
	mut blds: HashMap<BuildingKey, Building>,
	bld_data: HashMap<BuildingKey, AvailabilityData>,
	requirements: HashMap<BuildingKey, Vec<BuildingRequirement>>,
) -> Vec<BuildingAvailability> {
	bld_data
		.into_iter()
		.map(|(bld_key, (owned_count, max_count, max_level))| {
			let mut locks: Vec<BuildingLock> = vec![];

			// Check if max count is reached
			if owned_count >= (max_count as i64) {
				locks.push(BuildingLock::MaxCountReached);
			}

			// Check requirement locks
			if let Some(reqs) = requirements.get(&bld_key) {
				locks.extend(reqs.iter().filter_map(|req| {
					parse_req_lock(&bld_key, req, max_level.unwrap_or_default())
				}));
			}

			BuildingAvailability {
				building: blds
					.remove(&bld_key)
					.expect("Buildings in map should always match buildings in data"),
				buildable: locks.is_empty(),
				current_count: owned_count,
				max_count,
				locks,
			}
		})
		.collect()
}

/// Parses a building requirement to determine if it creates a building lock based on
/// building levels or technology requirements.
///
/// # Arguments
/// * `bld_key` - The key of the building being checked for requirements
/// * `req` - The building requirement containing level or tech prerequisites
/// * `max_level` - The current maximum level of the required building
///
/// # Returns
/// * `Some(BuildingLock::BuildingLevelRequired)` if a required building level is not met
/// * `Some(BuildingLock::TechNodeRequired)` if a required technology is not researched
/// * `None` if all requirements are satisfied
fn parse_req_lock(
	bld_key: &BuildingKey,
	req: &BuildingRequirement,
	max_level: MaxBuildingLevel,
) -> Option<BuildingLock> {
	if let Some(req_bld_id) = req.required_building_id {
		let req_level = req
			.required_building_level
			.expect("Required building level must be set for building requirements");
		if max_level < req_level {
			Some(BuildingLock::BuildingLevelRequired {
				building: req_bld_id,
				current: max_level,
				required: req_level,
			})
		} else {
			None
		}
	} else {
		req.required_tech_id
			.map(|node_id| BuildingLock::TechNodeRequired { node_id })
	}
}
