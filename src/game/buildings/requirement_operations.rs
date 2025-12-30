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

/// Contains resource costs and time required for building construction
#[derive(Debug, Clone, Default, Serialize, Eq, PartialEq)]
pub struct ConstructionInfo {
	/// Food required for construction
	pub food: i64,
	/// Wood required for construction
	pub wood: i64,
	/// Stone required for construction
	pub stone: i64,
	/// Gold required for construction
	pub gold: i64,
	/// Time in seconds to complete construction
	pub time_seconds: i64,
}

/// Represents the availability status of a building, including build restrictions and current state
#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
pub struct BuildingAvailability {
	/// The building definition
	pub building: Building,
	/// Whether the building can currently be constructed
	pub buildable: bool,
	/// Number of instances of this building currently owned
	pub current_count: i64,
	/// Maximum number of instances allowed
	pub max_count: i32,
	/// List of restrictions preventing construction
	pub locks: Vec<BuildingLock>,
	/// Resource costs and time required for construction
	pub construction: ConstructionInfo,
}

/// Represents different types of restrictions that can prevent a building from being constructed
#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
#[serde(tag = "kind")]
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

/// Generates availability status for a single building based on its current count and requirements
///
/// # Arguments
/// * `building` - The building definition
/// * `data` - Tuple containing current count, maximum count, and optional maximum level
/// * `requirements` - List of building requirements for this building
/// * `construction` - Construction costs and time for this building
///
/// # Returns
/// [BuildingAvailability] containing the availability status for the building
pub fn gen_avail_data(
	building: Building,
	data: AvailabilityData,
	requirements: Vec<BuildingRequirement>,
	construction: ConstructionInfo,
) -> BuildingAvailability {
	let (owned_count, max_count, max_level) = data;
	let mut locks: Vec<BuildingLock> = vec![];

	// Check if max count is reached
	if owned_count >= (max_count as i64) {
		locks.push(BuildingLock::MaxCountReached);
	}

	// Check requirement locks
	locks.extend(
		requirements
			.iter()
			.filter_map(|req| parse_req_lock(&building.id, req, max_level.unwrap_or_default())),
	);

	BuildingAvailability {
		building,
		buildable: locks.is_empty(),
		current_count: owned_count,
		max_count,
		locks,
		construction,
	}
}

