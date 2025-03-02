use diesel::prelude::*;

use crate::db::{DbConn, Repository};
use crate::domain::error::Result;
use crate::domain::user::{NewUser, UpdateUser, User, PK as UserKey};
use crate::schema::users::dsl::*;

pub struct UserRepository;

impl Repository<User, NewUser, UpdateUser, UserKey> for UserRepository {
    fn get_all(&self, conn: &mut DbConn) -> Result<Vec<User>> {
        let usr_list = users.select(User::as_select()).load(conn)?;
        Ok(usr_list)
    }

    fn get_by_id(&self, conn: &mut DbConn, user_id: &UserKey) -> Result<User> {
        let user = users.find(user_id).first(conn)?;
        Ok(user)
    }

    fn create(&self, conn: &mut DbConn, entity: NewUser) -> Result<User> {
        let user = diesel::insert_into(users)
            .values(entity)
            .returning(User::as_returning())
            .get_result(conn)?;
        Ok(user)
    }

    fn update(&self, conn: &mut DbConn, user_id: &UserKey, changeset: UpdateUser) -> Result<User> {
        let user = diesel::update(users.find(user_id))
            .set(changeset)
            .get_result(conn)?;
        Ok(user)
    }

    fn delete(&self, conn: &mut DbConn, user_id: &UserKey) -> Result<usize> {
        let res = diesel::delete(users.find(user_id)).execute(conn)?;
        Ok(res)
    }
}

impl UserRepository {
    pub fn find_by_id(&self, conn: &mut DbConn, user_id: &UserKey) -> Result<Option<User>> {
        let user: Option<User> = users.find(user_id).first(conn).optional()?;
        Ok(user)
    }

    pub fn get_by_name(&self, conn: &mut DbConn, usr_name: impl AsRef<str>) -> Result<User> {
        let user = users.filter(name.eq(usr_name.as_ref())).first(conn)?;
        Ok(user)
    }

    pub fn find_by_name(
        &self,
        conn: &mut DbConn,
        usr_name: impl AsRef<str>,
    ) -> Result<Option<User>> {
        let user: Option<User> = users
            .filter(name.eq(usr_name.as_ref()))
            .first(conn)
            .optional()?;
        Ok(user)
    }

    pub fn exists_by_name(&self, conn: &mut DbConn, usr_name: impl AsRef<str>) -> Result<bool> {
        let user = self.find_by_name(conn, usr_name)?;
        Ok(user.is_some())
    }
}
