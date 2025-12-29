//! Integration tests for unit training operations.
//!
//! These tests cover the complete training lifecycle including:
//! - Starting training with resource deduction
//! - Viewing training queue
//! - Completing training and receiving units
//! - Cancelling training with refunds
//! - Validation error cases

use diesel::prelude::*;
use empire::auth::utils::hash_password;
use empire::db::{
	DbConn, player_buildings, player_units, players, resources, training_queue, units,
};
use empire::domain::factions::FactionCode;
use empire::domain::jobs::JobType;
use empire::domain::player::buildings::{NewPlayerBuilding, PlayerBuilding};
use empire::domain::player::{NewPlayer, Player, PlayerKey, UserName};
use empire::domain::unit::training::TrainingStatus;
use empire::domain::unit::{Unit, UnitType};
use empire::game::units::training_operations::{
	MAX_QUEUE_PER_BUILDING, TrainingJobPayload, cancel_training, complete_training,
	get_available_units_for_building, start_training,
};
use empire::schema::{job, unit};

use crate::common::TestHarness;

// ============================================================================
// Helper Functions
// ============================================================================

/// Shorten all unit training times to 1 second for fast tests.
fn shorten_training_times(conn: &mut DbConn) {
	diesel::update(unit::table)
		.set(unit::base_training_seconds.eq(1))
		.execute(conn)
		.expect("Failed to shorten training times");
}

/// Create a test player with the specified faction.
fn create_test_player(conn: &mut DbConn, faction: FactionCode) -> Player {
	players::create(
		conn,
		NewPlayer {
			name: UserName::parse(format!("test_player_{}", uuid::Uuid::new_v4())).unwrap(),
			pwd_hash: hash_password(b"test1234").unwrap(),
			email: None,
			faction,
		},
	)
	.expect("Failed to create test player")
}

/// Get a building by name for a specific faction.
fn get_building_by_name(
	conn: &mut DbConn,
	name: &str,
	faction: FactionCode,
) -> empire::domain::building::Building {
	use empire::schema::building::dsl;
	dsl::building
		.filter(dsl::name.eq(name))
		.filter(dsl::faction.eq(faction))
		.first(conn)
		.unwrap_or_else(|_| panic!("Building '{}' not found for faction {:?}", name, faction))
}

/// Construct a building for a player (for non-starter buildings like Barracks).
fn construct_building_for_player(
	conn: &mut DbConn,
	player_id: &PlayerKey,
	building_name: &str,
	faction: FactionCode,
) -> PlayerBuilding {
	let bld = get_building_by_name(conn, building_name, faction);
	player_buildings::construct(
		conn,
		NewPlayerBuilding {
			player_id: *player_id,
			building_id: bld.id,
			level: Some(0),
			upgrade_finishes_at: None,
		},
	)
	.expect("Failed to construct building")
}

/// Get an infantry unit from the database.
fn get_infantry_unit(conn: &mut DbConn) -> Unit {
	units::get_by_type(conn, &UnitType::Infantry)
		.expect("Failed to get infantry units")
		.into_iter()
		.next()
		.expect("No infantry unit found")
}

/// Get a cavalry unit from the database.
fn get_cavalry_unit(conn: &mut DbConn) -> Unit {
	units::get_by_type(conn, &UnitType::Cavalry)
		.expect("Failed to get cavalry units")
		.into_iter()
		.next()
		.expect("No cavalry unit found")
}

/// Give a player plenty of resources for training.
fn give_player_resources(conn: &mut DbConn, player_id: &PlayerKey) {
	use empire::schema::player_resource::dsl as pr;
	diesel::update(pr::player_resource.filter(pr::player_id.eq(player_id)))
		.set((
			pr::food.eq(10000),
			pr::wood.eq(10000),
			pr::stone.eq(10000),
			pr::gold.eq(10000),
		))
		.execute(conn)
		.expect("Failed to set player resources");
}

/// Get player's current resources.
fn get_player_resources(conn: &mut DbConn, player_id: &PlayerKey) -> (i64, i64, i64, i64) {
	let res = resources::get_by_player_id(conn, player_id).expect("Failed to get resources");
	(res.food, res.wood, res.stone, res.gold)
}

// ============================================================================
// Happy Path Tests
// ============================================================================

