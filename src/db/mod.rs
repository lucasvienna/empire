use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;

use crate::models::error::EmpResult;

pub mod buildings;
pub mod conn;
pub mod factions;
pub mod migrations;
pub mod users;

pub type DbConn = PooledConnection<ConnectionManager<SqliteConnection>>;

pub trait Repository<T, S> {
    /// get all entities
    fn get_all(&self, connection: &mut DbConn) -> EmpResult<Vec<T>>;

    /// get a single entity by id
    fn get_by_id(&self, connection: &mut DbConn, id: &i32) -> EmpResult<T>;

    /// add an entity to the database
    fn create(&mut self, connection: &mut DbConn, entity: &S) -> EmpResult<T>;

    /// update an entity
    fn update(&mut self, connection: &mut DbConn, entity: &T) -> EmpResult<T>;

    /// delete an entity by its id
    fn delete(&mut self, connection: &mut DbConn, id: &i32) -> EmpResult<()>;
}
