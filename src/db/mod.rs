use diesel::{AsChangeset, Identifiable};

use crate::Result;

pub mod active_modifiers;
pub mod building_levels;
pub mod buildings;
pub mod connection;
pub mod extractor;
pub mod factions;
pub mod migrations;
pub mod modifiers;
pub mod player_buildings;
pub mod players;
pub mod resources;

pub use connection::{DbConn, DbPool};

pub trait Repository<Entity, NewEntity, UpdateEntity, PK = i32>
where
    UpdateEntity: Identifiable + AsChangeset,
{
    /// Creates a new repository instance from a connection pool.
    ///
    /// # Arguments
    /// * `pool` - The database connection pool
    ///
    /// # Returns
    /// A Result containing the new repository instance
    fn try_from_pool(pool: &DbPool) -> Result<Self>
    where
        Self: Sized;

    /// Creates a new repository instance from an existing database connection.
    ///
    /// # Arguments
    /// * `connection` - The database connection
    ///
    /// # Returns
    /// A Result containing the new repository instance
    fn from_connection(connection: DbConn) -> Self;

    /// get all entities
    fn get_all(&mut self) -> Result<Vec<Entity>>;

    /// get a single entity by id
    fn get_by_id(&mut self, id: &PK) -> Result<Entity>;

    /// add an entity to the database
    fn create(&mut self, entity: NewEntity) -> Result<Entity>;

    /// update an entity
    fn update(&mut self, changeset: UpdateEntity) -> Result<Entity>;

    /// delete an entity by its id
    fn delete(&mut self, id: &PK) -> Result<usize>;
}
