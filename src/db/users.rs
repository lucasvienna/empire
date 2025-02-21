use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::domain::error::Result;
use crate::domain::user;
use crate::domain::user::{NewUser, User};
use crate::schema::users;

pub struct UserRepository;

impl Repository<User, NewUser, user::PK> for UserRepository {
    fn get_all(&self, connection: &mut DbConn) -> Result<Vec<User>> {
        let users = users::table.select(User::as_select()).load(connection)?;
        Ok(users)
    }

    fn get_by_id(&self, connection: &mut DbConn, id: &user::PK) -> Result<User> {
        let user = users::table.find(id).first(connection)?;
        Ok(user)
    }

    fn create(&self, connection: &mut DbConn, entity: &NewUser) -> Result<User> {
        let user = diesel::insert_into(users::table)
            .values(entity)
            .returning(User::as_returning())
            .get_result(connection)?;
        Ok(user)
    }

    fn update(&self, connection: &mut DbConn, entity: &User) -> Result<User> {
        let user = diesel::update(users::table.find(entity.id))
            .set(entity)
            .get_result(connection)?;
        Ok(user)
    }

    fn delete(&self, connection: &mut DbConn, id: &user::PK) -> Result<usize> {
        let res = diesel::delete(users::table.find(id)).execute(connection)?;
        Ok(res)
    }
}

impl UserRepository {
    pub fn get_by_name(&self, connection: &mut DbConn, name: impl AsRef<str>) -> Result<User> {
        let user = users::table
            .filter(users::name.eq(name.as_ref()))
            .first(connection)?;
        Ok(user)
    }

    pub fn find_by_name(
        &self,
        connection: &mut DbConn,
        name: impl AsRef<str>,
    ) -> Result<Option<User>> {
        let user: Option<User> = users::table
            .filter(users::name.eq(name.as_ref()))
            .first(connection)
            .optional()?;
        Ok(user)
    }

    pub fn exists_by_name(&self, connection: &mut DbConn, name: impl AsRef<str>) -> Result<bool> {
        let user = self.find_by_name(connection, name)?;
        Ok(user.is_some())
    }
}
