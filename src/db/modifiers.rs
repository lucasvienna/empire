use std::fmt;
use std::sync::Arc;

use diesel::prelude::*;

use crate::db::Repository;
use crate::domain::app_state::AppPool;
use crate::domain::modifier::{Modifier, ModifierKey, NewModifier, UpdateModifier};
use crate::schema::modifiers::dsl::*;
use crate::Result;

/// Repository for managing modifier entities in the database.
///
/// This struct provides CRUD operations for [`Modifier`] entities using a database connection.
///
/// # Fields
/// * `pool` - Thread-safe connection pool of type [`AppPool`] for database access
pub struct ModifiersRepository {
	pool: AppPool,
}

impl fmt::Debug for ModifiersRepository {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ModifiersRepository")
	}
}

/// Implementation of the [`Repository`] trait for [`ModifiersRepository`].
impl Repository<Modifier, NewModifier, &UpdateModifier, ModifierKey> for ModifiersRepository {
	fn new(pool: &AppPool) -> Self {
		Self {
			pool: Arc::clone(pool),
		}
	}

	/// Retrieves all modifiers from the database.
	///
	/// # Returns
	/// A Result containing a vector of [`Modifier`] entities
	fn get_all(&self) -> Result<Vec<Modifier>> {
		let mut conn = self.pool.get()?;
		let mod_list = modifiers.select(Modifier::as_select()).load(&mut conn)?;
		Ok(mod_list)
	}

	/// Retrieves a single modifier by its ID.
	///
	/// # Arguments
	/// * `modifier_id` - The unique identifier of the modifier
	///
	/// # Returns
	/// A Result containing the found [`Modifier`]
	fn get_by_id(&self, modifier_id: &ModifierKey) -> Result<Modifier> {
		let mut conn = self.pool.get()?;
		let modifier = modifiers.find(modifier_id).first(&mut conn)?;
		Ok(modifier)
	}

	/// Creates a new modifier in the database.
	///
	/// # Arguments
	/// * `entity` - The [`NewModifier`] to create
	///
	/// # Returns
	/// A Result containing the created [`Modifier`]
	fn create(&self, entity: NewModifier) -> Result<Modifier> {
		let mut conn = self.pool.get()?;
		let modifier = diesel::insert_into(modifiers)
			.values(entity)
			.returning(Modifier::as_returning())
			.get_result(&mut conn)?;
		Ok(modifier)
	}

	/// Updates an existing modifier in the database.
	///
	/// # Arguments
	/// * `changeset` - The [`UpdateModifier`] containing the changes
	///
	/// # Returns
	/// A Result containing the updated [`Modifier`]
	fn update(&self, changeset: &UpdateModifier) -> Result<Modifier> {
		let mut conn = self.pool.get()?;
		let modifier = diesel::update(modifiers)
			.set(changeset)
			.get_result(&mut conn)?;
		Ok(modifier)
	}

	/// Deletes a modifier from the database.
	///
	/// # Arguments
	/// * `modifier_id` - The unique identifier of the modifier to delete
	///
	/// # Returns
	/// A Result containing the number of affected rows
	fn delete(&self, modifier_id: &ModifierKey) -> Result<usize> {
		let mut conn = self.pool.get()?;
		let deleted_count = diesel::delete(modifiers.find(modifier_id)).execute(&mut conn)?;
		Ok(deleted_count)
	}
}
