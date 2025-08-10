//! Represents the current state and status of modifiers in the system.
//! This module provides structures for tracking modifier states and their lifecycle.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::modifier::active_modifier::ModifierSourceType;

/// Represents the current state of a modifier, including its magnitude and status.
pub struct ModifierState {
	/// The numerical value/strength of the modifier
	pub magnitude: BigDecimal,
	/// The type of entity that created or owns this modifier
	pub source_type: ModifierSourceType,
	/// Optional identifier of the specific source entity
	pub source_id: Option<Uuid>,
	/// Current status of the modifier (active, expired, or removed)
	pub status: ModifierStatus,
	/// Timestamp of the last update to this modifier state
	pub last_updated: DateTime<Utc>,
}

/// Represents the lifecycle status of a modifier
pub enum ModifierStatus {
	/// Modifier is currently active and applying its effects
	Active,
	/// Modifier has reached its end condition and no longer applies
	Expired,
	/// Modifier was manually removed or cancelled
	Removed,
}
