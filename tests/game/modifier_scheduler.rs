use std::sync::Arc;

use chrono::{Duration, DurationRound, TimeDelta, Utc};
use diesel::{QueryDsl, RunQueryDsl};
use empire::domain::jobs::{Job, JobStatus, JobType};
use empire::game::modifiers::modifier_scheduler::{ModifierJobPayload, ModifierScheduler};
use empire::job_queue::JobQueue;
use empire::schema::job;
use uuid::Uuid;

use crate::common::TestHarness;

#[tokio::test]
async fn test_schedule_expiration() {
	let pool = Arc::new(TestHarness::new().db_pool);
	let mut connection = pool.get().expect("Failed to get connection from pool");
	let queue = Arc::new(JobQueue::new(pool));
	let scheduler = ModifierScheduler::new(&queue);

	let modifier_id = Uuid::new_v4();
	let player_id = Uuid::new_v4();
	let expires_at = Utc::now().duration_trunc(TimeDelta::seconds(1)).unwrap() + Duration::hours(1);

	let job_id = scheduler
		.schedule_expiration(modifier_id, player_id, expires_at)
		.unwrap();

	// Verify the job was created with correct parameters
	let job = job::table
		.find(job_id)
		.first::<Job>(&mut connection)
		.unwrap();

	assert_eq!(job.job_type, JobType::Modifier);
	assert_eq!(job.status, JobStatus::Pending);
	assert_eq!(job.run_at, expires_at);

	let payload: ModifierJobPayload = serde_json::from_value(job.payload).unwrap();
	match payload {
		ModifierJobPayload::ExpireModifier {
			modifier_id: mid,
			player_id: uid,
		} => {
			assert_eq!(mid, modifier_id);
			assert_eq!(uid, player_id);
		}
		_ => panic!("Wrong payload type"),
	}
}
