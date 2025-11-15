use anyhow::{Context, Result};
use empire::db::connection::initialize_pool_from_env;
use empire::db::seeds;

/// Seed binary - executes all seed files from the seeds directory
fn main() -> Result<()> {
	// Initialize tracing
	tracing_subscriber::fmt::init();

	// Establish database connection from environment
	let pool = initialize_pool_from_env(None);
	let mut conn = pool.get().context("Failed to get database connection")?;

	// Run seeds using the shared function
	seeds::run(&mut conn)?;

	Ok(())
}
