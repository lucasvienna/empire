use diesel::prelude::*;

use crate::domain::user::{self, User};
use crate::schema::resources_accumulator;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = resources_accumulator)]
#[diesel(primary_key(user_id))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ResourceAccumulator {
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
