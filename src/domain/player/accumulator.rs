use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::domain::player::{Player, PlayerKey};
use crate::schema::player_accumulator;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = player_accumulator, check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Player), primary_key(player_id))]
pub struct PlayerAccumulator {
    pub player_id: PlayerKey,
    pub food: i32,
    pub wood: i32,
    pub stone: i32,
    pub gold: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
