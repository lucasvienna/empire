use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::domain::user::{self, User};
use crate::schema::user_accumulator;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = user_accumulator)]
#[diesel(primary_key(user_id))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserAccumulator {
    pub user_id: user::UserKey,
    pub food: i32,
    pub wood: i32,
    pub stone: i32,
    pub gold: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
