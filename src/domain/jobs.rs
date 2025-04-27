use std::io::Write;

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

/// Unique identifier type for jobs, implemented as a UUID
pub type JobKey = Uuid;

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
/// Represents different types of background jobs that can be processed by the system
pub enum JobType {
    /// Jobs that handle modifier-related tasks like applying or removing effects
    Modifier,
    /// Jobs related to building construction, upgrade or maintenance
    Building,
    /// Jobs for resource collection, resources or distribution
    Resource,
}

impl ToSql<crate::schema::sql_types::JobType, Pg> for JobType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            JobType::Modifier => out.write_all(b"modifier")?,
            JobType::Building => out.write_all(b"building")?,
            JobType::Resource => out.write_all(b"resource")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::JobType, Pg> for JobType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"modifier" => Ok(JobType::Modifier),
            b"building" => Ok(JobType::Building),
            b"resource" => Ok(JobType::Resource),
            _ => {
                let unrecognized_value = String::from_utf8_lossy(bytes.as_bytes());
                Err(format!("Unrecognized enum variant: {}", unrecognized_value).into())
            }
        }
    }
}

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
/// Represents the current state of a background job
pub enum JobStatus {
    /// Job is queued and waiting to be processed
    Pending,
    /// A worker is currently processing Job
    InProgress,
    /// Job has finished processing successfully
    Completed,
    /// Job encountered an error during processing
    Failed,
    /// Job was manually cancelled or terminated
    Cancelled,
}

impl ToSql<crate::schema::sql_types::JobStatus, Pg> for JobStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            JobStatus::Pending => out.write_all(b"pending")?,
            JobStatus::InProgress => out.write_all(b"in_progress")?,
            JobStatus::Completed => out.write_all(b"completed")?,
            JobStatus::Failed => out.write_all(b"failed")?,
            JobStatus::Cancelled => out.write_all(b"cancelled")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::JobStatus, Pg> for JobStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"pending" => Ok(JobStatus::Pending),
            b"in_progress" => Ok(JobStatus::InProgress),
            b"completed" => Ok(JobStatus::Completed),
            b"failed" => Ok(JobStatus::Failed),
            b"cancelled" => Ok(JobStatus::Cancelled),
            _ => {
                let unrecognized_value = String::from_utf8_lossy(bytes.as_bytes());
                Err(format!("Unrecognized enum variant: {}", unrecognized_value).into())
            }
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = job)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a background job that can be queued and processed asynchronously
/// by workers, with support for retries, priorities, and locking mechanisms
pub struct Job {
    pub id: JobKey,
    pub job_type: JobType,
    pub status: JobStatus,
    pub payload: serde_json::Value,
    pub run_at: DateTime<Utc>,
    pub last_error: Option<String>,
    pub retries: i32,
    pub max_retries: i32,
    pub priority: i32,
    pub timeout_seconds: i32,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = job)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents data required to create a new background job entry in the queue system
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

#[derive(AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = job)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents the fields that can be updated for an existing job in the queue system
pub struct UpdateJob {
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
