use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use serde::Serialize;
use tokio::sync::broadcast;
use tracing::trace;

use crate::db::DbConn;
use crate::domain::app_state::AppPool;
use crate::domain::jobs::{Job, JobKey, JobStatus, JobType, NewJob};
use crate::schema::job::dsl::job;
use crate::schema::job::*;
use crate::{Error, ErrorKind, Result};

pub mod job_processor;
pub mod worker_pool;

#[derive(Debug, Clone, Copy)]
pub enum JobPriority {
    High = 0,
    Normal = 50,
    Low = 100,
}

#[derive(Debug, Clone)]
pub struct JobQueue {
    pool: AppPool,
    shutdown_tx: broadcast::Sender<()>,
}

/// A job request is a tuple of the job type, payload, priority, and run time.
pub type JobRequest = (JobType, serde_json::Value, JobPriority, DateTime<Utc>);

impl JobQueue {
    pub fn new(pool: AppPool) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self { pool, shutdown_tx }
    }

    /// Enqueues a new job with the specified parameters
    pub fn enqueue(
        &self,
        new_job_type: JobType,
        job_payload: impl Serialize,
        job_priority: JobPriority,
        job_run_at: DateTime<Utc>,
    ) -> Result<JobKey> {
        let mut conn = self.pool.get()?;
        let pld = serde_json::to_value(job_payload)?;

        let new_job = NewJob {
            job_type: new_job_type,
            status: JobStatus::Pending,
            payload: pld,
            run_at: job_run_at,
            last_error: None,
            max_retries: 3,
            priority: job_priority as i32,
            timeout_seconds: 300,
        };

        let job_id = diesel::insert_into(job)
            .values(&new_job)
            .returning(id)
            .get_result(&mut conn)?;

        Ok(job_id)
    }

    /// Enqueues a batch of jobs with the specified parameters
    pub fn enqueue_batch(&self, jobs: Vec<JobRequest>) -> Result<Vec<JobKey>> {
        let values: Vec<NewJob> = jobs
            .into_iter()
            .map(
                |(new_job_type, job_payload, job_priority, job_run_at)| NewJob {
                    job_type: new_job_type,
                    status: JobStatus::Pending,
                    payload: job_payload,
                    run_at: job_run_at,
                    last_error: None,
                    max_retries: 3,
                    priority: job_priority as i32,
                    timeout_seconds: 300,
                },
            )
            .collect();
        let mut conn = self.pool.get()?;

        let job_ids: Vec<JobKey> = diesel::insert_into(job)
            .values(&values)
            .returning(id)
            .get_results(&mut conn)?;

        Ok(job_ids)
    }

    /// Gets the next available job of a specific type for processing
    pub fn get_next_job_of_type(
        &self,
        worker_id: &str,
        requested_type: &JobType,
    ) -> Result<Option<Job>> {
        let mut conn = self.pool.get()?;

        let next: Option<Job> = conn.transaction(|conn| -> Result<Option<Job>> {
            let now = Utc::now();

            // First, clean up stuck jobs (those locked for too long)
            diesel::update(job)
                .filter(status.eq(JobStatus::InProgress))
                .filter(locked_at.lt(now - Duration::seconds(5))) // TODO: make job timeout configurable
                .set((
                    status.eq(JobStatus::Failed),
                    last_error.eq(Some("Job timed out after 5min")),
                    locked_at.eq(None::<DateTime<Utc>>),
                    locked_by.eq(None::<String>),
                ))
                .execute(conn)?;

            // Then select the next job to process
            let next_job: Option<Job> = job
                .filter(
                    status
                        .eq(JobStatus::Pending)
                        .or(status.eq(JobStatus::Failed).and(retries.le(max_retries))),
                )
                .filter(run_at.le(now))
                .filter(locked_at.is_null())
                .filter(job_type.eq(requested_type))
                .order_by((
                    priority.asc(), // Higher priority (lower number) first
                    run_at.asc(),   // Older jobs first
                ))
                .limit(1)
                .for_update() // This locks the row
                .get_result(conn)
                .optional()?;

            if let Some(next) = &next_job {
                // Then lock this specific job
                self.lock_job(conn, &next.id, worker_id, now)?;
            }

            Ok(next_job)
        })?;

        Ok(next)
    }

    /// Marks a job as completed
    pub fn complete_job(&self, job_id: &JobKey) -> Result<(), Error> {
        let mut conn = self.pool.get()?;

        diesel::update(job.filter(id.eq(job_id)))
            .set((
                status.eq(JobStatus::Completed),
                locked_at.eq(None::<DateTime<Utc>>),
                locked_by.eq(None::<String>),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Marks a job as failed in the database and records the error message.
    ///
    /// This method updates the job status to `Failed`, stores the error message,
    /// and releases any locks on the job. This allows the job to be potentially
    /// retried later if the maximum retry count hasn't been reached.
    ///
    /// # Parameters
    /// * `job_id` - The unique identifier of the job to mark as failed
    /// * `error` - The error message to store with the failed job. This can be any
    ///   type that can be converted to a string reference.
    ///
    /// # Returns
    /// * `Ok(())` if the job was successfully marked as failed
    /// * `Err(Error)` if there was a database error, or the job couldn't be updated
    pub fn fail_job(&self, job_id: &JobKey, error: impl AsRef<str>) -> Result<(), Error> {
        let mut conn = self.pool.get()?;

        conn.transaction(|conn| {
            // Get current job state with FOR UPDATE lock
            let cur_job: Job = job.filter(id.eq(job_id)).for_update().get_result(conn)?;

            // Only increment retries if the job failed before
            let new_retries = if let Some(err) = cur_job.last_error {
                cur_job.retries + 1
            } else {
                cur_job.retries
            };

            // Calculate next run time with exponential backoff
            let backoff_seconds = if new_retries > 0 {
                std::cmp::min(
                    300, // Max 5 minute delay
                    30 * (2_i64.pow((new_retries - 1) as u32)),
                )
            } else {
                0
            };
            let next_run_at = Utc::now() + Duration::seconds(backoff_seconds);

            diesel::update(job)
                .filter(id.eq(job_id))
                .set((
                    status.eq(JobStatus::Failed),
                    retries.eq(new_retries),
                    run_at.eq(next_run_at),
                    last_error.eq(Some(error.as_ref())),
                    locked_at.eq(None::<DateTime<Utc>>),
                    locked_by.eq(None::<String>),
                ))
                .execute(conn)
        })?;

        Ok(())
    }

    /// Creates a new receiver for shutdown signals from this job queue.
    ///
    /// This method returns a broadcast channel receiver that will be notified when
    /// the job queue initiates a shutdown. Each receiver gets its own clone of the
    /// channel, allowing multiple components to listen for shutdown signals independently.
    ///
    /// # Returns
    /// A broadcast channel receiver that can be used to await shutdown signals from
    /// this job queue. When a signal is received, the job queue is requesting shutdown.
    pub fn subscribe_shutdown(&self) -> broadcast::Receiver<()> {
        self.shutdown_tx.subscribe()
    }

    /// Signals all workers to shut down gracefully by broadcasting a shutdown signal.
    ///
    /// This method sends a shutdown signal to all active worker tasks through a broadcast channel.
    /// Workers will receive this signal and terminate their processing loops.
    ///
    /// # Returns
    /// - `Ok(usize)`: The number of receivers that got the shutdown signal
    /// - `Err`: If broadcasting the shutdown signal fails
    pub fn shutdown(&self) -> Result<usize> {
        let send_result = self.shutdown_tx.send(()).map_err(|err| {
            trace!("Errored while sending shutdown signal: {:?}", err);
            (ErrorKind::InternalError, "Failed to close job queue")
        })?;
        Ok(send_result)
    }

    fn lock_job(
        &self,
        conn: &mut DbConn,
        job_id: &JobKey,
        worker_id: &str,
        lock_time: DateTime<Utc>,
    ) -> Result<()> {
        diesel::update(job)
            .filter(id.eq(job_id))
            .set((
                status.eq(JobStatus::InProgress),
                locked_at.eq(Some(lock_time)),
                locked_by.eq(Some(worker_id)),
            ))
            .execute(conn)?;
        Ok(())
    }
}
