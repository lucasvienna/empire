use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use diesel::PgConnection;
use diesel::connection::SimpleConnection;
use tracing::{info, warn};

/// Execute all seed files from the seeds directory in alphabetical order
///
/// This function reads all .sql files from the `seeds/` directory and executes
/// them in order using batch execution. Each seed file should be idempotent
/// using `ON CONFLICT DO NOTHING` clauses.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection
///
/// # Errors
///
/// Returns an error if:
/// - The seeds directory cannot be read
/// - A seed file cannot be read
/// - SQL execution fails
pub fn run(conn: &mut PgConnection) -> Result<()> {
	info!("Running database seeds...");

	// Get seeds directory path
	let seeds_dir = Path::new("seeds");
	if !seeds_dir.exists() {
		warn!("Seeds directory does not exist, skipping seeding");
		return Ok(());
	}

	// Read all seed files and sort them by name
	let mut seed_files: Vec<_> = fs::read_dir(seeds_dir)
		.context("Failed to read seeds directory")?
		.filter_map(|entry| entry.ok())
		.filter(|entry| {
			entry
				.path()
				.extension()
				.map(|ext| ext == "sql")
				.unwrap_or(false)
		})
		.collect();

	seed_files.sort_by_key(|entry| entry.file_name());

	if seed_files.is_empty() {
		info!("No seed files found in seeds directory");
		return Ok(());
	}

	// Execute each seed file
	for entry in seed_files {
		let path = entry.path();
		let filename = path.file_name().unwrap().to_string_lossy();

		info!("Executing seed file: {}", filename);

		let sql = fs::read_to_string(&path)
			.with_context(|| format!("Failed to read seed file: {}", filename))?;

		// Use batch_execute to run multiple SQL statements from the file
		conn.batch_execute(&sql)
			.with_context(|| format!("Failed to execute seed file: {}", filename))?;

		info!("Successfully executed seed file: {}", filename);
	}

	info!("Database seeds completed successfully");

	Ok(())
}
