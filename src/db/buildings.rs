use std::fmt;

use diesel::prelude::*;

use crate::db::{DbConn, DbPool, Repository};
use crate::domain::buildings::{Building, BuildingKey, NewBuilding, UpdateBuilding};
use crate::domain::error::Result;
use crate::schema::building::dsl::*;

/// Repository for managing building entities in the database.
///
/// This struct implements the [`Repository`] trait for [`Building`] entities
/// and provides CRUD operations for building management.
///
/// # Fields
/// * `connection` - Database connection handle of type [`DbConn`]
pub struct BuildingRepository {
    connection: DbConn,
}

impl fmt::Debug for BuildingRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BuildingRepository")
    }
}

impl Repository<Building, NewBuilding, &UpdateBuilding, BuildingKey> for BuildingRepository {
    /// Creates a new repository instance from a connection pool.
    ///
    /// # Arguments
    /// * `pool` - Reference to a [`DbPool`] connection pool
    ///
    /// # Returns
    /// * `Result<Self>` - New repository instance wrapped in Result
    fn try_from_pool(pool: &DbPool) -> Result<Self> {
        Ok(Self {
            connection: pool.get()?,
        })
    }

    /// Creates a new repository instance from an existing database connection.
    ///
    /// # Arguments
    /// * `connection` - [`DbConn`] database connection
    ///
    /// # Returns
    /// * `Self` - New repository instance
    fn from_connection(connection: DbConn) -> Self {
        Self { connection }
    }

    /// Retrieves all buildings from the database.
    ///
    /// # Returns
    /// * `Result<Vec<Building>>` - Vector of all [`Building`] entities
    fn get_all(&mut self) -> Result<Vec<Building>> {
        let bld_list = building
            .select(Building::as_select())
            .load(&mut self.connection)?;
        Ok(bld_list)
    }

    /// Retrieves a single building by its ID.
    ///
    /// # Arguments
    /// * `bld_id` - Reference to [`BuildingKey`] identifying the building
    ///
    /// # Returns
    /// * `Result<Building>` - The requested [`Building`] entity
    fn get_by_id(&mut self, bld_id: &BuildingKey) -> Result<Building> {
        let bld = building.find(bld_id).first(&mut self.connection)?;
        Ok(bld)
    }

    /// Creates a new building in the database.
    ///
    /// # Arguments
    /// * `entity` - [`NewBuilding`] struct containing the building data
    ///
    /// # Returns
    /// * `Result<Building>` - The newly created [`Building`] entity
    fn create(&mut self, entity: NewBuilding) -> Result<Building> {
        let created_building = diesel::insert_into(building)
            .values(entity)
            .returning(Building::as_returning())
            .get_result(&mut self.connection)?;
        Ok(created_building)
    }

    /// Updates an existing building in the database.
    ///
    /// # Arguments
    /// * `changeset` - Reference to [`UpdateBuilding`] containing the changes
    ///
    /// # Returns
    /// * `Result<Building>` - The updated [`Building`] entity
    fn update(&mut self, changeset: &UpdateBuilding) -> Result<Building> {
        let updated_building = diesel::update(building)
            .set(changeset)
            .get_result(&mut self.connection)?;
        Ok(updated_building)
    }

    /// Deletes a building from the database.
    ///
    /// # Arguments
    /// * `bld_id` - Reference to [`BuildingKey`] identifying the building to delete
    ///
    /// # Returns
    /// * `Result<usize>` - Number of deleted records
    fn delete(&mut self, bld_id: &BuildingKey) -> Result<usize> {
        let deleted_count = diesel::delete(building.find(bld_id)).execute(&mut self.connection)?;
        Ok(deleted_count)
    }
}
