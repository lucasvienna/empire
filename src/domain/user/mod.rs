mod user_email;
mod user_name;

use std::fmt;

use diesel::prelude::*;
pub use user_email::UserEmail;
pub use user_name::UserName;
use uuid::Uuid;

use crate::schema::users;

/// User Primary Key
pub type PK = Uuid;

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Clone)]
pub struct User {
    pub id: PK,
    pub name: String,
    pub pwd_hash: String,
    pub email: Option<String>,
    pub faction: i32,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("password", &"[redacted]")
            .field("email", &self.email)
            .field("faction", &self.faction)
            .finish()
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: UserName,
    pub pwd_hash: String,
    pub email: Option<UserEmail>,
    pub faction: i32,
}

impl fmt::Debug for NewUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NewUser")
            .field("name", &self.name)
            .field("password", &"[redacted]")
            .field("email", &self.email)
            .field("faction", &self.faction)
            .finish()
    }
}
