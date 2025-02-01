use crate::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, AsChangeset, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: PK,
    pub name: String,
    pub faction: i32,
    pub data: Option<serde_json::Value>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub faction: i32,
    pub data: Option<serde_json::Value>,
}

pub type PK = Uuid;
