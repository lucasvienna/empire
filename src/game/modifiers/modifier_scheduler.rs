use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::jobs::JobType;
use crate::domain::player::resource::ResourceType;
use crate::domain::player::PlayerKey;
use crate::domain::{jobs, modifier};
use crate::job_queue::{JobPriority, JobQueue};
use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub enum ModifierJobPayload {
    ExpireModifier {
        modifier_id: Uuid,
        player_id: PlayerKey,
    },
    RecalculateResources {
        player_id: PlayerKey,
        resource_types: Vec<ResourceType>,
    },
    UpdateModifierCache {
        player_id: PlayerKey,
    },
}

pub struct ModifierScheduler {
    job_queue: Arc<JobQueue>,
}

impl ModifierScheduler {
    pub fn new(job_queue: &Arc<JobQueue>) -> Self {
        Self {
            job_queue: Arc::clone(job_queue),
        }
    }

    /// Schedule a job to expire a modifier at the specified time
    pub async fn schedule_expiration(
        &self,
        modifier_id: modifier::ModifierKey,
        player_id: PlayerKey,
        expires_at: DateTime<Utc>,
    ) -> Result<jobs::JobKey, Error> {
        let payload = ModifierJobPayload::ExpireModifier {
            modifier_id,
            player_id,
        };

        self.job_queue
            .enqueue(JobType::Modifier, payload, JobPriority::Normal, expires_at)
            .await
    }

    /// Schedule an immediate recalculation of resources for a player
    pub async fn schedule_resource_recalculation(
        &self,
        player_id: PlayerKey,
        resource_types: Vec<ResourceType>,
    ) -> Result<jobs::JobKey, Error> {
        let payload = ModifierJobPayload::RecalculateResources {
            player_id,
            resource_types,
        };

        self.job_queue
            .enqueue(
                JobType::Modifier,
                payload,
                JobPriority::High, // Higher priority since it affects player's resources
                Utc::now(),
            )
            .await
    }

    /// Schedule a cache update for a player's modifiers
    pub async fn schedule_cache_update(
        &self,
        player_id: PlayerKey,
        run_at: DateTime<Utc>,
    ) -> Result<jobs::JobKey, Error> {
        let payload = ModifierJobPayload::UpdateModifierCache { player_id };

        self.job_queue
            .enqueue(
                JobType::Modifier,
                payload,
                JobPriority::Low, // Lower priority since it's just a cache update
                run_at,
            )
            .await
    }

    /// Schedule multiple cache updates for a batch of players
    pub async fn schedule_batch_cache_update(
        &self,
        player_ids: Vec<PlayerKey>,
        run_at: DateTime<Utc>,
    ) -> Result<Vec<jobs::JobKey>, Error> {
        let mut job_ids = Vec::with_capacity(player_ids.len());

        for player_id in player_ids {
            let job_id = self.schedule_cache_update(player_id, run_at).await?;
            job_ids.push(job_id);
        }

        Ok(job_ids)
    }
}
