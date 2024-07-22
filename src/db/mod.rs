use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;

use crate::models::error::EmpResult;

pub mod building_levels;
pub mod buildings;
pub mod conn;
pub mod factions;
pub mod migrations;
pub mod resources;
pub mod user_buildings;
pub mod users;

pub type DbConn = PooledConnection<ConnectionManager<SqliteConnection>>;

pub trait Repository<Entity, NewEntity, PK = i32> {
    /// get all entities
    fn get_all(&self, connection: &mut DbConn) -> EmpResult<Vec<Entity>>;

    /// get a single entity by id
    fn get_by_id(&self, connection: &mut DbConn, id: &PK) -> EmpResult<Entity>;

    /// add an entity to the database
    fn create(&mut self, connection: &mut DbConn, entity: &NewEntity) -> EmpResult<Entity>;

    /// update an entity
    fn update(&mut self, connection: &mut DbConn, entity: &Entity) -> EmpResult<Entity>;

    /// delete an entity by its id
    fn delete(&mut self, connection: &mut DbConn, id: &PK) -> EmpResult<()>;
}
