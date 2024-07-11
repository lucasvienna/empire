use crate::models::user::{NewUser, User};
use diesel::prelude::*;

pub fn create_user(conn: &mut SqliteConnection, name: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser { name, data: None };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error creating user")
}
