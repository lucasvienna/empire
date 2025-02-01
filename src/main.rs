use anyhow::Result;
use empire::configuration::{self, get_configuration};
use empire::db::connection;
use empire::db::migrations::run_migrations;
use empire::startup::launch;
use empire::telemetry;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    configuration::load_env().expect("Failed to load environment variables.");
    telemetry::init_tracing().expect("Failed to setup tracing.");

    info!("Starting Empire server...");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let pool = connection::get_pool(&configuration.database);
    {
        let mut conn = pool.get()?;
        run_migrations(&mut conn).expect("Failed to execute pending migrations.");
    }

    Ok(launch(configuration, pool).await?)
}
