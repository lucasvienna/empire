use anyhow::Result;
use empire::configuration::get_configuration;
use empire::db::connection;
use empire::db::migrations::run_migrations;
use empire::net::server;
use empire::net::server::AppState;
use empire::{load_env, setup_tracing, shutdown_signal};
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    load_env().expect("Failed to load environment variables.");
    setup_tracing().expect("Failed to setup tracing.");
    info!("Starting Empire server...");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let pool = connection::get_pool(configuration.database);
    {
        let mut conn = pool.get()?;
        run_migrations(&mut conn).expect("Should execute pending migrations");
    }

    let (listener, router) = server::init(configuration.server).await?;
    let router = router.with_state(AppState {
        db_pool: Arc::new(pool),
    });

    info!("Listening on {}", listener.local_addr()?);

    Ok(axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?)
}
