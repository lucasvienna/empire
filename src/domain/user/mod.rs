mod user_email;
mod user_name;

use std::fmt;

use diesel::prelude::*;
pub use user_email::UserEmail;
pub use user_name::UserName;
use uuid::Uuid;

use crate::domain::faction::FactionCode;
use crate::schema::users;

/// User Primary Key
pub type PK = Uuid;

#[derive(Queryable, Selectable, Identifiable, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = users, check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: PK,
    pub name: String,
    pub pwd_hash: String,
    pub email: Option<String>,
    pub faction: FactionCode,
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

#[derive(Insertable, PartialEq, Eq)]
#[diesel(table_name = users, check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub name: UserName,
    pub pwd_hash: String,
    pub email: Option<UserEmail>,
    pub faction: FactionCode,
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

#[derive(AsChangeset, Clone, PartialEq, Eq)]
#[diesel(table_name = users, check_for_backend(diesel::pg::Pg))]
pub struct UpdateUser {
    pub name: Option<UserName>,
    pub pwd_hash: Option<String>,
    pub email: Option<UserEmail>,
    pub faction: Option<FactionCode>,
}

impl fmt::Debug for UpdateUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NewUser")
            .field("name", &self.name)
            .field("password", &"[redacted]")
            .field("email", &self.email)
            .field("faction", &self.faction)
            .finish()
    }
}
