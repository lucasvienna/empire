use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, trace, warn};
use ulid::Ulid;

use crate::db::DbPool;
use crate::domain::jobs::Job;
use crate::job_queue::JobQueue;
use crate::Error;

struct JobProcessor {
    id: String,
    pool: DbPool,
    shutdown_rx: broadcast::Receiver<()>,
}

impl JobProcessor {
    fn new(pool: DbPool, shutdown_rx: broadcast::Receiver<()>) -> Self {
        let id = format!("task-goblin-{}", Ulid::new());
        debug!("Starting worker {}", id);
        Self {
            id,
            pool,
            shutdown_rx,
        }
    }

    async fn run(&mut self, queue: Arc<JobQueue>) -> Result<(), Error> {
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

pub struct WorkerPool {
    pub queue: Arc<JobQueue>,
    workers: Vec<JoinHandle<()>>,
}

impl WorkerPool {
    pub fn new(queue: JobQueue) -> Self {
        Self {
            queue: Arc::new(queue),
            workers: Vec::new(),
        }
    }

    pub async fn start(&mut self, worker_count: usize) -> Result<(), Error> {
        info!("Starting worker pool with {} workers", worker_count);
        for _ in 0..worker_count {
            let queue = self.queue.clone();
            let shutdown_rx = self.queue.subscribe_shutdown();
            let pool = queue.pool.clone();

            let handle = tokio::spawn(async move {
                let mut processor = JobProcessor::new(pool, shutdown_rx);
                if let Err(e) = processor.run(queue).await {
                    error!("Worker error: {}", e);
                }
            });

            self.workers.push(handle);
        }
        info!("Worker pool started");

        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        // Signal all workers to shut down
        self.queue.shutdown().await?;

        // Take ownership of the workers vector
        let workers = std::mem::take(&mut self.workers);

        // Create a timeout future
        let shutdown_timeout = tokio::time::sleep(Duration::from_secs(30));

        // Wait for all workers with timeout
        tokio::select! {
            _ = async {
                for handle in workers {
                    // Ignore errors from cancelled tasks
                    let _ = handle.await;
                }
            } => {
                info!("All workers shut down successfully");
            }
            _ = shutdown_timeout => {
                warn!("Worker shutdown timed out");
            }
        }

        Ok(())
    }

    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }
}
