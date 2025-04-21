use std::fmt;

use diesel::prelude::*;

use crate::db::{DbConn, DbPool, Repository};
use crate::domain::error::Result;
use crate::domain::factions::{Faction, FactionKey, NewFaction, UpdateFaction};
use crate::schema::faction::dsl::*;

/// Repository for managing faction entities in the database.
///
/// This struct provides CRUD operations for [`Faction`] entities using a database connection.
///
/// # Fields
/// * `connection` - Database connection pool
pub struct FactionRepository {
    connection: DbConn,
}

impl fmt::Debug for FactionRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FactionRepository")
    }
}

impl Repository<Faction, NewFaction, &UpdateFaction, FactionKey> for FactionRepository {
    /// Creates a new [`FactionRepository`] instance from a database connection pool.
    ///
    /// # Arguments
    /// * `pool` - The database connection pool to create the repository from
    ///
    /// # Returns
    /// A Result containing the new repository instance
    fn try_from_pool(pool: &DbPool) -> Result<Self> {
        Ok(Self {
            connection: pool.get()?,
        })
    }

    /// Creates a new [`FactionRepository`] instance from an existing database connection.
    ///
    /// # Arguments
    /// * `connection` - The database connection to create the repository from
    ///
    /// # Returns
    /// The new repository instance
    fn from_connection(connection: DbConn) -> Self {
        Self { connection }
    }

    /// Retrieves all [`Faction`] entities from the database.
    ///
    /// # Returns
    /// A Result containing a vector of all faction entities
    fn get_all(&mut self) -> Result<Vec<Faction>> {
        let fac_list = faction
            .select(Faction::as_select())
            .load(&mut self.connection)?;
        Ok(fac_list)
    }

    /// Retrieves a single [`Faction`] by its ID.
    ///
    /// # Arguments
    /// * `faction_id` - The unique identifier of the faction to retrieve
    ///
    /// # Returns
    /// A Result containing the requested faction
    fn get_by_id(&mut self, faction_id: &FactionKey) -> Result<Faction> {
        let fac = faction.find(faction_id).first(&mut self.connection)?;
        Ok(fac)
    }

    /// Creates a new [`Faction`] in the database.
    ///
    /// # Arguments
    /// * `entity` - The new faction entity to create
    ///
    /// # Returns
    /// A Result containing the created faction
    fn create(&mut self, entity: NewFaction) -> Result<Faction> {
        let created_faction = diesel::insert_into(faction)
            .values(entity)
            .returning(Faction::as_returning())
            .get_result(&mut self.connection)?;
        Ok(created_faction)
    }

    /// Updates an existing [`Faction`] in the database.
    ///
    /// # Arguments
    /// * `changeset` - The changes to apply to the faction
    ///
    /// # Returns
    /// A Result containing the updated faction
    fn update(&mut self, changeset: &UpdateFaction) -> Result<Faction> {
        let updated_faction = diesel::update(faction)
            .set(changeset)
            .get_result(&mut self.connection)?;
        Ok(updated_faction)
    }

    /// Deletes a [`Faction`] from the database by its ID.
    ///
    /// # Arguments
    /// * `faction_id` - The unique identifier of the faction to delete
    ///
    /// # Returns
    /// A Result containing the number of rows deleted
    fn delete(&mut self, faction_id: &FactionKey) -> Result<usize> {
        let rows_deleted =
            diesel::delete(faction.find(faction_id)).execute(&mut self.connection)?;
        Ok(rows_deleted)
    }
}
