use std::fmt::Formatter;
use std::sync::Arc;

use axum::extract::FromRef;

use crate::configuration::Settings;
use crate::db::DbPool;
use crate::job_queue::JobQueue;

/// Shared application state
#[derive(FromRef, Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
    pub job_queue: Arc<JobQueue>,
    pub settings: Settings,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Retrieve the state from the db_pool
        let db_state = self.db_pool.state();

        f.debug_struct("AppState")
            .field("db_pool", &db_state)
            .finish()
    }
}

impl AppState {
    pub fn new(db_pool: DbPool, settings: Settings) -> AppState {
        let job_queue = JobQueue::new(db_pool.clone());
        AppState {
            db_pool: Arc::new(db_pool),
            job_queue: Arc::new(job_queue),
            settings,
        }
    }
}
