use std::fmt;

use diesel::prelude::*;

use crate::db::{DbConn, DbPool, Repository};
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
/// * `connection` - Database connection pool
pub struct ActiveModifiersRepository {
    connection: DbConn,
}

impl fmt::Debug for ActiveModifiersRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ActiveModifiersRepository")
    }
}

impl Repository<ActiveModifier, NewActiveModifier, &UpdateActiveModifier, ActiveModifierKey>
    for ActiveModifiersRepository
{
    fn try_from_pool(pool: &DbPool) -> Result<Self> {
        Ok(Self {
            connection: pool.get()?,
        })
    }

    fn from_connection(connection: DbConn) -> Self {
        Self { connection }
    }

    /// Retrieves all active modifiers from the database.
    ///
    /// # Returns
    /// * `Result<Vec<ActiveModifier>>` - List of all active modifiers or an error
    fn get_all(&mut self) -> Result<Vec<ActiveModifier>> {
        let mod_list = active_modifiers
            .select(ActiveModifier::as_select())
            .load(&mut self.connection)?;
        Ok(mod_list)
    }

    /// Retrieves a single active modifier by its ID.
    ///
    /// # Arguments
    /// * `mod_id` - The [`ActiveModifierKey`] of the modifier to retrieve
    ///
    /// # Returns
    /// * `Result<ActiveModifier>` - The requested modifier or an error
    fn get_by_id(&mut self, mod_id: &ActiveModifierKey) -> Result<ActiveModifier> {
        let modifier = active_modifiers.find(mod_id).first(&mut self.connection)?;
        Ok(modifier)
    }

    /// Creates a new active modifier in the database.
    ///
    /// # Arguments
    /// * `entity` - The [`NewActiveModifier`] to create
    ///
    /// # Returns
    /// * `Result<ActiveModifier>` - The created modifier or an error
    fn create(&mut self, entity: NewActiveModifier) -> Result<ActiveModifier> {
        let modifier = diesel::insert_into(active_modifiers)
            .values(entity)
            .returning(ActiveModifier::as_returning())
            .get_result(&mut self.connection)?;
        Ok(modifier)
    }

    /// Updates an existing active modifier in the database.
    ///
    /// # Arguments
    /// * `changeset` - Reference to the [`UpdateActiveModifier`] containing the changes
    ///
    /// # Returns
    /// * `Result<ActiveModifier>` - The updated modifier or an error
    fn update(&mut self, changeset: &UpdateActiveModifier) -> Result<ActiveModifier> {
        let modifier = diesel::update(active_modifiers)
            .set(changeset)
            .get_result(&mut self.connection)?;
        Ok(modifier)
    }

    /// Deletes an active modifier from the database.
    ///
    /// # Arguments
    /// * `mod_id` - The [`ActiveModifierKey`] of the modifier to delete
    ///
    /// # Returns
    /// * `Result<usize>` - The number of deleted rows or an error
    fn delete(&mut self, mod_id: &ActiveModifierKey) -> Result<usize> {
        let deleted_count =
            diesel::delete(active_modifiers.find(mod_id)).execute(&mut self.connection)?;
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
    pub fn get_by_player_id(&mut self, player_key: &PlayerKey) -> Result<Vec<ActiveModifier>> {
        let active_mods = active_modifiers
            .filter(player_id.eq(player_key))
            .get_results(&mut self.connection)?;
        Ok(active_mods)
    }
}
