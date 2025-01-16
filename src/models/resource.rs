use diesel::prelude::*;

use crate::schema::resources;

#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = resources)]
#[diesel(primary_key(user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Resource {
    pub user_id: i32,
    pub food: i32,
    pub wood: i32,
    pub stone: i32,
    pub gold: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = resources)]
pub struct NewResource {
    pub user_id: i32,
    pub food: Option<i32>,
    pub wood: Option<i32>,
    pub stone: Option<i32>,
    pub gold: Option<i32>,
}

pub type PK = i32;

impl NewResource {
    pub fn new(user_id: i32) -> NewResource {
        NewResource {
            user_id,
            food: None,
            wood: None,
            stone: None,
            gold: None,
        }
    }
}
