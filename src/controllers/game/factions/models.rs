//! Faction system models and data structures.
//!
//! This module defines the core data structures for the faction system in Empire,
//! including faction bonuses, response models, and detailed faction information.
//! Factions provide strategic advantages through various bonuses and add depth
//! to the gameplay through lore and thematic elements.

use std::default::Default;

use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};

use crate::domain::factions::{Faction, FactionKey};
use crate::domain::modifier::{MagnitudeKind, Modifier, ModifierTarget};
use crate::domain::player::resource::ResourceType;

/// A specific bonus or advantage provided by a faction.
///
/// Faction bonuses modify game mechanics to provide strategic advantages
/// to players who belong to that faction. Each bonus has a type that
/// determines how it affects gameplay, a target that specifies what
/// it modifies, and a value that quantifies the effect.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FactionBonus {
    /// The target of the bonus (e.g., "resource", "combat", "research")
    pub target: ModifierTarget,
    /// Specific resource type this bonus affects, if applicable
    pub target_resource: Option<ResourceType>,
    /// The numerical value of the bonus (percentage, flat amount, etc.)
    pub value: f64,
    /// The type of bonus application (e.g., multiplicative, flat, or percentual)
    pub scaling: MagnitudeKind,
    /// Human-readable description of what this bonus does
    pub description: String,
}

impl From<Modifier> for FactionBonus {
    fn from(modifier: Modifier) -> Self {
        Self {
            target: modifier.target_type,
            target_resource: modifier.target_resource,
            value: modifier.magnitude.to_f64().unwrap_or_default(),
            scaling: modifier.magnitude_kind,
            description: modifier.description,
        }
    }
}

/// Basic faction information for API responses.
///
/// This struct is used when returning faction data in API responses
/// where only essential information is needed. It includes the faction's
/// core identity and bonuses but excludes detailed lore and descriptions
/// to keep response payloads smaller.
///
/// Typically used in:
/// - Faction listing endpoints
/// - Player faction assignment responses
/// - Quick faction overview displays
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FactionResponse {
    /// Unique identifier for the faction
    pub id: FactionKey,
    /// Display name of the faction
    pub name: String,
    /// List of bonuses this faction provides
    pub bonuses: Vec<FactionBonus>,
}

impl From<Faction> for FactionResponse {
    fn from(faction: Faction) -> Self {
        Self {
            id: faction.id,
            name: faction.name,
            bonuses: Default::default(), // has to be enriched after
        }
    }
}

/// Complete faction information including lore and detailed descriptions.
///
/// This struct contains all available information about a faction,
/// including narrative elements like lore and extended descriptions.
/// It's used when clients need comprehensive faction data, such as
/// for faction selection screens or detailed information displays.
///
/// The additional fields compared to `FactionResponse` provide context
/// and immersion for players making faction choices.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FactionDetails {
    /// Unique identifier for the faction
    pub id: String,
    /// Display name of the faction
    pub name: String,
    /// Detailed description of the faction's characteristics and playstyle
    pub description: String,
    /// Background story and narrative context for the faction
    pub lore: String,
    /// List of bonuses this faction provides
    pub bonuses: Vec<FactionBonus>,
}

impl From<Faction> for FactionDetails {
    fn from(faction: Faction) -> Self {
        Self {
            id: faction.id.to_string(),
            name: faction.name,
            description: String::default(),
            lore: String::default(),
            bonuses: Vec::default(),
        }
    }
}
