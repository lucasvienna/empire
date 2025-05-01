use std::sync::Arc;
use std::time::Duration;

use axum::extract::FromRef;
use tokio::sync::broadcast::Receiver;
use tokio::time::sleep;
use tracing::{debug, error, info, instrument, trace, warn};
use ulid::Ulid;

use crate::domain::app_state::{AppPool, AppState};
use crate::domain::jobs::{Job, JobType};
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
    /// A reference to the application's database connection pool
    db_pool: AppPool,
    /// A broadcast channel receiver for handling graceful shutdowns
    shutdown_rx: Receiver<()>,
    /// Modifier service instance
    srv: ResourceService,
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
    /// * `db_pool` - A reference to the application's database connection pool
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
        let srv = ResourceService::from_ref(app_state);
        debug!("Starting worker {}", id);
        Self {
            id,
            db_pool: Arc::clone(&app_state.db_pool),
            shutdown_rx,
            srv,
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
                    match queue.get_next_job_of_type(&self.id, &JobType::Resource).await {
                        Ok(Some(job)) => {
                            // Found a job, process it
                            trace!("Worker {} is processing job {:?}", self.id, job);
                            match self.process_job(job.clone()).await {
                                Ok(()) => {
                                    trace!("Worker {} completed job {:?}", self.id, job);
                                    queue.complete_job(&job.id).await?;
                                }
                                Err(e) => {
                                    warn!("Worker {} failed job {:?}", self.id, job);
                                    trace!("Error: {}", e);
                                    queue.fail_job(&job.id, &e.to_string()).await?;
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

    async fn process_job(&self, job: Job) -> Result<(), Error> {
        info!("Processing job {:?}", job);
        assert_eq!(
            job.job_type,
            JobType::Resource,
            "Expected a modifier job, got: {}",
            job.job_type
        );
        let payload: ProductionJobPayload = serde_json::from_value(job.payload.clone())?;

        match payload {
            ProductionJobPayload::ProduceResources { players_id } => {
                match self.srv.produce_for_player(&players_id).await {
                    Ok(_) => {
                        debug!("Produced resources for player {}", players_id);
                    }
                    Err(e) => {
                        warn!("Error producing resources: {}", e);
                        debug!("Error: {:?}", e);
                        return Err(e);
                    }
                }
            }
            ProductionJobPayload::CollectResources { players_id } => {
                match self.srv.collect_resources(&players_id) {
                    Ok(_) => {
                        debug!("Collected resources for player {}", players_id);
                    }
                    Err(e) => {
                        warn!("Error collecting resources: {}", e);
                        debug!("Error: {:?}", e);
                        return Err(e);
                    }
                }
            }
        }

        Ok(())
    }
}
