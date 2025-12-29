//! Database access layer for training queue entities.
//!
//! This module provides operations for managing the training queue,
//! including creating entries, updating status, and querying by player or status.

use chrono::Utc;
use diesel::prelude::*;
use tracing::{debug, instrument, trace};

use crate::Result;
use crate::db::DbConn;
use crate::domain::jobs::JobKey;
use crate::domain::player::PlayerKey;
use crate::domain::player::buildings::PlayerBuildingKey;
use crate::domain::unit::training::{
	NewTrainingQueueEntry, TrainingQueueEntry, TrainingQueueKey, TrainingStatus,
};
use crate::schema::{building_level as bl, player_building as pb, training_queue as tq};

/// Current queue state for a building, including active count and capacity.
#[derive(Debug, Clone)]
pub struct QueueState {
	/// Number of active (pending or in-progress) training entries
	pub active_count: i64,
	/// Maximum training capacity for this building at its current level
	pub capacity: i64,
}

/// Creates a new training queue entry.
#[instrument(skip(conn, entity))]
pub fn create(conn: &mut DbConn, entity: NewTrainingQueueEntry) -> Result<TrainingQueueEntry> {
	debug!(
		"Creating training queue entry for player {} unit {}",
		entity.player_id, entity.unit_id
	);
	let entry = diesel::insert_into(tq::table)
		.values(entity)
		.returning(TrainingQueueEntry::as_returning())
		.get_result(conn)?;
	trace!("Created training queue entry: {:?}", entry);
	Ok(entry)
}

/// Retrieves all active (non-completed, non-cancelled) training queue entries for a player.
#[instrument(skip(conn))]
pub fn get_active_for_player(
	conn: &mut DbConn,
	player_key: &PlayerKey,
) -> Result<Vec<TrainingQueueEntry>> {
	let entries = tq::table
		.filter(tq::player_id.eq(player_key))
		.filter(
			tq::status
				.eq(TrainingStatus::Pending)
				.or(tq::status.eq(TrainingStatus::InProgress)),
		)
		.select(TrainingQueueEntry::as_select())
		.load(conn)?;
	Ok(entries)
}

/// Retrieves all training queue entries with a specific status.
#[instrument(skip(conn))]
pub fn get_by_status(
	conn: &mut DbConn,
	status_filter: &TrainingStatus,
) -> Result<Vec<TrainingQueueEntry>> {
	let entries = tq::table
		.filter(tq::status.eq(status_filter))
		.select(TrainingQueueEntry::as_select())
		.load(conn)?;
	Ok(entries)
}

/// Marks a training queue entry as completed.
///
/// Sets the status to Completed and records the completion timestamp.
#[instrument(skip(conn))]
pub fn complete(conn: &mut DbConn, entry_id: &TrainingQueueKey) -> Result<TrainingQueueEntry> {
	debug!("Completing training queue entry {}", entry_id);
	let entry = diesel::update(tq::table.find(entry_id))
		.set((
			tq::status.eq(TrainingStatus::Completed),
			tq::completed_at.eq(Some(Utc::now())),
		))
		.returning(TrainingQueueEntry::as_returning())
		.get_result(conn)?;
	trace!("Completed training queue entry: {:?}", entry);
	Ok(entry)
}

/// Cancels a training queue entry.
///
/// Sets the status to Cancelled. Does not record a completion timestamp.
#[instrument(skip(conn))]
pub fn cancel(conn: &mut DbConn, entry_id: &TrainingQueueKey) -> Result<usize> {
	debug!("Cancelling training queue entry {}", entry_id);
	let count = diesel::update(tq::table.find(entry_id))
		.set(tq::status.eq(TrainingStatus::Cancelled))
		.execute(conn)?;
	Ok(count)
}

/// Updates the status of a training queue entry.
#[instrument(skip(conn))]
pub fn update_status(
	conn: &mut DbConn,
	entry_id: &TrainingQueueKey,
	new_status: &TrainingStatus,
) -> Result<TrainingQueueEntry> {
	debug!(
		"Updating training queue entry {} status to {:?}",
		entry_id, new_status
	);
	let entry = diesel::update(tq::table.find(entry_id))
		.set(tq::status.eq(new_status))
		.returning(TrainingQueueEntry::as_returning())
		.get_result(conn)?;
	trace!("Updated training queue entry: {:?}", entry);
	Ok(entry)
}

/// Retrieves a training queue entry by its ID.
#[instrument(skip(conn))]
pub fn get_by_id(conn: &mut DbConn, entry_id: &TrainingQueueKey) -> Result<TrainingQueueEntry> {
	let entry = tq::table.find(entry_id).first(conn)?;
	Ok(entry)
}

/// Gets the count of active training entries for a specific building.
///
/// Used to enforce per-building queue capacity limits.
#[instrument(skip(conn))]
pub fn get_active_count_for_building(
	conn: &mut DbConn,
	building_key: &PlayerBuildingKey,
) -> Result<i64> {
	let count = tq::table
		.filter(tq::building_id.eq(building_key))
		.filter(
			tq::status
				.eq(TrainingStatus::Pending)
				.or(tq::status.eq(TrainingStatus::InProgress)),
		)
		.count()
		.get_result(conn)?;
	Ok(count)
}

