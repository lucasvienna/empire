use std::error::Error;

use diesel::pg::Pg;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Runs all pending migrations on the given database connection.
///
/// This function leverages Diesel's migration system to execute
/// any migrations that have not yet been applied to the database.
///
/// # Arguments
///
/// * `connection` - A mutable reference to an object implementing `MigrationHarness`
///   for a `Pg` connection (PostgreSQL).
pub fn run_pending(
	connection: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
	connection.run_pending_migrations(MIGRATIONS)?;

	Ok(())
}
