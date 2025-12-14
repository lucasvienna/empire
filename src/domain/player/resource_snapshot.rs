use chrono::{DateTime, Utc};

/// Aggregated snapshot of a player's current resource state.
///
/// This type combines data from multiple sources:
/// - `player_resource` table (current storage amounts and caps)
/// - `player_accumulator` table (accumulated resources awaiting collection)
/// - `resource_generation` view (production rates and accumulator caps)
///
/// Used to provide a complete picture of a player's resources at a point in time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerResourceSnapshot {
	// Current storage amounts
	pub food: i64,
	pub wood: i64,
	pub stone: i64,
	pub gold: i64,

	// Storage caps
	pub food_cap: i64,
	pub wood_cap: i64,
	pub stone_cap: i64,
	pub gold_cap: i64,

	// Production rates (per hour)
	pub food_rate: i64,
	pub wood_rate: i64,
	pub stone_rate: i64,
	pub gold_rate: i64,

	// Accumulator amounts (resources awaiting collection)
	pub food_acc: i64,
	pub wood_acc: i64,
	pub stone_acc: i64,
	pub gold_acc: i64,

	// Accumulator caps
	pub food_acc_cap: i64,
	pub wood_acc_cap: i64,
	pub stone_acc_cap: i64,
	pub gold_acc_cap: i64,

	// Timestamps
	pub produced_at: DateTime<Utc>,
	pub collected_at: DateTime<Utc>,
}