/// Gets the count of active training entries for a building with row-level locking.
///
/// Uses `FOR UPDATE` to prevent race conditions when checking queue capacity.
/// Must be called within a transaction.
#[instrument(skip(conn))]
pub fn get_active_count_for_building_locked(
	conn: &mut DbConn,
	building_key: &PlayerBuildingKey,
) -> Result<i64> {
	// Lock all active entries for this building to prevent concurrent inserts
	let _locked_entries: Vec<TrainingQueueEntry> = tq::table
		.filter(tq::building_id.eq(building_key))
		.filter(
			tq::status
				.eq(TrainingStatus::Pending)
				.or(tq::status.eq(TrainingStatus::InProgress)),
		)
		.for_update()
		.load(conn)?;

	Ok(_locked_entries.len() as i64)
}

/// Gets the queue state (active count and capacity) for a building with row-level locking.
///
/// Performs a single query that:
/// 1. Locks active training queue entries (FOR UPDATE) to prevent race conditions
/// 2. Joins with player_building and building_level to get the capacity
///
/// Must be called within a transaction.
///
/// # Returns
/// A [`QueueState`] containing the active count and capacity for capacity validation.
/// Returns capacity of 0 if the building has no training_capacity set (non-military building).
#[instrument(skip(conn))]
pub fn get_queue_state(conn: &mut DbConn, building_key: &PlayerBuildingKey) -> Result<QueueState> {
	// Lock all active entries for this building to prevent concurrent inserts
	let locked_entries: Vec<TrainingQueueEntry> = tq::table
		.filter(tq::building_id.eq(building_key))
		.filter(
			tq::status
				.eq(TrainingStatus::Pending)
				.or(tq::status.eq(TrainingStatus::InProgress)),
		)
		.for_update()
		.load(conn)?;

	let active_count = locked_entries.len() as i64;

	// Get the building's training capacity by joining player_building -> building_level
	// AIDEV-NOTE: Uses (building_id, level) to find the correct building_level row
	let capacity: Option<i32> = pb::table
		.inner_join(
			bl::table.on(pb::building_id
				.eq(bl::building_id)
				.and(pb::level.eq(bl::level))),
		)
		.filter(pb::id.eq(building_key))
		.select(bl::training_capacity)
		.first::<Option<i32>>(conn)?;

	debug!(
		"Queue state for building {}: active={}, capacity={:?}",
		building_key, active_count, capacity
	);

	Ok(QueueState {
		active_count,
		capacity: capacity.unwrap_or(0) as i64,
	})
}

/// Gets the queue status (active count and capacity) for a building without locking.
///
/// Single-query read-only operation for API responses. Use `get_queue_state` for
/// transactional capacity checks that require locking.
///
/// # Returns
/// A [`QueueState`] containing the active count and capacity.
/// Returns capacity of 0 if the building has no training_capacity set (non-military building).
#[instrument(skip(conn))]
pub fn get_queue_status(conn: &mut DbConn, building_key: &PlayerBuildingKey) -> Result<QueueState> {
	// Single query: count active entries and get capacity via joins
	// AIDEV-NOTE: Uses a subquery for count to avoid loading all entries
	let (active_count, capacity): (i64, Option<i32>) = pb::table
		.inner_join(
			bl::table.on(pb::building_id
				.eq(bl::building_id)
				.and(pb::level.eq(bl::level))),
		)
		.filter(pb::id.eq(building_key))
		.select((
			tq::table
				.filter(tq::building_id.eq(building_key))
				.filter(
					tq::status
						.eq(TrainingStatus::Pending)
						.or(tq::status.eq(TrainingStatus::InProgress)),
				)
				.count()
				.single_value()
				.assume_not_null(),
			bl::training_capacity,
		))
		.first(conn)?;

	trace!(
		"Queue status for building {}: active={}, capacity={:?}",
		building_key, active_count, capacity
	);

	Ok(QueueState {
		active_count,
		capacity: capacity.unwrap_or(0) as i64,
	})
}

/// Gets all active training queue entries for a specific building.
#[instrument(skip(conn))]
pub fn get_active_for_building(
	conn: &mut DbConn,
	building_key: &PlayerBuildingKey,
) -> Result<Vec<TrainingQueueEntry>> {
	let entries = tq::table
		.filter(tq::building_id.eq(building_key))
		.filter(
			tq::status
				.eq(TrainingStatus::Pending)
				.or(tq::status.eq(TrainingStatus::InProgress)),
		)
		.select(TrainingQueueEntry::as_select())
		.load(conn)?;
	Ok(entries)
}

/// Retrieves a training queue entry by its associated job ID.
#[instrument(skip(conn))]
pub fn get_by_job_id(conn: &mut DbConn, job_key: &JobKey) -> Result<TrainingQueueEntry> {
	let entry = tq::table.filter(tq::job_id.eq(job_key)).first(conn)?;
	Ok(entry)
}

/// Sets the job ID for a training queue entry.
///
/// Called after scheduling the completion job.
#[instrument(skip(conn))]
pub fn set_job_id(
	conn: &mut DbConn,
	entry_id: &TrainingQueueKey,
	job_key: &JobKey,
) -> Result<TrainingQueueEntry> {
	debug!(
		"Setting job_id {} for training queue entry {}",
		job_key, entry_id
	);
	let entry = diesel::update(tq::table.find(entry_id))
		.set(tq::job_id.eq(Some(job_key)))
		.returning(TrainingQueueEntry::as_returning())
		.get_result(conn)?;
	Ok(entry)
}

/// Deletes a training queue entry.
///
/// Used for cleanup when job scheduling fails after entry creation.
#[instrument(skip(conn))]
pub fn delete(conn: &mut DbConn, entry_id: &TrainingQueueKey) -> Result<usize> {
	debug!("Deleting training queue entry {}", entry_id);
	let count = diesel::delete(tq::table.find(entry_id)).execute(conn)?;
	Ok(count)
}
