use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use empire::domain::jobs::JobType;
use empire::job_queue::worker_pool::WorkerPool;
use empire::job_queue::{JobPriority, JobQueue};
use serde_json::json;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

mod common;

#[tokio::test]
async fn test_worker_pool_lifecycle() {
    let pool = common::init_server().db_pool;
    let token = CancellationToken::new();
    let queue = Arc::new(JobQueue::new(Arc::new(pool)));
    let mut worker_pool = WorkerPool::new(queue, token);

    // Start workers
    worker_pool.initialise_workers(3).await.unwrap();
    assert_eq!(worker_pool.worker_count(), 3);

    // Enqueue a test job
    let _job_id = worker_pool
        .queue
        .enqueue(
            JobType::Modifier,
            json!({ "test": "payload" }),
            JobPriority::Normal,
            Utc::now(),
        )
        .await
        .unwrap();

    // Give some time for processing
    sleep(Duration::from_secs(2)).await;

    // Shutdown with timeout
    let shutdown_timeout = sleep(Duration::from_secs(30));

    tokio::select! {
        result = worker_pool.shutdown() => {
            assert!(result.is_ok());
        }
        _ = shutdown_timeout => {
            panic!("Shutdown timed out");
        }
    }

    assert_eq!(worker_pool.worker_count(), 0);
}