#[tokio::test]
async fn test_start_training_happy_path() {
	let TestHarness { app, db_pool, .. } = TestHarness::new();
	let mut conn = db_pool.get().unwrap();

	// Setup: create player with Human faction
	let player = create_test_player(&mut conn, FactionCode::Human);

	// Give player resources
	give_player_resources(&mut conn, &player.id);
	let (food_before, wood_before, _, _) = get_player_resources(&mut conn, &player.id);

	// Construct a Barracks for the player (not a starter building)
	let barracks =
		construct_building_for_player(&mut conn, &player.id, "Barracks", FactionCode::Human);

	// Shorten training times for faster tests
	shorten_training_times(&mut conn);

	// Get infantry unit
	let infantry = get_infantry_unit(&mut conn);

	// Start training 5 infantry units
	let quantity = 5;
	let entry = start_training(
		&mut conn,
		&app.job_queue,
		&player.id,
		&barracks.id,
		&infantry.id,
		quantity,
	)
	.expect("Failed to start training");

	// Assert: entry created with correct status
	assert_eq!(entry.status, TrainingStatus::InProgress);
	assert_eq!(entry.quantity, quantity);
	assert_eq!(entry.unit_id, infantry.id);
	assert_eq!(entry.building_id, barracks.id);
	assert!(entry.job_id.is_some(), "Job ID should be set");

	// Assert: resources were deducted
	// Infantry costs: Food 20, Wood 10 per unit
	let expected_food_cost = 20 * quantity;
	let expected_wood_cost = 10 * quantity;
	let (food_after, wood_after, _, _) = get_player_resources(&mut conn, &player.id);
	assert_eq!(food_after, food_before - expected_food_cost);
	assert_eq!(wood_after, wood_before - expected_wood_cost);

	// Assert: job was created in the database
	let job_id = entry.job_id.unwrap();
	let created_job: empire::domain::jobs::Job = job::table
		.find(job_id)
		.first(&mut conn)
		.expect("Job not found");
	assert_eq!(created_job.job_type, JobType::Training);
}

#[tokio::test]
async fn test_get_training_queue() {
	let TestHarness { app, db_pool, .. } = TestHarness::new();
	let mut conn = db_pool.get().unwrap();

	// Setup
	let player = create_test_player(&mut conn, FactionCode::Human);
	give_player_resources(&mut conn, &player.id);
	let barracks =
		construct_building_for_player(&mut conn, &player.id, "Barracks", FactionCode::Human);
	shorten_training_times(&mut conn);
	let infantry = get_infantry_unit(&mut conn);

	// Start two training entries
	let entry1 = start_training(
		&mut conn,
		&app.job_queue,
		&player.id,
		&barracks.id,
		&infantry.id,
		3,
	)
	.expect("Failed to start first training");
	let entry2 = start_training(
		&mut conn,
		&app.job_queue,
		&player.id,
		&barracks.id,
		&infantry.id,
		2,
	)
	.expect("Failed to start second training");

	// Get training queue
	let queue = training_queue::get_active_for_player(&mut conn, &player.id)
		.expect("Failed to get training queue");

	// Assert: both entries are in the queue
	assert_eq!(queue.len(), 2);
	let entry_ids: Vec<_> = queue.iter().map(|e| e.id).collect();
	assert!(entry_ids.contains(&entry1.id));
	assert!(entry_ids.contains(&entry2.id));

	// Assert: all entries have correct status
	for entry in &queue {
		assert_eq!(entry.status, TrainingStatus::InProgress);
	}
}

#[tokio::test]
async fn test_complete_training() {
	let TestHarness { app, db_pool, .. } = TestHarness::new();
	let mut conn = db_pool.get().unwrap();

	// Setup
	let player = create_test_player(&mut conn, FactionCode::Human);
	give_player_resources(&mut conn, &player.id);
	let barracks =
		construct_building_for_player(&mut conn, &player.id, "Barracks", FactionCode::Human);
	shorten_training_times(&mut conn);
	let infantry = get_infantry_unit(&mut conn);

	// Check initial unit count
	let initial_count = player_units::get_player_unit_count(&mut conn, &player.id, &infantry.id)
		.expect("Failed to get initial count");

	// Start training
	let quantity = 7;
	let entry = start_training(
		&mut conn,
		&app.job_queue,
		&player.id,
		&barracks.id,
		&infantry.id,
		quantity,
	)
	.expect("Failed to start training");

	// Complete training (simulates job processor calling this with payload)
	let payload = TrainingJobPayload {
		training_queue_entry_id: entry.id,
		player_id: player.id,
		unit_id: infantry.id,
		quantity,
	};
	let completed = complete_training(&mut conn, &payload).expect("Failed to complete training");

	// Assert: entry status is now Completed
	assert_eq!(completed.status, TrainingStatus::Completed);

	// Assert: player now has the trained units
	let final_count = player_units::get_player_unit_count(&mut conn, &player.id, &infantry.id)
		.expect("Failed to get final count");
	assert_eq!(final_count, initial_count + quantity);

	// Assert: function is idempotent
	let completed_again =
		complete_training(&mut conn, &payload).expect("Idempotent call should succeed");
	assert_eq!(completed_again.status, TrainingStatus::Completed);

	// Assert: unit count didn't increase again
	let final_count_again =
		player_units::get_player_unit_count(&mut conn, &player.id, &infantry.id)
			.expect("Failed to get final count again");
	assert_eq!(
		final_count_again, final_count,
		"Idempotent call should not add more units"
	);
}

