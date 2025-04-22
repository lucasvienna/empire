use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::job_queue::job_processor::JobProcessor;
use crate::job_queue::JobQueue;
use crate::Error;

/// A pool of worker threads that process jobs from a [`JobQueue`].
///
/// The `WorkerPool` manages a collection of worker threads that continuously poll
/// for and process jobs from a shared queue. It handles worker lifecycle management,
/// graceful shutdown, and coordination between workers.
pub struct WorkerPool {
    pub queue: Arc<JobQueue>,
    workers: Vec<JoinHandle<()>>,
    cancellation_token: CancellationToken,
}

impl WorkerPool {
    /// Creates a new [`WorkerPool`] instance.
    ///
    /// # Arguments
    ///
    /// * `queue` - The shared [`JobQueue`] that workers will process jobs from
    /// * `token` - A [`CancellationToken`] used to signal shutdown to the worker pool
    pub fn new(queue: Arc<JobQueue>, token: CancellationToken) -> Self {
        Self {
            queue,
            workers: Vec::new(),
            cancellation_token: token,
        }
    }

    /// Runs the worker pool until cancellation is requested.
    ///
    /// This method blocks until the cancellation token is triggered, at which point
    /// it initiates a graceful shutdown of all workers. It ensures that all workers
    /// complete their current tasks and terminate properly.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if shutdown completes successfully, or an [`Error`] if shutdown fails
    pub async fn run(&mut self) -> Result<(), Error> {
        info!("Starting worker pool...");
        self.cancellation_token.cancelled().await;
        info!("Shutting down worker pool...");
        if let Err(e) = self.shutdown().await {
            warn!("Error shutting down worker pool: {}", e);
        } else {
            info!("Worker pool shut down");
        }
        Ok(())
    }

    /// Initialises the specified number of worker threads.
    ///
    /// # Arguments
    ///
    /// * `worker_count` - The number of worker threads to spawn
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all workers are successfully spawned, or an [`Error`] if initialisation fails
    pub async fn initialise_workers(&mut self, worker_count: usize) -> Result<(), Error> {
        info!("Initialising worker pool with {} workers", worker_count);
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

    /// Initiates a graceful shutdown of all worker threads.
    ///
    /// This method signals all workers to stop and waits for them to complete
    /// their current tasks. If workers do not shut down within 30 seconds,
    /// a timeout warning is logged.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if shutdown completes successfully, or an [`Error`] if shutdown fails
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

    /// Returns the current number of active worker threads.
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }
}
