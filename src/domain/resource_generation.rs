use diesel::{Identifiable, Queryable, Selectable};

use crate::custom_schema::resource_generation;
use crate::domain::player;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = resource_generation)]
#[diesel(primary_key(player_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ResourceGeneration {
    pub player_id: player::PlayerKey,
    pub population: i64,
    pub food: i64,
    pub wood: i64,
    pub stone: i64,
    pub gold: i64,
}
