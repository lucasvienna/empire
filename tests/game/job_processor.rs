use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use empire::domain::app_state::AppState;
use empire::domain::jobs::JobType;
use empire::domain::player::resource::ResourceType;
use empire::game::modifiers::modifier_processor::ModifierProcessor;
use empire::game::modifiers::modifier_scheduler::ModifierJobPayload;
use empire::job_queue::worker_pool::WorkerPool;
use empire::job_queue::JobPriority;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::info;
use uuid::Uuid;

use crate::common::TestHarness;

#[tokio::test]
async fn test_worker_pool_lifecycle() {
	let TestHarness { app, .. } = TestHarness::new();
	let state = AppState(app);

	// Start pool
	let token = CancellationToken::new();
	let queue = Arc::clone(&state.job_queue);
	let mut worker_pool = WorkerPool::new(queue, token);

	// Start workers
	let mod_workers = ModifierProcessor::initialise_n(3, &state);
	worker_pool.add_workers(mod_workers);
	assert_eq!(worker_pool.worker_count(), 3);

	// Enqueue a test job
	let _job_id = worker_pool
		.queue
		.enqueue(
			JobType::Modifier,
			ModifierJobPayload::RecalculateResources {
				player_id: Uuid::new_v4(),
				resource_types: vec![ResourceType::Food],
			},
			JobPriority::Normal,
			Utc::now(),
		)
		.unwrap();

	// Give some time for processing
	sleep(Duration::from_secs(2)).await;

	// Shutdown with timeout
	let shutdown_timeout = sleep(Duration::from_secs(30));

	tokio::select! {
		result = worker_pool.shutdown() => {
			info!("Result is {:?}", result);
			assert!(result.is_ok(), "Worker pool shutdown failed: {:?}", result.unwrap_err());
		}
		_ = shutdown_timeout => {
			panic!("Shutdown timed out");
		}
	}

	assert_eq!(worker_pool.worker_count(), 0);
}

#[tokio::test]
async fn test_worker_pool_cancellation_token() {
	let TestHarness { app, .. } = TestHarness::new();
	let state = AppState(app);

	// Start pool
	let token = CancellationToken::new();
	let queue = Arc::clone(&state.job_queue);
	let mut worker_pool = WorkerPool::new(queue, token.clone());

	// Start workers
	let mod_workers = ModifierProcessor::initialise_n(2, &state);
	worker_pool.add_workers(mod_workers);
	assert_eq!(worker_pool.worker_count(), 2);

	// Shutdown with timeout
	let shutdown_timeout = sleep(Duration::from_secs(30));
	let monitor = worker_pool.monitor();

	sleep(Duration::from_secs(2)).await;
	token.cancel();

	tokio::select! {
		result = monitor => {
			info!("Result is {:?}", result);
			assert!(result.is_ok(), "Worker pool shutdown failed: {:?}", result.unwrap_err());
		}

		_ = shutdown_timeout => {
			panic!("Shutdown timed out");
		}
	}

	assert_eq!(worker_pool.worker_count(), 0);
}
