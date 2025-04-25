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

    /// Monitors the worker pool until cancellation is requested.
    ///
    /// This method blocks until the cancellation token is triggered, at which point
    /// it initiates a graceful shutdown of all workers. It ensures that all workers
    /// complete their current tasks and terminate properly.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if shutdown completes successfully, or an [`Error`] if shutdown fails
    pub async fn monitor(&mut self) -> Result<(), Error> {
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

    /// Adds a new worker thread to the pool.
    ///
    /// This method allows dynamically expanding the worker pool by adding new worker
    /// threads at runtime. Each worker is tracked and managed as part of the pool's
    /// lifecycle.
    ///
    /// # Arguments
    ///
    /// * `worker` - A type implementing [`JobProcessor`] that will process jobs from the queue.
    ///   Must be [`Send`] and have a static lifetime.
    ///
    /// # Type Parameters
    ///
    /// The worker type must implement:
    /// - [`JobProcessor`] for job processing functionality
    /// - [`Send`] for thread safety
    /// - `'static` lifetime for thread spawning
    pub fn add_worker(&mut self, mut worker: impl JobProcessor + Send + 'static) {
        info!("Adding new worker to pool");
        let queue = self.queue.clone();
        let handle = tokio::spawn(async move {
            if let Err(e) = worker.run(queue).await {
                error!("Worker error: {}", e);
            }
        });
        self.workers.push(handle);
    }

    /// Adds multiple worker threads to the pool at once.
    ///
    /// This method provides a convenient way to add multiple workers simultaneously.
    /// Each worker will process jobs from the shared queue independently.
    ///
    /// # Arguments
    ///
    /// * `workers` - A vector of types implementing [`JobProcessor`] that will process jobs from the queue.
    ///   Each worker must be [`Send`] and have a static lifetime.
    ///
    /// # Type Parameters
    ///
    /// The worker type must implement:
    /// - [`JobProcessor`] for job processing functionality
    /// - [`Send`] for thread safety
    /// - `'static` lifetime for thread spawning
    pub fn add_workers(&mut self, workers: Vec<impl JobProcessor + Send + 'static>) {
        for worker in workers {
            self.add_worker(worker);
        }
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

        // Take ownership of the workers' vector
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
