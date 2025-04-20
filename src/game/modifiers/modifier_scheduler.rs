use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::job::JobType;
use crate::domain::resource::ResourceType;
use crate::domain::{job, modifier, user};
use crate::job_queue::{JobPriority, JobQueue};
use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub enum ModifierJobPayload {
    ExpireModifier {
        modifier_id: Uuid,
        user_id: Uuid,
    },
    RecalculateResources {
        user_id: Uuid,
        resource_types: Vec<ResourceType>,
    },
    UpdateModifierCache {
        user_id: Uuid,
    },
}

pub struct ModifierScheduler {
    job_queue: Arc<JobQueue>,
}

impl ModifierScheduler {
    pub fn new(job_queue: Arc<JobQueue>) -> Self {
        Self { job_queue }
    }

    /// Schedule a job to expire a modifier at the specified time
    pub async fn schedule_expiration(
        &self,
        modifier_id: modifier::PK,
        user_id: user::PK,
        expires_at: DateTime<Utc>,
    ) -> Result<job::PK, Error> {
        let payload = ModifierJobPayload::ExpireModifier {
            modifier_id,
            user_id,
        };

        self.job_queue
            .enqueue(JobType::Modifier, payload, JobPriority::Normal, expires_at)
            .await
    }

    /// Schedule an immediate recalculation of resources for a user
    pub async fn schedule_resource_recalculation(
        &self,
        user_id: user::PK,
        resource_types: Vec<ResourceType>,
    ) -> Result<job::PK, Error> {
        let payload = ModifierJobPayload::RecalculateResources {
            user_id,
            resource_types,
        };

        self.job_queue
            .enqueue(
                JobType::Modifier,
                payload,
                JobPriority::High, // Higher priority since it affects user's resources
                Utc::now(),
            )
            .await
    }

    /// Schedule a cache update for a user's modifiers
    pub async fn schedule_cache_update(
        &self,
        user_id: user::PK,
        run_at: DateTime<Utc>,
    ) -> Result<job::PK, Error> {
        let payload = ModifierJobPayload::UpdateModifierCache { user_id };

        self.job_queue
            .enqueue(
                JobType::Modifier,
                payload,
                JobPriority::Low, // Lower priority since it's just a cache update
                run_at,
            )
            .await
    }

    /// Schedule multiple cache updates for a batch of users
    pub async fn schedule_batch_cache_update(
        &self,
        user_ids: Vec<user::PK>,
        run_at: DateTime<Utc>,
    ) -> Result<Vec<job::PK>, Error> {
        let mut job_ids = Vec::with_capacity(user_ids.len());

        for user_id in user_ids {
            let job_id = self.schedule_cache_update(user_id, run_at).await?;
            job_ids.push(job_id);
        }

        Ok(job_ids)
    }
}
