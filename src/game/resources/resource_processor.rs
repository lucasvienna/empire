use std::sync::Arc;
use std::time::Duration;

use axum::extract::FromRef;
use tokio::sync::broadcast::Receiver;
use tokio::time::sleep;
use tracing::{debug, error, info, instrument, trace, warn};
use ulid::Ulid;

use crate::domain::app_state::AppState;
use crate::domain::jobs::{Job, JobType};
use crate::game::modifiers::modifier_service::ModifierService;
use crate::game::resources::resource_operations;
use crate::game::resources::resource_scheduler::ProductionJobPayload;
use crate::game::resources::resource_service::ResourceService;
use crate::job_queue::job_processor::JobProcessor;
use crate::job_queue::JobQueue;
use crate::Error;

/// A processor for handling resources-related background jobs.
///
/// The `ResourceProcessor` implements the `JobProcessor` trait and is responsible
/// for processing various resources-related tasks such as:
/// - Producing resources
/// - Adding resources to the accumulator
///
/// Each processor instance runs in its own task and polls the job queue for new work.
pub struct ResourceProcessor {
	/// A unique ID for the processor instance
	id: String,
	/// A broadcast channel receiver for handling graceful shutdowns
	shutdown_rx: Receiver<()>,
	/// Resource service instance
	resource_srv: ResourceService,
	/// Modifier service instance
	modifier_srv: ModifierService,
}

impl ResourceProcessor {
	pub fn initialise_n(n: usize, state: &AppState) -> Vec<ResourceProcessor> {
		(0..n)
			.map(|_| ResourceProcessor::from_ref(state))
			.collect::<Vec<_>>()
	}
}

impl FromRef<AppState> for ResourceProcessor {
	fn from_ref(state: &AppState) -> Self {
		let rx = state.job_queue.subscribe_shutdown();
		Self::new(state, rx)
	}
}

impl JobProcessor for ResourceProcessor {
	/// Creates a new `ProductionProcessor` instance.
	///
	/// # Arguments
	///
	/// * `app_state` - A reference to the application state
	/// * `shutdown_rx` - A broadcast channel receiver for handling graceful shutdowns
	///
	/// # Returns
	///
	/// A new `ProductionProcessor` instance with a unique ID
	fn new(app_state: &AppState, shutdown_rx: Receiver<()>) -> Self
	where
		Self: Sized,
	{
		let id = format!("resource-goblin-{}", Ulid::new());
		let resource_srv = ResourceService::from_ref(app_state);
		let modifier_srv = ModifierService::from_ref(app_state);
		debug!("Starting worker {}", id);
		Self {
			id,
			shutdown_rx,
			resource_srv,
			modifier_srv,
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
					match queue.get_next_job_of_type(&self.id, &JobType::Resource) {
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
									debug!("Failed job: {:#?} {:?}",job, e);
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

	#[instrument(skip(self, job))]
	async fn process_job(&self, job: Job) -> Result<(), Error> {
		debug!("Processing job: {}", job.id);
		trace!("Job details: {:?}", job);

		assert_eq!(
			job.job_type,
			JobType::Resource,
			"Expected a resource job, got: {}",
			job.job_type
		);

		let payload: ProductionJobPayload = serde_json::from_value(job.payload.clone())?;

		match payload {
			ProductionJobPayload::ProduceResources { players_id } => {
				debug!(
					"Processing produce resources job for player: {}",
					players_id
				);
				// Compose both services: get modifiers, get base rates, combine, then produce
				match self.produce_resources_for_player(&players_id).await {
					Ok(_) => {
						info!("Successfully produced resources for player: {}", players_id);
					}
					Err(e) => {
						error!(
							"Failed to produce resources for player {}: {}",
							players_id, e
						);
						trace!("Detailed error: {:?}", e);
						return Err(e);
					}
				}
			}
			ProductionJobPayload::CollectResources { players_id } => {
				debug!(
					"Processing collect resources job for player: {}",
					players_id
				);
				match self.resource_srv.collect(&players_id) {
					Ok(_) => {
						info!(
							"Successfully collected resources for player: {}",
							players_id
						);
					}
					Err(e) => {
						error!(
							"Failed to collect resources for player {}: {}",
							players_id, e
						);
						trace!("Detailed error: {:?}", e);
						return Err(e);
					}
				}
			}
		}

		debug!("Completed process job: {}", job.id);
		Ok(())
	}
}

impl ResourceProcessor {
	/// Orchestrates resource production by composing modifier and resource services
	async fn produce_resources_for_player(
		&self,
		player_id: &crate::domain::player::PlayerKey,
	) -> Result<(), Error> {
		// Step 1: Fetch all resource modifiers (uses caching)
		let modifiers = self
			.modifier_srv
			.get_resource_multipliers(player_id)
			.await?;

		// Step 2: Get base rates from database
		let base_rates = self.resource_srv.get_base_rates(player_id)?;

		// Step 3: Combine base rates with modifiers to get production rates
		let production_rates = resource_operations::apply_rate_modifiers(&base_rates, &modifiers);

		// Step 4: Produce resources with the calculated rates
		self.resource_srv
			.produce(player_id, &production_rates)
			.await?;

		Ok(())
	}
}
