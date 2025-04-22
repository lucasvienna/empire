use std::fmt;
use std::sync::Arc;

use diesel::prelude::*;

use crate::db::Repository;
use crate::domain::app_state::AppPool;
use crate::domain::buildings::{Building, BuildingKey, NewBuilding, UpdateBuilding};
use crate::domain::error::Result;
use crate::schema::building::dsl::*;

/// Repository for managing building entities in the database.
///
/// This struct implements the [`Repository`] trait for [`Building`] entities
/// and provides CRUD operations for building management. It handles persistence
/// and retrieval of building data.
///
/// # Fields
/// * `pool` - Thread-safe connection pool of type [`AppPool`] for database access
pub struct BuildingRepository {
    pool: AppPool,
}

impl fmt::Debug for BuildingRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BuildingRepository")
    }
}

impl Repository<Building, NewBuilding, &UpdateBuilding, BuildingKey> for BuildingRepository {
    fn new(pool: &AppPool) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    /// Retrieves all buildings from the database.
    ///
    /// # Returns
    /// * `Result<Vec<Building>>` - Vector of all [`Building`] entities
    fn get_all(&self) -> Result<Vec<Building>> {
        let mut conn = self.pool.get()?;
        let bld_list = building.select(Building::as_select()).load(&mut conn)?;
        Ok(bld_list)
    }

    /// Retrieves a single building by its ID.
    ///
    /// # Arguments
    /// * `bld_id` - Reference to [`BuildingKey`] identifying the building
    ///
    /// # Returns
    /// * `Result<Building>` - The requested [`Building`] entity
    fn get_by_id(&self, bld_id: &BuildingKey) -> Result<Building> {
        let mut conn = self.pool.get()?;
        let bld = building.find(bld_id).first(&mut conn)?;
        Ok(bld)
    }

    /// Creates a new building in the database.
    ///
    /// # Arguments
    /// * `entity` - [`NewBuilding`] struct containing the building data
    ///
    /// # Returns
    /// * `Result<Building>` - The newly created [`Building`] entity
    fn create(&self, entity: NewBuilding) -> Result<Building> {
        let mut conn = self.pool.get()?;
        let created_building = diesel::insert_into(building)
            .values(entity)
            .returning(Building::as_returning())
            .get_result(&mut conn)?;
        Ok(created_building)
    }

    /// Updates an existing building in the database.
    ///
    /// # Arguments
    /// * `changeset` - Reference to [`UpdateBuilding`] containing the changes
    ///
    /// # Returns
    /// * `Result<Building>` - The updated [`Building`] entity
    fn update(&self, changeset: &UpdateBuilding) -> Result<Building> {
        let mut conn = self.pool.get()?;
        let updated_building = diesel::update(building)
            .set(changeset)
            .get_result(&mut conn)?;
        Ok(updated_building)
    }

    /// Deletes a building from the database.
    ///
    /// # Arguments
    /// * `bld_id` - Reference to [`BuildingKey`] identifying the building to delete
    ///
    /// # Returns
    /// * `Result<usize>` - Number of deleted records
    fn delete(&self, bld_id: &BuildingKey) -> Result<usize> {
        let mut conn = self.pool.get()?;
        let deleted_count = diesel::delete(building.find(bld_id)).execute(&mut conn)?;
        Ok(deleted_count)
    }
}
