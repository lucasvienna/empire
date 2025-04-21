use std::fmt;

use diesel::prelude::*;

use crate::db::{DbConn, DbPool, Repository};
use crate::domain::modifier::{Modifier, ModifierKey, NewModifier, UpdateModifier};
use crate::schema::modifiers::dsl::*;
use crate::Result;

/// Repository for managing modifier entities in the database.
///
/// This struct provides CRUD operations for [`Modifier`] entities using a database connection.
///
/// # Fields
/// * `connection` - Database connection pool
pub struct ModifiersRepository {
    connection: DbConn,
}

impl fmt::Debug for ModifiersRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModifiersRepository")
    }
}

/// Implementation of the [`Repository`] trait for [`ModifiersRepository`].
impl Repository<Modifier, NewModifier, &UpdateModifier, ModifierKey> for ModifiersRepository {
    fn try_from_pool(pool: &DbPool) -> Result<Self> {
        Ok(Self {
            connection: pool.get()?,
        })
    }

    fn from_connection(connection: DbConn) -> Self {
        Self { connection }
    }

    /// Retrieves all modifiers from the database.
    ///
    /// # Returns
    /// A Result containing a vector of [`Modifier`] entities
    fn get_all(&mut self) -> Result<Vec<Modifier>> {
        let mod_list = modifiers
            .select(Modifier::as_select())
            .load(&mut self.connection)?;
        Ok(mod_list)
    }

    /// Retrieves a single modifier by its ID.
    ///
    /// # Arguments
    /// * `modifier_id` - The unique identifier of the modifier
    ///
    /// # Returns
    /// A Result containing the found [`Modifier`]
    fn get_by_id(&mut self, modifier_id: &ModifierKey) -> Result<Modifier> {
        let modifier = modifiers.find(modifier_id).first(&mut self.connection)?;
        Ok(modifier)
    }

    /// Creates a new modifier in the database.
    ///
    /// # Arguments
    /// * `entity` - The [`NewModifier`] to create
    ///
    /// # Returns
    /// A Result containing the created [`Modifier`]
    fn create(&mut self, entity: NewModifier) -> Result<Modifier> {
        let modifier = diesel::insert_into(modifiers)
            .values(entity)
            .returning(Modifier::as_returning())
            .get_result(&mut self.connection)?;
        Ok(modifier)
    }

    /// Updates an existing modifier in the database.
    ///
    /// # Arguments
    /// * `changeset` - The [`UpdateModifier`] containing the changes
    ///
    /// # Returns
    /// A Result containing the updated [`Modifier`]
    fn update(&mut self, changeset: &UpdateModifier) -> Result<Modifier> {
        let modifier = diesel::update(modifiers)
            .set(changeset)
            .get_result(&mut self.connection)?;
        Ok(modifier)
    }

    /// Deletes a modifier from the database.
    ///
    /// # Arguments
    /// * `modifier_id` - The unique identifier of the modifier to delete
    ///
    /// # Returns
    /// A Result containing the number of affected rows
    fn delete(&mut self, modifier_id: &ModifierKey) -> Result<usize> {
        let deleted_count =
            diesel::delete(modifiers.find(modifier_id)).execute(&mut self.connection)?;
        Ok(deleted_count)
    }
}
