use std::sync::Arc;

use axum::extract::FromRef;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, warn};

use crate::domain::app_state::AppState;
use crate::domain::jobs::{JobKey, JobType};
use crate::domain::player::PlayerKey;
use crate::job_queue::{JobPriority, JobQueue, JobRequest};
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProductionJobPayload {
    ProduceResources { players_id: PlayerKey },
    CollectResources { players_id: PlayerKey },
}

impl FromRef<AppState> for ProductionScheduler {
    fn from_ref(state: &AppState) -> Self {
        Self::new(&state.job_queue)
    }
}

pub struct ProductionScheduler {
    job_queue: Arc<JobQueue>,
}

impl ProductionScheduler {
    pub fn new(job_queue: &Arc<JobQueue>) -> Self {
        Self {
            job_queue: Arc::clone(job_queue),
        }
    }

    /// Schedules resource jobs for multiple players simultaneously.
    ///
    /// # Parameters
    /// * `player_keys` - A vector of player identifiers to schedule resources for
    /// * `produce_at` - The time when the resources should occur
    ///
    /// # Note
    /// This method silently ignores any scheduling errors for individual players
    pub async fn batch_schedule_production(
        &self,
        player_keys: Vec<PlayerKey>,
        produce_at: DateTime<Utc>,
    ) -> Result<Vec<JobKey>> {
        let new_jobs: Vec<JobRequest> = player_keys
            .into_iter()
            .map(|key| -> JobRequest {
                let payload = serde_json::to_value(ProductionJobPayload::ProduceResources {
                    players_id: key,
                })
                .unwrap_or(Value::Null); // should never fail
                (JobType::Resource, payload, JobPriority::Normal, produce_at)
            })
            .filter(|(_, payload, _, _)| Value::Null.ne(payload))
            .collect();
        self.job_queue.enqueue_batch(new_jobs)
    }

    /// Schedules a resource production job for a specific player.
    ///
    /// # Parameters
    /// * `player_id` - The unique identifier of the player
    /// * `produce_at` - The time when the resources should occur
    ///
    /// # Returns
    /// A `Result` containing the `JobKey` if successful, or an error if scheduling fails
    pub fn schedule_production(
        &self,
        player_id: &PlayerKey,
        produce_at: DateTime<Utc>,
    ) -> Result<JobKey> {
        let payload = ProductionJobPayload::ProduceResources {
            players_id: *player_id,
        };

        let job_key =
            self.job_queue
                .enqueue(JobType::Resource, payload, JobPriority::Normal, produce_at);

        match job_key {
            Ok(key) => {
                debug!("Scheduled resources job for player: {:?}", player_id);
                Ok(key)
            }
            Err(e) => {
                warn!("Error scheduling resources job: {:?}", e);
                Err(e)
            }
        }
    }
}
