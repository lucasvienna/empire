use diesel::{AsChangeset, Identifiable};

use crate::domain::app_state::AppPool;
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
pub mod player_sessions;
pub mod players;
pub mod resources;

pub use connection::{DbConn, DbPool};

pub trait Repository<Entity, NewEntity, UpdateEntity, EntityKey>
where
    UpdateEntity: Identifiable + AsChangeset,
{
    /// Creates a new repository instance from a connection pool.
    ///
    /// # Arguments
    /// * `pool` - Reference to a [`AppPool`] connection pool
    ///
    /// # Returns
    /// * `Self` - New repository instance
    fn new(pool: &AppPool) -> Self;

    /// get all entities
    fn get_all(&self) -> Result<Vec<Entity>>;

    /// get a single entity by id
    fn get_by_id(&self, key: &EntityKey) -> Result<Entity>;

    /// add an entity to the database
    fn create(&self, entity: NewEntity) -> Result<Entity>;

    /// update an entity
    fn update(&self, changeset: UpdateEntity) -> Result<Entity>;

    /// delete an entity by its id
    fn delete(&self, key: &EntityKey) -> Result<usize>;
}
