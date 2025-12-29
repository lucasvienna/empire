//! Training job processor for completing unit training.
//!
//! This module implements the job processing functionality for training completion,
//! handling the addition of trained units to player inventories when training
//! time has elapsed.

use std::sync::Arc;
use std::time::Duration;

use axum::extract::FromRef;
use tokio::sync::broadcast::Receiver;
use tokio::time::sleep;
use tracing::{debug, error, info, instrument, trace, warn};
use ulid::Ulid;

use crate::Error;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::jobs::{Job, JobType};
use crate::game::units::training_operations::{self, TrainingJobPayload};
use crate::job_queue::JobQueue;
use crate::job_queue::job_processor::JobProcessor;

/// A processor for handling training-related background jobs.
///
/// The `TrainingProcessor` implements the `JobProcessor` trait and is responsible
/// for completing unit training when the training duration has elapsed.
///
/// Each processor instance runs in its own task and polls the job queue for new work.
pub struct TrainingProcessor {
	/// A unique ID for the processor instance
	id: String,
	/// A broadcast channel receiver for handling graceful shutdowns
	shutdown_rx: Receiver<()>,
	/// Database connection pool
	pool: AppPool,
}

impl TrainingProcessor {
	/// Creates multiple TrainingProcessor instances for parallel processing.
	pub fn initialise_n(n: usize, state: &AppState) -> Vec<TrainingProcessor> {
		(0..n)
			.map(|_| TrainingProcessor::from_ref(state))
			.collect::<Vec<_>>()
	}
}

impl FromRef<AppState> for TrainingProcessor {
	fn from_ref(state: &AppState) -> Self {
		let rx = state.job_queue.subscribe_shutdown();
		Self::new(state, rx)
	}
}

impl JobProcessor for TrainingProcessor {
	/// Creates a new `TrainingProcessor` instance.
	///
	/// # Arguments
	///
	/// * `app_state` - A reference to the application state
	/// * `shutdown_rx` - A broadcast channel receiver for handling graceful shutdowns
	///
	/// # Returns
	///
	/// A new `TrainingProcessor` instance with a unique ID
	fn new(app_state: &AppState, shutdown_rx: Receiver<()>) -> Self
	where
		Self: Sized,
	{
		let id = format!("training-goblin-{}", Ulid::new());
		let pool = AppPool::from_ref(app_state);
		debug!("Starting worker {}", id);
		Self {
			id,
			shutdown_rx,
			pool,
		}
	}

	#[instrument(skip(self, queue))]
	async fn run(&mut self, queue: Arc<JobQueue>) -> Result<(), Error> {
		let mut interval = tokio::time::interval(Duration::from_secs(1));
		trace!("Worker {} running", self.id);

		loop {
			tokio::select! {
				_ = self.shutdown_rx.recv() => {
					debug!("Worker {} shutting down", self.id);
					break;
				}
				_ = interval.tick() => {
					match queue.get_next_job_of_type(&self.id, &JobType::Training) {
						Ok(Some(job)) => {
							// Found a job, process it
							trace!("Worker {} picked up job {}", self.id, job.id);
							match self.process_job(job.clone()).await {
								Ok(()) => {
									trace!("Worker {} completed job {}", self.id, job.id);
									queue.complete_job(&job.id)?;
								}
								Err(e) => {
									warn!("Worker {} failed to process job {}", self.id, job.id);
									debug!("Failed job: {:#?} {:?}", job, e);
									queue.fail_job(&job.id, e.to_string())?;
								}
							}
						}
						Ok(None) => {
							// No jobs available, continue polling
							sleep(Duration::from_secs(1)).await;
						}
						Err(e) => {
							// Error fetching job, retry after a short delay
							error!("Error fetching job: {}", e);
							sleep(Duration::from_secs(5)).await;
						}
					}
				}
			}
		}

		Ok(())
	}

	#[instrument(skip(self, job), fields(job_id = %job.id))]
	async fn process_job(&self, job: Job) -> Result<(), Error> {
		debug!("Processing training job: {}", job.id);
		trace!("Job details: {:?}", job);

		assert_eq!(
			job.job_type,
			JobType::Training,
			"Expected a training job, got: {}",
			job.job_type
		);

		let payload: TrainingJobPayload = serde_json::from_value(job.payload.clone())?;

		let mut conn = self.pool.get()?;

		// AIDEV-NOTE: complete_training handles idempotency - calling multiple times is safe
		match training_operations::complete_training(&mut conn, &payload) {
			Ok(entry) => {
				info!(
					"Successfully completed training {} for player {}: {} x {} units",
					entry.id, payload.player_id, payload.quantity, payload.unit_id
				);
			}
			Err(e) => {
				error!(
					"Failed to complete training {} for player {}: {}",
					payload.training_queue_entry_id, payload.player_id, e
				);
				return Err(e);
			}
		}

		debug!("Completed processing training job: {}", job.id);
		Ok(())
	}
}