#[tokio::test]
async fn test_cancel_training_full_refund() {
	let TestHarness { app, db_pool, .. } = TestHarness::new();
	let mut conn = db_pool.get().unwrap();

	// Setup
	let player = create_test_player(&mut conn, FactionCode::Human);
	give_player_resources(&mut conn, &player.id);
	let barracks =
		construct_building_for_player(&mut conn, &player.id, "Barracks", FactionCode::Human);
	shorten_training_times(&mut conn);
	let infantry = get_infantry_unit(&mut conn);

	// Start training
	let quantity = 5;
	let entry = start_training(
		&mut conn,
		&app.job_queue,
		&player.id,
		&barracks.id,
		&infantry.id,
		quantity,
	)
	.expect("Failed to start training");

	let (food_after_start, wood_after_start, _, _) = get_player_resources(&mut conn, &player.id);

	// Immediately cancel training
	let cancelled = cancel_training(&mut conn, &app.job_queue, &player.id, &entry.id)
		.expect("Failed to cancel training");

	// Assert: entry status is Cancelled
	assert_eq!(cancelled.0.status, TrainingStatus::Cancelled);

	// Assert: player received ~80% refund
	// Infantry costs: Food 20, Wood 10 per unit = 100 food, 50 wood for 5 units
	// Since cancelled immediately, remaining ratio ≈ 1.0, so refund ≈ 80%
	let cost_food = 20 * quantity;
	let cost_wood = 10 * quantity;
	let expected_refund_food = (cost_food as f64 * 0.80) as i64;
	let expected_refund_wood = (cost_wood as f64 * 0.80) as i64;

	let (food_after_cancel, wood_after_cancel, _, _) = get_player_resources(&mut conn, &player.id);

	// Allow small rounding difference
	assert!((food_after_cancel - (food_after_start + expected_refund_food)).abs() <= 1);
	assert!((wood_after_cancel - (wood_after_start + expected_refund_wood)).abs() <= 1);
}

#[tokio::test]
async fn test_get_available_units_for_building() {
	let TestHarness { db_pool, .. } = TestHarness::new();
	let mut conn = db_pool.get().unwrap();

	// Setup
	let player = create_test_player(&mut conn, FactionCode::Human);
	let barracks =
		construct_building_for_player(&mut conn, &player.id, "Barracks", FactionCode::Human);

	// Get available units for Barracks
	let available = get_available_units_for_building(&mut conn, &player.id, &barracks.id)
		.expect("Failed to get available units");

	// Assert: Barracks can train Infantry
	assert!(
		!available.is_empty(),
		"Barracks should have trainable units"
	);

	// All returned units should be Infantry type
	for unit in &available {
		assert_eq!(
			unit.unit_type,
			UnitType::Infantry,
			"Barracks should only train Infantry"
		);
	}

	// Construct a Stables and verify it trains Cavalry
	let stables =
		construct_building_for_player(&mut conn, &player.id, "Stables", FactionCode::Human);
	let cavalry_available = get_available_units_for_building(&mut conn, &player.id, &stables.id)
		.expect("Failed to get cavalry units");

	assert!(
		!cavalry_available.is_empty(),
		"Stables should have trainable units"
	);
	for unit in &cavalry_available {
		assert_eq!(
			unit.unit_type,
			UnitType::Cavalry,
			"Stables should only train Cavalry"
		);
	}
}

// ============================================================================
// Validation Error Tests
// ============================================================================

#[tokio::test]
async fn test_start_training_invalid_quantity() {
	let TestHarness { app, db_pool, .. } = TestHarness::new();
	let mut conn = db_pool.get().unwrap();

	// Setup
	let player = create_test_player(&mut conn, FactionCode::Human);
	give_player_resources(&mut conn, &player.id);
	let barracks =
		construct_building_for_player(&mut conn, &player.id, "Barracks", FactionCode::Human);
	let infantry = get_infantry_unit(&mut conn);

	// Try to start training with quantity = 0
	let result = start_training(
		&mut conn,
		&app.job_queue,
		&player.id,
		&barracks.id,
		&infantry.id,
		0,
	);

	assert!(result.is_err(), "Should fail with invalid quantity");
	let err = result.unwrap_err();
	assert!(
		err.to_string().contains("positive"),
		"Error should mention quantity: {}",
		err
	);
}

