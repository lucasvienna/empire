use std::fmt;
use std::sync::Arc;

use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::domain::app_state::AppPool;
use crate::domain::modifier::active_modifier::{
	ActiveModifier, ActiveModifierKey, NewActiveModifier, UpdateActiveModifier,
};
use crate::domain::player::PlayerKey;
use crate::schema::active_modifiers::dsl::*;
use crate::Result;

/// Repository for managing active modifiers in the database.
///
/// Provides CRUD operations for [`ActiveModifier`] entities and additional
/// query capabilities specific to active modifiers.
///
/// # Fields
/// * `pool` - Thread-safe connection pool of type [`AppPool`] for database access
pub struct ActiveModifiersRepository {
	pool: AppPool,
}

impl fmt::Debug for ActiveModifiersRepository {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ActiveModifiersRepository")
	}
}

impl Repository<ActiveModifier, NewActiveModifier, &UpdateActiveModifier, ActiveModifierKey>
	for ActiveModifiersRepository
{
	fn new(pool: &AppPool) -> Self {
		Self {
			pool: Arc::clone(pool),
		}
	}

	/// Retrieves all active modifiers from the database.
	///
	/// # Returns
	/// * `Result<Vec<ActiveModifier>>` - List of all active modifiers or an error
	fn get_all(&self) -> Result<Vec<ActiveModifier>> {
		let mut conn = self.pool.get()?;
		let mod_list = active_modifiers
			.select(ActiveModifier::as_select())
			.load(&mut conn)?;
		Ok(mod_list)
	}

	/// Retrieves a single active modifier by its ID.
	///
	/// # Arguments
	/// * `mod_id` - The [`ActiveModifierKey`] of the modifier to retrieve
	///
	/// # Returns
	/// * `Result<ActiveModifier>` - The requested modifier or an error
	fn get_by_id(&self, mod_id: &ActiveModifierKey) -> Result<ActiveModifier> {
		let mut conn = self.pool.get()?;
		let modifier = active_modifiers.find(mod_id).first(&mut conn)?;
		Ok(modifier)
	}

	/// Creates a new active modifier in the database.
	///
	/// # Arguments
	/// * `entity` - The [`NewActiveModifier`] to create
	///
	/// # Returns
	/// * `Result<ActiveModifier>` - The created modifier or an error
	fn create(&self, entity: NewActiveModifier) -> Result<ActiveModifier> {
		let mut conn = self.pool.get()?;
		let modifier = diesel::insert_into(active_modifiers)
			.values(entity)
			.returning(ActiveModifier::as_returning())
			.get_result(&mut conn)?;
		Ok(modifier)
	}

	/// Updates an existing active modifier in the database.
	///
	/// # Arguments
	/// * `changeset` - Reference to the [`UpdateActiveModifier`] containing the changes
	///
	/// # Returns
	/// * `Result<ActiveModifier>` - The updated modifier or an error
	fn update(&self, changeset: &UpdateActiveModifier) -> Result<ActiveModifier> {
		let mut conn = self.pool.get()?;
		let modifier = diesel::update(active_modifiers)
			.set(changeset)
			.get_result(&mut conn)?;
		Ok(modifier)
	}

	/// Deletes an active modifier from the database.
	///
	/// # Arguments
	/// * `mod_id` - The [`ActiveModifierKey`] of the modifier to delete
	///
	/// # Returns
	/// * `Result<usize>` - The number of deleted rows or an error
	fn delete(&self, mod_id: &ActiveModifierKey) -> Result<usize> {
		let mut conn = self.pool.get()?;
		let deleted_count = diesel::delete(active_modifiers.find(mod_id)).execute(&mut conn)?;
		Ok(deleted_count)
	}
}

impl ActiveModifiersRepository {
	/// Retrieves all active modifiers for a specific player.
	///
	/// # Arguments
	/// * `player_key` - Reference to the [`PlayerKey`] to filter modifiers by
	///
	/// # Returns
	/// * `Result<Vec<ActiveModifier>>` - List of active modifiers for the player or an error
	pub fn get_by_player_id(
		&self,
		conn: &mut DbConn,
		player_key: &PlayerKey,
	) -> Result<Vec<ActiveModifier>> {
		let active_mods = active_modifiers
			.filter(player_id.eq(player_key))
			.get_results(conn)?;
		Ok(active_mods)
	}
}
