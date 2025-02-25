use std::fmt::Formatter;
use std::sync::Arc;

use crate::configuration::Settings;
use crate::db::DbPool;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
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
        AppState {
            db_pool: Arc::new(db_pool),
            settings,
        }
    }
}
