use diesel::{Identifiable, Queryable, Selectable};

use crate::custom_schema::resource_generation;
use crate::domain::player;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = resource_generation, check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Player), primary_key(player_id))]
pub struct ResourceGeneration {
	pub player_id: player::PlayerKey,
	pub population: i64,
	/// Food per hour
	pub food: i64,
	/// Wood per hour
	pub wood: i64,
	/// Stone per hour
	pub stone: i64,
	/// Gold per hour
	pub gold: i64,
	/// Represents the cap on accumulated food in the system.
	pub food_acc_cap: i64,
	/// Represents the cap on accumulated wood in the system.
	pub wood_acc_cap: i64,
	/// Represents the cap on accumulated stone in the system.
	pub stone_acc_cap: i64,
	/// Represents the cap on accumulated gold in the system.
	pub gold_acc_cap: i64,
}
