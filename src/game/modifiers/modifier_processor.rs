//! Modifier processing system for handling game modifier jobs.
//!
//! This module implements the job processing functionality for game modifiers,
//! including expiration, resource recalculation, and cache updates. It provides
//! a robust worker system that can handle multiple jobs concurrently while
//! gracefully handling shutdowns and errors.

use std::sync::Arc;

use axum::extract::FromRef;
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, instrument, trace, warn};
use ulid::Ulid;

use crate::domain::app_state::{AppPool, AppState};
use crate::domain::jobs::{Job, JobType};
use crate::game::modifiers::modifier_scheduler::ModifierJobPayload;
use crate::game::modifiers::modifier_service::ModifierService;
use crate::job_queue::job_processor::JobProcessor;
use crate::job_queue::JobQueue;
use crate::Error;

/// A processor for handling modifier-related background jobs.
///
/// The `ModifierProcessor` implements the `JobProcessor` trait and is responsible
/// for processing various modifier-related tasks such as:
/// - Expiring modifiers
/// - Recalculating player resources
/// - Updating modifier caches
///
/// Each processor instance runs in its own task and polls the job queue for new work.
pub struct ModifierProcessor {
	/// A unique ID for the processor instance
	id: String,
	/// A reference to the application's database connection pool
	db_pool: AppPool,
	/// A broadcast channel receiver for handling graceful shutdowns
	shutdown_rx: broadcast::Receiver<()>,
	/// Modifier service instance
	srv: ModifierService,
}

impl ModifierProcessor {
	/// Creates multiple ModifierProcessor instances for parallel processing.
	///
	/// # Arguments
	///
	/// * `n` - The number of processor instances to create
	/// * `state` - A reference to the application state containing shared resources
	///
	/// # Returns
	///
	/// A vector containing `n` ModifierProcessor instances, each initialised with
	/// the same application state but unique identifiers
	pub fn initialise_n(n: usize, state: &AppState) -> Vec<ModifierProcessor> {
		(0..n)
			.map(|_| ModifierProcessor::from_ref(state))
			.collect::<Vec<_>>()
	}
}

impl FromRef<AppState> for ModifierProcessor {
	fn from_ref(state: &AppState) -> Self {
		let rx = state.job_queue.subscribe_shutdown();
		Self::new(state, rx)
	}
}

impl JobProcessor for ModifierProcessor {
	/// Creates a new `ModifierProcessor` instance.
	///
	/// # Arguments
	///
	/// * `db_pool` - A reference to the application's database connection pool
	/// * `shutdown_rx` - A broadcast channel receiver for handling graceful shutdowns
	///
	/// # Returns
	///
	/// A new `ModifierProcessor` instance with a unique ID
	fn new(app_state: &AppState, shutdown_rx: broadcast::Receiver<()>) -> Self {
		let id = format!("modifier-goblin-{}", Ulid::new());
		let srv = ModifierService::from_ref(app_state);
		debug!("Starting worker {}", id);
		Self {
			id,
			db_pool: Arc::clone(&app_state.db_pool),
			shutdown_rx,
			srv,
		}
	}

	/// Runs the processor's main loop, continuously processing jobs from the queue.
	///
	/// This method will run until a shutdown signal is received or an unrecoverable
	/// error occurs. It handles job processing, completion, and failure states while
	/// implementing backoff strategies for error conditions.
	///
	/// # Arguments
	///
	/// * `queue` - An Arc reference to the job queue to process
	///
	/// # Returns
	///
	/// A Result indicating success or containing an error if the processor fails
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
					match queue.get_next_job_of_type(&self.id, &JobType::Modifier) {
						Ok(Some(job)) => {
							// Found a job, process it
							debug!("Worker {} picked up job {}", self.id, job.id);
							match self.process_job(job.clone()).await {
								Ok(()) => {
									debug!("Worker {} completed job {}", self.id, job.id);
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

	/// Processes a single job from the queue.
	///
	/// # Arguments
	///
	/// * `job` - The job to process
	///
	/// # Returns
	///
	/// A Result indicating success or containing an error if job processing fails
	#[instrument(skip(self, job), fields(job_id = %job.id))]
	async fn process_job(&self, job: Job) -> Result<(), Error> {
		info!("Processing job {:?}", job);
		assert_eq!(
			job.job_type,
			JobType::Modifier,
			"Expected a modifier job, got: {}",
			job.job_type
		);
		let payload: ModifierJobPayload = serde_json::from_value(job.payload.clone())?;

		// Now you can match on the payload enum variants and handle each case
		match payload {
			ModifierJobPayload::ExpireModifier {
				modifier_id,
				player_id,
			} => {
				// Handle modifier expiration
			}
			ModifierJobPayload::RecalculateResources {
				player_id,
				resource_types,
			} => {
				// Handle resource recalculation
			}
			ModifierJobPayload::UpdateModifierCache { player_id } => {
				// Handle cache update
			}
		}

		Ok(())
	}
}
