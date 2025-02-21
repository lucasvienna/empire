use crate::domain::error::Result;

pub mod building_levels;
pub mod buildings;
pub mod connection;
pub mod extractor;
pub mod factions;
pub mod migrations;
pub mod resources;
pub mod user_buildings;
pub mod users;

pub use connection::{DbConn, DbPool};

pub trait Repository<Entity, NewEntity, PK = i32> {
    /// get all entities
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<Entity>>;

    /// get a single entity by id
    fn get_by_id(&self, connection: &mut DbConn, id: &PK) -> Result<Entity>;

    /// add an entity to the database
    fn create(&self, connection: &mut DbConn, entity: &NewEntity) -> Result<Entity>;

    /// update an entity
    fn update(&self, connection: &mut DbConn, entity: &Entity) -> Result<Entity>;

    /// delete an entity by its id
    fn delete(&self, connection: &mut DbConn, id: &PK) -> Result<usize>;
}
