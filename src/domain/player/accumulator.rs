use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::player::{Player, PlayerKey};
use crate::schema::player_accumulator;

pub type AccumulatorKey = Uuid;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = player_accumulator, check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Player))]
pub struct PlayerAccumulator {
    pub id: AccumulatorKey,
    pub player_id: PlayerKey,
    pub food: i64,
    pub wood: i64,
    pub stone: i64,
    pub gold: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
