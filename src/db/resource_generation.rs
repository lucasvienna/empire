use diesel::{Identifiable, Queryable, Selectable};

use crate::custom_schema::resource_generation;
use crate::domain::user;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = resource_generation)]
#[diesel(primary_key(user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ResourceGeneration {
    pub user_id: user::UserKey,
    pub population: i32,
    pub food: i32,
    pub wood: i32,
    pub stone: i32,
    pub gold: i32,
}
