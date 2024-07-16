use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::models::error::EmpResult;
use crate::models::user::{NewUser, User};
use crate::schema::users;

pub struct UserRepository {}

impl Repository<User, NewUser<'static>> for UserRepository {
    fn get_all(&self, connection: &mut DbConn) -> EmpResult<Vec<User>> {
        let users = users::table.select(User::as_select()).load(connection)?;
        Ok(users)
    }

    fn get_by_id(&self, connection: &mut DbConn, id: &i32) -> EmpResult<User> {
        let user = users::table.find(id).first(connection)?;
        Ok(user)
    }

    fn create(&mut self, connection: &mut DbConn, entity: &NewUser<'static>) -> EmpResult<User> {
        let user = diesel::insert_into(users::table)
            .values(entity)
            .returning(User::as_returning())
            .get_result(connection)?;
        Ok(user)
    }

    fn update(&mut self, connection: &mut DbConn, entity: &User) -> EmpResult<User> {
        let user = diesel::update(users::table.find(entity.id))
            .set(entity)
            .get_result(connection)?;
        Ok(user)
    }

    fn delete(&mut self, connection: &mut DbConn, id: &i32) -> EmpResult<()> {
        diesel::delete(users::table.find(id)).execute(connection)?;
        Ok(())
    }
}