/// Generates an availability map for buildings based on their current counts and requirements
///
/// # Arguments
/// * `blds` - Map of building definitions by their keys
/// * `bld_data` - Map containing current counts and maximum limits for each building
/// * `reqs_and_info` - Map of building requirements and construction info indexed by building keys
///
/// # Returns
/// Vector of [BuildingAvailability] containing availability status for each building
pub fn gen_avail_list(
	mut blds: HashMap<BuildingKey, Building>,
	bld_data: HashMap<BuildingKey, AvailabilityData>,
	mut reqs_and_info: HashMap<BuildingKey, (Vec<BuildingRequirement>, ConstructionInfo)>,
) -> Vec<BuildingAvailability> {
	bld_data
		.iter()
		.map(|(&bld_key, &(owned_count, max_count, _))| {
			let mut locks: Vec<BuildingLock> = vec![];

			// Get requirements and construction info for this building
			let (reqs, construction) = reqs_and_info.remove(&bld_key).unwrap_or_default();

			// Check if max count is reached
			if owned_count >= (max_count as i64) {
				locks.push(BuildingLock::MaxCountReached);
			}

			// Check requirement locks
			// AIDEV-NOTE: Look up the REQUIRED building's level, not the target building's level
			locks.extend(reqs.iter().filter_map(|req| {
				let required_bld_level = req
					.required_building_id
					.and_then(|req_bld_id| bld_data.get(&req_bld_id))
					.and_then(|(_, _, level)| *level)
					.unwrap_or(0);
				parse_req_lock(&bld_key, req, required_bld_level)
			}));

			BuildingAvailability {
				building: blds
					.remove(&bld_key)
					.expect("Buildings in map should always match buildings in data"),
				buildable: locks.is_empty(),
				current_count: owned_count,
				max_count,
				locks,
				construction,
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

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use chrono::Utc;
	use uuid::Uuid;

	use super::{BuildingLock, ConstructionInfo, gen_avail_data, gen_avail_list, parse_req_lock};
	use crate::domain::building::Building;
	use crate::domain::building::requirement::BuildingRequirement;
	use crate::domain::factions::FactionCode;

	fn make_test_building(id: i32, max_count: i32) -> Building {
		Building {
			id,
			name: format!("Test Building {}", id),
			max_level: 10,
			max_count,
			faction: FactionCode::Neutral,
			starter: false,
			created_at: Utc::now(),
			updated_at: Utc::now(),
		}
	}

	fn make_building_requirement(
		required_building_id: i32,
		required_building_level: i32,
	) -> BuildingRequirement {
		BuildingRequirement {
			id: Uuid::now_v7(),
			building_level_id: Uuid::now_v7(),
			required_building_id: Some(required_building_id),
			required_building_level: Some(required_building_level),
			required_tech_id: None,
			required_tech_level: None,
			created_at: Utc::now(),
			updated_at: Utc::now(),
		}
	}

	#[test]
	fn test_gen_avail_data_buildable_when_no_locks() {
		let building = make_test_building(1, 5);
		let data = (0, 5, None); // count=0, max_count=5, no max_level yet
		let requirements: Vec<BuildingRequirement> = vec![];

		let availability = gen_avail_data(
			building.clone(),
			data,
			requirements,
			ConstructionInfo::default(),
		);

		assert!(availability.buildable);
		assert!(availability.locks.is_empty());
		assert_eq!(availability.current_count, 0);
		assert_eq!(availability.max_count, 5);
		assert_eq!(availability.building.id, building.id);
	}

	#[test]
	fn test_gen_avail_list_mixed_availability() {
		// Building 1: buildable (count 0, max 5, no requirements)
		let bld1 = make_test_building(1, 5);
		// Building 2: locked due to max count reached (count 3, max 3)
		let bld2 = make_test_building(2, 3);

		let mut buildings = HashMap::new();
		buildings.insert(1, bld1);
		buildings.insert(2, bld2);

		let mut bld_data = HashMap::new();
		bld_data.insert(1, (0_i64, 5_i32, None)); // buildable
		bld_data.insert(2, (3_i64, 3_i32, Some(3))); // at max count

		let reqs_and_info: HashMap<i32, (Vec<BuildingRequirement>, ConstructionInfo)> =
			HashMap::new();

		let availability_list = gen_avail_list(buildings, bld_data, reqs_and_info);

		assert_eq!(availability_list.len(), 2);

		let bld1_avail = availability_list
			.iter()
			.find(|a| a.building.id == 1)
			.unwrap();
		assert!(bld1_avail.buildable);
		assert!(bld1_avail.locks.is_empty());

		let bld2_avail = availability_list
			.iter()
			.find(|a| a.building.id == 2)
			.unwrap();
		assert!(!bld2_avail.buildable);
		assert_eq!(bld2_avail.locks.len(), 1);
		assert_eq!(bld2_avail.locks[0], BuildingLock::MaxCountReached);
	}

	#[test]
	fn test_parse_req_lock_returns_none_when_level_met() {
		// Requirement: Building 1 must be at level 2
		let req = make_building_requirement(1, 2);
		// Player has building at level 3, which meets the requirement
		let result = parse_req_lock(&2, &req, 3);
		assert!(
			result.is_none(),
			"Should return None when requirement is met"
		);
	}

	#[test]
	fn test_parse_req_lock_returns_lock_when_level_not_met() {
		// Requirement: Building 1 must be at level 3
		let req = make_building_requirement(1, 3);
		// Player has building at level 2, which does NOT meet the requirement
		let result = parse_req_lock(&2, &req, 2);
		assert!(
			result.is_some(),
			"Should return a lock when requirement is not met"
		);
		assert_eq!(
			result.unwrap(),
			BuildingLock::BuildingLevelRequired {
				building: 1,
				current: 2,
				required: 3,
			}
		);
	}

	/// Tests that building requirements correctly check the REQUIRED building's level.
	///
	/// Scenario: Player wants to build a Warehouse (id=2) which requires Keep (id=1) at level 1.
	/// The player has a Keep at level 1 but no Warehouse yet.
	///
	/// BUG: The current implementation passes the Warehouse's max_level (None/0) to parse_req_lock
	/// instead of the Keep's max_level (1). This causes a false BuildingLevelRequired lock.
	///
	/// Expected: Warehouse should be buildable (no BuildingLevelRequired lock)
	/// Actual (with bug): Warehouse has BuildingLevelRequired lock with current=0, required=1
	#[test]
	fn test_gen_avail_list_checks_required_building_level_not_target_building_level() {
		// Building 1: Keep (the required building) - player has it at level 1
		let keep = make_test_building(1, 1);
		// Building 2: Warehouse (requires Keep level 1) - player has none yet
		let warehouse = make_test_building(2, 5);

		let mut buildings = HashMap::new();
		buildings.insert(1, keep);
		buildings.insert(2, warehouse);

		let mut bld_data = HashMap::new();
		// Keep: count=1, max_count=1, max_level=1 (player has one at level 1)
		bld_data.insert(1, (1_i64, 1_i32, Some(1)));
		// Warehouse: count=0, max_count=5, max_level=None (player has none)
		bld_data.insert(2, (0_i64, 5_i32, None));

		// Warehouse requires Keep at level 1
		let warehouse_req = make_building_requirement(1, 1);
		let mut reqs_and_info: HashMap<i32, (Vec<BuildingRequirement>, ConstructionInfo)> =
			HashMap::new();
		reqs_and_info.insert(2, (vec![warehouse_req], ConstructionInfo::default()));

		let availability_list = gen_avail_list(buildings, bld_data, reqs_and_info);

		// Find Warehouse availability
		let warehouse_avail = availability_list
			.iter()
			.find(|a| a.building.id == 2)
			.expect("Warehouse should be in the list");

		// The Warehouse requires Keep level 1, and the player HAS Keep at level 1.
		// Therefore, the Warehouse should NOT have a BuildingLevelRequired lock.
		let has_building_level_lock = warehouse_avail
			.locks
			.iter()
			.any(|lock| matches!(lock, BuildingLock::BuildingLevelRequired { .. }));

		assert!(
			!has_building_level_lock,
			"Warehouse should NOT have BuildingLevelRequired lock since player has Keep at level 1. \
			Found locks: {:?}",
			warehouse_avail.locks
		);
		assert!(
			warehouse_avail.buildable,
			"Warehouse should be buildable when Keep level requirement is met"
		);
	}

	/// Tests that a building correctly gets a lock when the required building level is NOT met.
	///
	/// Scenario: Player wants to build a Barracks (id=3) which requires Keep (id=1) at level 3.
	/// The player has a Keep at level 1.
	///
	/// Expected: Barracks should have BuildingLevelRequired lock with current=1, required=3
	#[test]
	fn test_gen_avail_list_locks_when_required_level_not_met() {
		// Building 1: Keep - player has it at level 1
		let keep = make_test_building(1, 1);
		// Building 3: Barracks (requires Keep level 3)
		let barracks = make_test_building(3, 3);

		let mut buildings = HashMap::new();
		buildings.insert(1, keep);
		buildings.insert(3, barracks);

		let mut bld_data = HashMap::new();
		// Keep: player has one at level 1
		bld_data.insert(1, (1_i64, 1_i32, Some(1)));
		// Barracks: player has none
		bld_data.insert(3, (0_i64, 3_i32, None));

		// Barracks requires Keep at level 3
		let barracks_req = make_building_requirement(1, 3);
		let mut reqs_and_info: HashMap<i32, (Vec<BuildingRequirement>, ConstructionInfo)> =
			HashMap::new();
		reqs_and_info.insert(3, (vec![barracks_req], ConstructionInfo::default()));

		let availability_list = gen_avail_list(buildings, bld_data, reqs_and_info);

		let barracks_avail = availability_list
			.iter()
			.find(|a| a.building.id == 3)
			.expect("Barracks should be in the list");

		// Barracks requires Keep level 3, but player only has Keep level 1.
		// So it SHOULD have a BuildingLevelRequired lock.
		let building_level_lock = barracks_avail.locks.iter().find(|lock| {
			matches!(
				lock,
				BuildingLock::BuildingLevelRequired {
					building: 1,
					required: 3,
					..
				}
			)
		});

		assert!(
			building_level_lock.is_some(),
			"Barracks should have BuildingLevelRequired lock for Keep level 3"
		);

		// The lock should show current=1 (the Keep's actual level), not current=0 (Barracks' level)
		if let Some(BuildingLock::BuildingLevelRequired { current, .. }) = building_level_lock {
			assert_eq!(
				*current, 1,
				"Lock should show Keep's current level (1), not Barracks' level (0)"
			);
		}

		assert!(
			!barracks_avail.buildable,
			"Barracks should not be buildable when Keep level requirement is not met"
		);
	}
}
