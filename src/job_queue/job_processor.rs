use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, trace, warn};
use ulid::Ulid;

use crate::domain::app_state::AppPool;
use crate::domain::jobs::Job;
use crate::job_queue::JobQueue;
use crate::Error;

pub struct JobProcessor {
    id: String,
    db_pool: AppPool,
    shutdown_rx: broadcast::Receiver<()>,
}

impl JobProcessor {
    pub fn new(db_pool: AppPool, shutdown_rx: broadcast::Receiver<()>) -> Self {
        let id = format!("task-goblin-{}", Ulid::new());
        debug!("Starting worker {}", id);
        Self {
            id,
            db_pool,
            shutdown_rx,
        }
    }

    pub async fn run(&mut self, queue: Arc<JobQueue>) -> Result<(), Error> {
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        loop {
            tokio::select! {
                _ = self.shutdown_rx.recv() => {
                    info!("Worker {} shutting down", self.id);
                    break;
                }
                _ = interval.tick() => {
                    match queue.get_next_job(&self.id).await {
                        Ok(Some(job)) => {
                            trace!("Worker {} is processing job {:?}", self.id, job);
                            match self.process_job(&job).await {
                                Ok(()) => {
                                    trace!("Worker {} completed job {:?}", self.id, job);
                                    queue.complete_job(job.id).await?;
                                }
                                Err(e) => {
                                    warn!("Worker {} failed job {:?}", self.id, job);
                                    queue.fail_job(job.id, &e.to_string()).await?;
                                }
                            }
                        }
                        Ok(None) => {
                            // No jobs available, continue polling
                            sleep(Duration::from_secs(1)).await;
                        }
                        Err(e) => {
                            error!("Error fetching job: {}", e);
                            sleep(Duration::from_secs(5)).await;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn process_job(&self, job: &Job) -> Result<(), Error> {
        // This will be implemented based on job types
        // For now, we'll just log the job
        info!("Processing job {:?}", job);
        Ok(())
    }
}
