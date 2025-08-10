use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::modifier::active_modifier::{
	ActiveModifier, ActiveModifierKey, ModifierSourceType,
};
use crate::domain::modifier::{
	MagnitudeKind, Modifier, ModifierKey, ModifierTarget, StackingBehaviour,
};
use crate::domain::player;
use crate::domain::player::resource::ResourceType;

/// Represents a complete modifier instance that can be applied to player resources or attributes.
/// This struct combines both the modifier definition and its active state information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FullModifier {
	/// Unique identifier for the active modifier instance
	pub id: ActiveModifierKey,
	/// Reference to the base modifier definition
	pub modifier_id: ModifierKey,
	/// ID of the player this modifier is applied to
	pub player_id: player::PlayerKey,

	/// Display name of the modifier
	pub name: String,
	/// Detailed description of the modifier's effect
	pub description: String,
	/// Numerical value of the modifier's effect
	pub magnitude: BigDecimal,
	/// Type of modification (e.g., additive, multiplicative)
	pub magnitude_kind: MagnitudeKind,
	/// Origin of the modifier (faction, item, skill, etc.)
	pub source_type: ModifierSourceType,
	/// Optional ID referencing the specific source entity
	pub source_id: Option<Uuid>,
	/// What the modifier affects (resource, attribute, etc.)
	pub target_type: ModifierTarget,
	/// Specific resource type this modifier affects, if applicable
	pub target_resource: Option<ResourceType>,
	/// How this modifier combines with others of the same type
	pub stacking_behaviour: StackingBehaviour,
	/// Optional group identifier for stacking rules
	pub stacking_group: Option<String>,

	/// When the modifier became active
	pub started_at: DateTime<Utc>,
	/// When the modifier will expire, if temporary
	pub expires_at: Option<DateTime<Utc>>,
	/// When the active modifier was created in the system
	pub created_at: DateTime<Utc>,
	/// When the active modifier was last updated
	pub updated_at: DateTime<Utc>,
}

impl FullModifier {
	/// Extract stacking group from modifier source and target
	/// Example: "faction_resource", "temporary_combat", etc.
	pub fn get_stacking_group(&self) -> String {
		format!(
			"{}_{}",
			self.source_type.to_string().to_lowercase(),
			self.target_type.to_string().to_lowercase()
		)
	}
}

impl Modifier {
	pub fn into_full(self, active: ActiveModifier) -> FullModifier {
		FullModifier {
			id: active.id,
			modifier_id: self.id,
			player_id: active.player_id,
			name: self.name,
			description: self.description,
			magnitude: self.magnitude,
			magnitude_kind: self.magnitude_kind,
			source_type: active.source_type,
			source_id: active.source_id,
			target_type: self.target_type,
			target_resource: self.target_resource,
			stacking_behaviour: self.stacking_behaviour,
			stacking_group: self.stacking_group,
			started_at: active.started_at,
			expires_at: active.expires_at,
			created_at: active.created_at,
			updated_at: active.updated_at,
		}
	}
}
