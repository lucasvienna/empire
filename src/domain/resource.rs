use diesel::prelude::*;

use crate::domain::user::{self, User};
use crate::schema::resources;

pub type PK = user::PK;

#[derive(Queryable, Selectable, Identifiable, AsChangeset, Associations, Debug, PartialEq)]
#[diesel(table_name = resources)]
#[diesel(primary_key(user_id))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Resource {
    pub user_id: user::PK,
    pub food: i32,
    pub wood: i32,
    pub stone: i32,
    pub gold: i32,
    pub food_cap: i32,
    pub wood_cap: i32,
    pub stone_cap: i32,
    pub gold_cap: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = resources)]
pub struct NewResource {
    pub user_id: user::PK,
    pub food: Option<i32>,
    pub wood: Option<i32>,
    pub stone: Option<i32>,
    pub gold: Option<i32>,
}

impl NewResource {
    pub fn new(user_id: user::PK) -> NewResource {
        NewResource {
            user_id,
            food: None,
            wood: None,
            stone: None,
            gold: None,
        }
    }
}
