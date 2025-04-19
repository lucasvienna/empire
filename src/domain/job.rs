use std::io::Write;

use chrono::{DateTime, Utc};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::jobs;

pub type PK = Uuid;

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
#[diesel(sql_type = crate::schema::sql_types::JobType)]
#[serde(rename_all = "lowercase")]
pub enum JobType {
    Modifier,
    Building,
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
pub enum JobStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
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
#[diesel(table_name = jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Job {
    pub id: PK,
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
#[diesel(table_name = jobs)]
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

#[derive(AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateJob {
    pub id: PK,
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
