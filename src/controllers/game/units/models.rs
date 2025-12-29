//! Request and response DTOs for the units API endpoints.
//!
//! These models handle the serialization/deserialization of unit training data
//! between the API layer and clients.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::player::buildings::PlayerBuildingKey;
use crate::domain::unit::training::{TrainingQueueKey, TrainingStatus};
use crate::domain::unit::{UnitKey, UnitType};

// === Request DTOs ===

/// Query parameters for GET /units/available
#[derive(Deserialize, Debug)]
pub struct AvailableUnitsQuery {
	pub building_id: PlayerBuildingKey,
}

/// Request body for POST /units/train
#[derive(Deserialize, Debug)]
pub struct TrainUnitsRequest {
	pub building_id: PlayerBuildingKey,
	pub unit_id: UnitKey,
	pub quantity: i64,
}

// === Response DTOs ===

/// Resource cost breakdown for a unit.
/// Represents the cost to train a single unit.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UnitCostDto {
	pub food: i64,
	pub wood: i64,
	pub stone: i64,
	pub gold: i64,
}

impl UnitCostDto {
	/// Creates a UnitCostDto from a (food, wood, stone, gold) tuple.
	pub fn from_tuple(costs: (i64, i64, i64, i64)) -> Self {
		Self {
			food: costs.0,
			wood: costs.1,
			stone: costs.2,
			gold: costs.3,
		}
	}
}

/// A single available unit with all UI-relevant data.
/// Used in the available units response to show what can be trained.
#[derive(Serialize, Debug)]
pub struct AvailableUnitDto {
	pub id: UnitKey,
	pub name: String,
	pub unit_type: UnitType,
	pub base_atk: i64,
	pub base_def: i64,
	pub description: Option<String>,
	/// Base training time per unit in seconds (before faction modifiers)
	pub base_training_seconds: i32,
	/// Modified training time per unit in seconds (after faction modifiers)
	pub modified_training_seconds: i32,
	/// Training speed modifier value (< 1.0 = faster, > 1.0 = slower)
	/// AIDEV-NOTE: Modifier < 1.0 means faster training (e.g., Goblin 0.8 = 20% faster)
	pub training_modifier: f64,
	/// Resource cost per single unit
	pub cost: UnitCostDto,
	/// Whether the player can afford to train at least 1 unit
	pub can_afford: bool,
	/// Maximum quantity the player can afford based on current resources
	pub max_affordable: i64,
}

/// Response for GET /units/available
#[derive(Serialize, Debug)]
pub struct AvailableUnitsResponse {
	pub building_id: PlayerBuildingKey,
	pub units: Vec<AvailableUnitDto>,
	/// Number of training slots currently in use at this building
	pub training_slots: i64,
	/// Maximum training slots available at this building
	pub max_training_slots: i64,
	/// Free training slots (max - used)
	pub free_training_slots: i64,
}

/// Response for POST /units/train
#[derive(Serialize, Debug)]
pub struct TrainUnitsResponse {
	pub training_id: TrainingQueueKey,
	pub unit_id: UnitKey,
	pub unit_name: String,
	pub quantity: i64,
	pub started_at: DateTime<Utc>,
	/// Estimated completion time (ISO 8601 format for client rendering)
	pub completion_time: DateTime<Utc>,
	/// Total training duration in seconds
	pub total_training_seconds: i64,
	/// Total resources spent for this training batch
	pub resources_spent: UnitCostDto,
}

/// A single training queue entry with progress information.
/// Includes all data needed for client-side progress bar rendering.
#[derive(Serialize, Debug)]
pub struct TrainingQueueEntryDto {
	pub id: TrainingQueueKey,
	pub building_id: PlayerBuildingKey,
	pub unit_id: UnitKey,
	pub unit_name: String,
	pub unit_type: UnitType,
	pub quantity: i64,
	pub started_at: DateTime<Utc>,
	pub status: TrainingStatus,
	/// Estimated completion time (ISO 8601 format)
	pub estimated_completion: DateTime<Utc>,
	/// Progress percentage (0.0 - 100.0)
	/// AIDEV-NOTE: Progress uses faction-modified duration, not base duration
	pub progress_percent: f64,
	/// Seconds remaining until completion
	pub seconds_remaining: i64,
}

/// Response for GET /units/queue
#[derive(Serialize, Debug)]
pub struct TrainingQueueResponse {
	pub entries: Vec<TrainingQueueEntryDto>,
	pub total_entries: usize,
}

/// A single player unit in the inventory.
#[derive(Serialize, Debug)]
pub struct PlayerUnitDto {
	pub unit_id: UnitKey,
	pub unit_name: String,
	pub unit_type: UnitType,
	pub quantity: i64,
}

/// Response for GET /units/inventory
#[derive(Serialize, Debug)]
pub struct PlayerUnitsResponse {
	pub units: Vec<PlayerUnitDto>,
	/// Total count of all units owned by the player
	pub total_units: i64,
}

/// Response for DELETE /units/queue/{id}
#[derive(Serialize, Debug)]
pub struct CancelTrainingResponse {
	pub training_id: TrainingQueueKey,
	pub status: TrainingStatus,
	/// Resources refunded to the player
	/// AIDEV-NOTE: Refund is 80% * remaining_ratio, where remaining_ratio = (1 - elapsed/total)
	pub refunded: UnitCostDto,
}
