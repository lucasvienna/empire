#![allow(async_fn_in_trait)] // this isn't a public crate where this matters

use std::future::Future;
use std::sync::Arc;

use tokio::sync::broadcast;
use tracing::info;

use crate::domain::app_state::AppState;
use crate::domain::jobs::Job;
use crate::job_queue::JobQueue;
use crate::Error;

/// A trait defining the behaviour of a job processor component that handles background tasks.
///
/// The JobProcessor is responsible for:
/// - Initializing with database access and shutdown coordination
/// - Running a processing loop that monitors the job queue
/// - Processing individual jobs based on their type and payload
///
/// Implementations of this trait should handle specific job types and provide
/// appropriate processing logic for each supported job category.
pub trait JobProcessor {
    /// Creates a new instance of the job processor.
    ///
    /// # Arguments
    /// * `app_state` - A thread-safe reference to the application state
    /// * `shutdown_rx` - A broadcast channel receiver for coordinating graceful shutdown
    fn new(app_state: &AppState, shutdown_rx: broadcast::Receiver<()>) -> Self
    where
        Self: Sized;

    /// Runs the main processing loop that monitors and processes jobs from the queue.
    ///
    /// # Arguments
    /// * `queue` - A thread-safe reference to the job queue to process
    ///
    /// # Returns
    /// A Result indicating success or containing an error if processing fails
    fn run(&mut self, queue: Arc<JobQueue>) -> impl Future<Output = Result<(), Error>> + Send;

    /// Processes a single job from the queue.
    ///
    /// # Arguments
    /// * `job` - The job to be processed
    ///
    /// # Returns
    /// A Result indicating success or containing an error if job processing fails
    async fn process_job(&self, job: Job) -> Result<(), Error> {
        // This will be implemented based on job types
        info!("Processing job {:?}", job);
        Ok(())
    }
}
