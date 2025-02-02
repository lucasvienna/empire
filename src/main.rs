use anyhow::Result;
use empire::configuration;
use empire::db::connection;
use empire::db::migrations::run_migrations;
use empire::startup::launch;
use empire::telemetry;
use std::process::ExitCode;
use tracing::info;

#[tokio::main]
async fn main() -> Result<ExitCode> {
    configuration::load_env().expect("Failed to load environment variables.");
    telemetry::init_tracing().expect("Failed to setup tracing.");

    info!("Starting Empire server...");

    let settings = configuration::get().expect("Failed to read configuration.");
    let pool = connection::initialize_pool(&settings.database);
    {
        let mut conn = pool.get()?;
        run_migrations(&mut conn).expect("Failed to execute pending migrations.");
    }

    launch(settings, pool).await?;

    Ok(ExitCode::SUCCESS)
}
