use std::io::Write;
use std::str::from_utf8;

use chrono::{DateTime, Utc};
use derive_more::Display;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::job;

/// Strongly typed alias for job identifier using UUID for clarity.
pub type JobKey = Uuid;

/// Enumerates valid job categories with PostgreSQL and serde integration.
/// Derives facilitate conversion to/from DB and serialization.
#[derive(
    AsExpression,
    FromSqlRow,
    Serialize,
    Deserialize,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[diesel(sql_type = crate::schema::sql_types::JobType)]
#[serde(rename_all = "lowercase")]
pub enum JobType {
    /// Modifier related tasks such as applying or removing game modifiers.
    Modifier,
    /// Building-related tasks like construction or upgrades.
    Building,
    /// Resource-related tasks such as gathering or distribution.
    Resource,
}

impl JobType {
    /// Returns a static string slice for DB serialization.
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            JobType::Modifier => "modifier",
            JobType::Building => "building",
            JobType::Resource => "resource",
        }
    }
}

impl ToSql<crate::schema::sql_types::JobType, Pg> for JobType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.as_str().as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::JobType, Pg> for JobType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match from_utf8(bytes.as_bytes())? {
            "modifier" => Ok(JobType::Modifier),
            "building" => Ok(JobType::Building),
            "resource" => Ok(JobType::Resource),
            other => Err(format!("Unrecognized job type: {other}").into()),
        }
    }
}

/// Represents status of a job with Diesel and serde support.
/// Explicit variants clarify job lifecycle states.
#[derive(
    AsExpression,
    FromSqlRow,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[diesel(sql_type = crate::schema::sql_types::JobStatus)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    /// Job is waiting in the queue.
    Pending,
    /// Job is currently being processed.
    InProgress,
    /// Job completed successfully.
    Completed,
    /// Job failed during processing.
    Failed,
    /// Job was cancelled.
    Cancelled,
}

impl JobStatus {
    /// Returns a static string slice for DB serialization.
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            JobStatus::Pending => "pending",
            JobStatus::InProgress => "in_progress",
            JobStatus::Completed => "completed",
            JobStatus::Failed => "failed",
            JobStatus::Cancelled => "cancelled",
        }
    }
}

impl ToSql<crate::schema::sql_types::JobStatus, Pg> for JobStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.as_str().as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::JobStatus, Pg> for JobStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        // Match byte slice directly for performance.
        match from_utf8(bytes.as_bytes())? {
            "pending" => Ok(JobStatus::Pending),
            "in_progress" => Ok(JobStatus::InProgress),
            "completed" => Ok(JobStatus::Completed),
            "failed" => Ok(JobStatus::Failed),
            "cancelled" => Ok(JobStatus::Cancelled),
            other => Err(format!("Unrecognized job status: {other}").into()),
        }
    }
}

/// Represents a background job entity with comprehensive metadata.
/// Diesel derives support querying, updating, and identifying by id.
#[derive(Queryable, Selectable, Identifiable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = job)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Job {
    /// Unique job identifier.
    pub id: JobKey,
    /// Job category/type.
    pub job_type: JobType,
    /// Current processing state.
    pub status: JobStatus,
    /// JSON payload containing job-specific data.
    pub payload: serde_json::Value,
    /// Scheduled time for execution.
    pub run_at: DateTime<Utc>,
    /// Error message for last failure, if any.
    pub last_error: Option<String>,
    /// Number of times the job has been retried.
    pub retries: i32,
    /// Maximum allowed retry attempts.
    pub max_retries: i32,
    /// Priority for job scheduling.
    pub priority: i32,
    /// Timeout threshold in seconds.
    pub timeout_seconds: i32,
    /// Timestamp when the job was locked for processing.
    pub locked_at: Option<DateTime<Utc>>,
    /// Identifier of the worker that locked the job.
    pub locked_by: Option<String>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
}

/// Data structure for inserting new jobs into the queue.
/// Does not include `id` or mutable fields like retries or locks.
#[derive(Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = job)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewJob {
    pub job_type: JobType,
    pub status: JobStatus,
    pub payload: serde_json::Value,
    pub run_at: DateTime<Utc>,
    pub last_error: Option<String>,
    pub max_retries: i32,
    pub priority: i32,
    pub timeout_seconds: i32,
}

/// Data structure for updating existing jobs.
/// All fields except for `id` are optional to allow partial updates.
#[derive(AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = job)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateJob {
    /// Unique job identifier to update.
    pub id: JobKey,
    pub job_type: Option<JobType>,
    pub status: Option<JobStatus>,
    pub payload: Option<serde_json::Value>,
    pub run_at: DateTime<Utc>,
    pub last_error: Option<String>,
    pub retries: Option<i32>,
    pub max_retries: Option<i32>,
    pub priority: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<String>,
}
