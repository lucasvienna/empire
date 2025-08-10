use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, instrument, trace, warn};

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
		trace!("Creating new WorkerPool instance");
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
	#[instrument(skip(self))]
	pub async fn monitor(&mut self) -> Result<(), Error> {
		debug!(
			"Starting worker pool monitor with {} workers",
			self.workers.len()
		);
		trace!("Waiting for cancellation token to be triggered");
		self.cancellation_token.cancelled().await;
		info!(
			"Shutting down worker pool with {} workers...",
			self.workers.len()
		);

		let start = std::time::Instant::now();
		if let Err(e) = self.shutdown().await {
			warn!("Error shutting down worker pool: {}", e);
		} else {
			let duration = start.elapsed();
			info!("Worker pool shut down successfully in {:?}", duration);
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
	#[instrument(skip(self, worker))]
	pub fn add_worker(&mut self, mut worker: impl JobProcessor + Send + 'static) {
		debug!("Adding new worker to pool");
		let queue = self.queue.clone();
		let handle = tokio::spawn(async move {
			if let Err(e) = worker.run(queue).await {
				error!("Worker error: {}", e);
				trace!("Detailed worker error: {:?}", e);
			}
		});
		self.workers.push(handle);
		info!(
			"Added new worker to pool, total workers: {}",
			self.workers.len()
		);
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
	#[instrument(skip(self, workers))]
	pub fn add_workers(&mut self, workers: Vec<impl JobProcessor + Send + 'static>) {
		let worker_count = workers.len();
		debug!("Adding {} workers to pool", worker_count);

		for (i, worker) in workers.into_iter().enumerate() {
			trace!("Adding worker {}/{}", i + 1, worker_count);
			self.add_worker(worker);
		}

		info!(
			"Added {} workers to pool, total workers: {}",
			worker_count,
			self.workers.len()
		);
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
	#[instrument(skip(self))]
	pub async fn shutdown(&mut self) -> Result<(), Error> {
		let worker_count = self.workers.len();
		debug!("Starting graceful shutdown of {} workers", worker_count);

		// Signal all workers to shut down
		trace!("Sending shutdown signal to job queue");
		if let Err(e) = self.queue.shutdown() {
			error!("Failed to send shutdown signal to job queue: {}", e);
			return Err(e);
		}

		// Take ownership of the workers' vector
		let workers = std::mem::take(&mut self.workers);
		debug!(
			"Waiting for {} workers to complete current tasks",
			worker_count
		);

		// Create a timeout future
		let shutdown_timeout = tokio::time::sleep(Duration::from_secs(30));

		// Wait for all workers with timeout
		tokio::select! {
			_ = async {
				for (i, handle) in workers.into_iter().enumerate() {
					trace!("Waiting for worker {}/{} to complete", i + 1, worker_count);
					// Ignore errors from cancelled tasks
					let _ = handle.await;
				}
			} => {
				info!("All {} workers shut down successfully", worker_count);
			}
			_ = shutdown_timeout => {
				warn!("Worker shutdown timed out after 30 seconds with {} workers", worker_count);
			}
		}

		Ok(())
	}

	/// Returns the current number of active worker threads.
	#[instrument(skip(self))]
	pub fn worker_count(&self) -> usize {
		let count = self.workers.len();
		trace!("Getting worker count: {}", count);
		count
	}
}
