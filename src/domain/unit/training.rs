//! Contains domain entities for the training queue system.
//! Tracks units currently being trained by players.

use std::io::Write;
use std::str::from_utf8;

use chrono::{DateTime, Utc};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{AsExpression, FromSqlRow, deserialize, serialize};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Unit, UnitKey};
use crate::domain::jobs::JobKey;
use crate::domain::player::buildings::PlayerBuildingKey;
use crate::domain::player::{Player, PlayerKey};
use crate::schema::training_queue;

/// Unique identifier for a training queue entry
pub type TrainingQueueKey = Uuid;

/// Represents the status of a training queue entry
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
#[diesel(sql_type = crate::schema::sql_types::TrainingStatus)]
#[serde(rename_all = "snake_case")]
pub enum TrainingStatus {
	Pending,
	InProgress,
	Completed,
	Cancelled,
}

impl AsRef<str> for TrainingStatus {
	fn as_ref(&self) -> &str {
		match self {
			TrainingStatus::Pending => "pending",
			TrainingStatus::InProgress => "in_progress",
			TrainingStatus::Completed => "completed",
			TrainingStatus::Cancelled => "cancelled",
		}
	}
}

impl ToSql<crate::schema::sql_types::TrainingStatus, Pg> for TrainingStatus {
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
		out.write_all(self.as_ref().as_bytes())?;
		Ok(IsNull::No)
	}
}

impl FromSql<crate::schema::sql_types::TrainingStatus, Pg> for TrainingStatus {
	fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
		match from_utf8(bytes.as_bytes())? {
			"pending" => Ok(TrainingStatus::Pending),
			"in_progress" => Ok(TrainingStatus::InProgress),
			"completed" => Ok(TrainingStatus::Completed),
			"cancelled" => Ok(TrainingStatus::Cancelled),
			other => Err(format!("Unrecognized enum variant: {other}").into()),
		}
	}
}

/// Represents an entry in the training queue
#[derive(
	Queryable, Selectable, Identifiable, Associations, Serialize, Debug, Clone, PartialEq, Eq,
)]
#[diesel(belongs_to(Player))]
#[diesel(belongs_to(Unit))]
#[diesel(table_name = training_queue, check_for_backend(diesel::pg::Pg))]
pub struct TrainingQueueEntry {
	pub id: TrainingQueueKey,
	pub player_id: PlayerKey,
	pub building_id: PlayerBuildingKey,
	pub unit_id: UnitKey,
	pub quantity: i32,
	pub started_at: DateTime<Utc>,
	pub completed_at: Option<DateTime<Utc>>,
	pub status: TrainingStatus,
	pub job_id: Option<JobKey>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Data transfer object for creating a new training queue entry
#[derive(Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = training_queue, check_for_backend(diesel::pg::Pg))]
pub struct NewTrainingQueueEntry {
	pub player_id: PlayerKey,
	pub building_id: PlayerBuildingKey,
	pub unit_id: UnitKey,
	pub quantity: i32,
	pub status: Option<TrainingStatus>,
	pub job_id: Option<JobKey>,
}

/// Data transfer object for updating a training queue entry
#[derive(Identifiable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = training_queue, check_for_backend(diesel::pg::Pg))]
pub struct UpdateTrainingQueueEntry {
	pub id: TrainingQueueKey,
	pub quantity: Option<i32>,
	pub completed_at: Option<DateTime<Utc>>,
	pub status: Option<TrainingStatus>,
	pub job_id: Option<JobKey>,
}
