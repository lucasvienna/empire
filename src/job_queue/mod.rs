use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use serde::Serialize;
use tokio::sync::broadcast;

use crate::db::DbPool;
use crate::domain::job;
use crate::domain::job::{Job, JobStatus, JobType, NewJob};
use crate::schema::jobs::dsl::jobs;
use crate::schema::jobs::*;
use crate::{Error, ErrorKind, Result};

pub mod job_processor;

#[derive(Debug, Clone, Copy)]
pub enum JobPriority {
    High = 0,
    Normal = 50,
    Low = 100,
}

#[derive(Debug, Clone)]
pub struct JobQueue {
    pool: DbPool,
    shutdown_tx: broadcast::Sender<()>,
}

impl JobQueue {
    pub fn new(pool: DbPool) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self { pool, shutdown_tx }
    }

    /// Enqueues a new job with the specified parameters
    pub async fn enqueue(
        &self,
        new_job_type: JobType,
        job_payload: impl Serialize,
        job_priority: JobPriority,
        job_run_at: DateTime<Utc>,
    ) -> Result<job::PK> {
        let mut conn = self.pool.get().expect("Failed to get database connection");
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

        let job_id = diesel::insert_into(jobs)
            .values(&new_job)
            .returning(id)
            .get_result(&mut conn)?;

        Ok(job_id)
    }

    /// Gets the next available job for processing
    pub async fn get_next_job(&self, worker_id: &str) -> Result<Option<Job>, Error> {
        let mut conn = self.pool.get().expect("Failed to get database connection");

        let job: Option<Job> = conn.transaction(|conn| -> Result<Option<Job>, Error> {
            let now = Utc::now();
            // First, select the job we want to process
            let next_job: Option<Job> = jobs
                .filter(status.eq(&JobStatus::Pending))
                .filter(run_at.le(now + Duration::seconds(1))) // avoid skipping jobs queued for this whole second + some random millis
                .filter(locked_at.is_null())
                .order_by(priority.asc())
                .limit(1)
                .for_update() // This locks the row
                .get_result(conn)
                .optional()?;

            if let Some(job) = &next_job {
                // Then lock this specific job
                diesel::update(jobs)
                    .filter(id.eq(job.id))
                    .set((
                        status.eq(JobStatus::InProgress),
                        locked_at.eq(Some(now)),
                        locked_by.eq(Some(worker_id)),
                    ))
                    .execute(conn)?;
            }

            Ok(next_job)
        })?;

        Ok(job)
    }

    /// Marks a job as completed
    pub async fn complete_job(&self, job_id: job::PK) -> Result<(), Error> {
        let mut conn = self.pool.get().expect("Failed to get database connection");

        diesel::update(jobs.filter(id.eq(&job_id)))
            .set((
                status.eq(JobStatus::Completed),
                locked_at.eq(None::<DateTime<Utc>>),
                locked_by.eq(None::<String>),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Marks a job as failed
    pub async fn fail_job(&self, job_id: job::PK, error: impl AsRef<str>) -> Result<(), Error> {
        let mut conn = self.pool.get().expect("Failed to get database connection");

        diesel::update(jobs)
            .filter(id.eq(job_id))
            .set((
                status.eq(JobStatus::Failed),
                last_error.eq(Some(error.as_ref())),
                locked_at.eq(None::<DateTime<Utc>>),
                locked_by.eq(None::<String>),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Get a shutdown receiver that can be used to listen for shutdown signals
    pub fn subscribe_shutdown(&self) -> broadcast::Receiver<()> {
        self.shutdown_tx.subscribe()
    }

    /// Signals all workers to shut down
    pub async fn shutdown(&self) -> Result<()> {
        self.shutdown_tx
            .send(())
            .map_err(|err| (ErrorKind::InternalError, "Faile to close job queue"))?;
        Ok(())
    }
}
