use anyhow::{Context, Result};
use empire::db::connection::initialize_pool_from_env;
use empire::db::seeds;
use tracing_subscriber::EnvFilter;

/// Seed binary - executes all seed files from the seeds directory
fn main() -> Result<()> {
	// Initialize tracing with INFO as default level
	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
		)
		.init();

	// Establish database connection from environment
	let pool = initialize_pool_from_env(None);
	let mut conn = pool.get().context("Failed to get database connection")?;

	// Run seeds using the shared function
	seeds::run(&mut conn)?;

	Ok(())
}
