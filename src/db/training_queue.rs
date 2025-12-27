//! Database access layer for training queue entities.
//!
//! This module provides operations for managing the training queue,
//! including creating entries, updating status, and querying by player or status.

use chrono::Utc;
use diesel::prelude::*;
use tracing::{debug, instrument, trace};

use crate::Result;
use crate::db::DbConn;
use crate::domain::player::PlayerKey;
use crate::domain::unit::training::{
	NewTrainingQueueEntry, TrainingQueueEntry, TrainingQueueKey, TrainingStatus,
};
use crate::schema::training_queue as tq;

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
