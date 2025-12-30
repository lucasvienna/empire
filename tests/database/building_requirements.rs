use claims::assert_some;
use empire::db::building_levels::get_by_building_id;
use empire::db::building_requirements::{get_for_bld_and_level, get_for_level};

use crate::common::TestHarness;

#[tokio::test]
async fn test_get_for_level() {
	let db_pool = TestHarness::new().db_pool;
	let mut conn = db_pool.get().expect("Failed to get database connection");

	// bld_id 2 = Warehouse
	let levels = get_by_building_id(&mut conn, &2).expect("Failed to get building levels");
	assert!(!levels.is_empty());
	let level = &levels[2]; // should be level 2 of Warehouse
	assert_eq!(level.building_level, 2);

	let reqs = get_for_level(&mut conn, &level.id).unwrap();
	assert!(!reqs.is_empty());
	assert_eq!(reqs.len(), 1); // only the main keep for now
	let req = &reqs[0];
	assert_some!(req.required_building_id);
	assert_eq!(req.required_building_id.unwrap(), 1);
	assert_some!(req.required_building_level);
	assert_eq!(req.required_building_level.unwrap(), 2);
}

#[tokio::test]
async fn test_get_for_building_and_level() {
	let db_pool = TestHarness::new().db_pool;
	let mut conn = db_pool.get().expect("Failed to get database connection");

	// bld_id 2 = Warehouse
	let reqs = get_for_bld_and_level(&mut conn, &2, 2).expect("Failed to get building levels");
	assert!(!reqs.is_empty());
	assert_eq!(reqs.len(), 1); // only the main keep for now

	let req = &reqs[0];
	assert_some!(req.required_building_id);
	assert_eq!(req.required_building_id.unwrap(), 1);
	assert_some!(req.required_building_level);
	assert_eq!(req.required_building_level.unwrap(), 2);
}