#[tokio::test]
async fn test_start_training_building_not_owned() {
	let TestHarness { app, db_pool, .. } = TestHarness::new();
	let mut conn = db_pool.get().unwrap();

	// Setup: create two players
	let player1 = create_test_player(&mut conn, FactionCode::Human);
	let player2 = create_test_player(&mut conn, FactionCode::Human);
	give_player_resources(&mut conn, &player1.id);

	// Player2 builds barracks
	let barracks =
		construct_building_for_player(&mut conn, &player2.id, "Barracks", FactionCode::Human);
	let infantry = get_infantry_unit(&mut conn);

	// Player1 tries to train at Player2's barracks
	let result = start_training(
		&mut conn,
		&app.job_queue,
		&player1.id,
		&barracks.id,
		&infantry.id,
		5,
	);

	assert!(result.is_err(), "Should fail when building not owned");
}

#[tokio::test]
async fn test_start_training_wrong_building_type() {
	let TestHarness { app, db_pool, .. } = TestHarness::new();
	let mut conn = db_pool.get().unwrap();

	// Setup
	let player = create_test_player(&mut conn, FactionCode::Human);
	give_player_resources(&mut conn, &player.id);
	let barracks =
		construct_building_for_player(&mut conn, &player.id, "Barracks", FactionCode::Human);
	let cavalry = get_cavalry_unit(&mut conn); // Cavalry should be trained at Stables, not Barracks

	// Try to train Cavalry at Barracks
	let result = start_training(
		&mut conn,
		&app.job_queue,
		&player.id,
		&barracks.id,
		&cavalry.id,
		5,
	);

	assert!(
		result.is_err(),
		"Should fail when training wrong unit type at building"
	);
}

#[tokio::test]
async fn test_start_training_insufficient_resources() {
	let TestHarness { app, db_pool, .. } = TestHarness::new();
	let mut conn = db_pool.get().unwrap();

	// Setup: create player but DON'T give resources
	let player = create_test_player(&mut conn, FactionCode::Human);
	let barracks =
		construct_building_for_player(&mut conn, &player.id, "Barracks", FactionCode::Human);
	let infantry = get_infantry_unit(&mut conn);

	// Drain all resources
	use empire::schema::player_resource::dsl as pr;
	diesel::update(pr::player_resource.filter(pr::player_id.eq(&player.id)))
		.set((
			pr::food.eq(0),
			pr::wood.eq(0),
			pr::stone.eq(0),
			pr::gold.eq(0),
		))
		.execute(&mut conn)
		.expect("Failed to drain resources");

	// Try to train with no resources
	let result = start_training(
		&mut conn,
		&app.job_queue,
		&player.id,
		&barracks.id,
		&infantry.id,
		5,
	);

	assert!(result.is_err(), "Should fail with insufficient resources");
}

#[tokio::test]
async fn test_start_training_queue_full() {
	let TestHarness { app, db_pool, .. } = TestHarness::new();
	let mut conn = db_pool.get().unwrap();

	// Setup
	let player = create_test_player(&mut conn, FactionCode::Human);
	give_player_resources(&mut conn, &player.id);
	let barracks =
		construct_building_for_player(&mut conn, &player.id, "Barracks", FactionCode::Human);
	shorten_training_times(&mut conn);
	let infantry = get_infantry_unit(&mut conn);

	// Fill the queue (max 5 per building)
	for i in 0..MAX_QUEUE_PER_BUILDING {
		start_training(
			&mut conn,
			&app.job_queue,
			&player.id,
			&barracks.id,
			&infantry.id,
			1,
		)
		.unwrap_or_else(|_| panic!("Failed to start training {}", i + 1));
	}

	// Try to add one more - should fail
	let result = start_training(
		&mut conn,
		&app.job_queue,
		&player.id,
		&barracks.id,
		&infantry.id,
		1,
	);

	assert!(result.is_err(), "Should fail when queue is full");
}

#[tokio::test]
async fn test_cancel_training_already_completed() {
	let TestHarness { app, db_pool, .. } = TestHarness::new();
	let mut conn = db_pool.get().unwrap();

	// Setup
	let player = create_test_player(&mut conn, FactionCode::Human);
	give_player_resources(&mut conn, &player.id);
	let barracks =
		construct_building_for_player(&mut conn, &player.id, "Barracks", FactionCode::Human);
	shorten_training_times(&mut conn);
	let infantry = get_infantry_unit(&mut conn);

	// Start and complete training
	let quantity = 3;
	let entry = start_training(
		&mut conn,
		&app.job_queue,
		&player.id,
		&barracks.id,
		&infantry.id,
		quantity,
	)
	.expect("Failed to start training");
	let payload = TrainingJobPayload {
		training_queue_entry_id: entry.id,
		player_id: player.id,
		unit_id: infantry.id,
		quantity,
	};
	complete_training(&mut conn, &payload).expect("Failed to complete training");

	// Try to cancel completed training
	let result = cancel_training(&mut conn, &app.job_queue, &player.id, &entry.id);

	assert!(
		result.is_err(),
		"Should fail when cancelling completed training"
	);
}
