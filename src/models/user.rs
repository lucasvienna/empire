use diesel::prelude::*;

use crate::schema::users;

#[derive(Queryable, Selectable, Identifiable, AsChangeset, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: PK,
    pub name: String,
    pub faction: i32,
    pub data: Option<Vec<u8>>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub faction: i32,
    pub data: Option<Vec<u8>>,
}

pub type PK = i32;
