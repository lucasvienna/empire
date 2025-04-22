use std::process::ExitCode;
use std::sync::Arc;

use anyhow::Result;
use empire::db::{connection, migrations};
use empire::startup::launch;
use empire::{configuration, telemetry};
use tracing::info;

#[tokio::main]
async fn main() -> Result<ExitCode> {
    configuration::load_env().expect("Failed to load environment variables.");
    telemetry::init_tracing().expect("Failed to setup tracing.");

    info!("Starting Empire server...");

    let settings = configuration::get_settings().expect("Failed to read configuration.");
    let pool = connection::initialize_pool(&settings.database);

    {
        let mut conn = pool.get()?;
        migrations::run_pending(&mut conn).expect("Failed to execute pending migrations.");
    }

    launch(settings, Arc::new(pool)).await?;

    Ok(ExitCode::SUCCESS)
}
