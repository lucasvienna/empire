//! Modifier operations for the Empire game.
//!
//! This module provides pure, stateless functions for modifier calculations and retrieval.
//! It follows the functional programming approach with direct function calls rather than
//! service structs, enabling better performance through single-connection-per-request optimization.
//!
//! ## Architecture
//!
//! This module is part of the operations layer, which provides:
//! - **Pure functions** - No side effects, deterministic outputs
//! - **No caching** - Always calculates fresh values from the database
//! - **Synchronous** - Direct database queries without async overhead
//! - **Lightweight** - Only requires a database connection
//!
//! ## When to Use
//!
//! Use these operations in **handlers** and interactive endpoints where:
//! - Fresh values are required (no cache staleness)
//! - Single request optimization is important
//! - You want to avoid instantiating service structs
//!
//! For **background jobs** that need caching and async processing, use
//! [`ModifierService`](crate::game::modifiers::modifier_service::ModifierService) instead.
//!
//! ## Modifier Stacking Behavior
//!
//! The module implements three stacking behaviors for modifiers:
//!
//! ### Additive
//! All modifiers of this type are summed together, then added to the base value (1.0).
//! ```text
//! Result = (1.0 + sum_of_magnitudes) * base_value
//! Example: +10% and +20% → 1.3x multiplier
//! ```
//!
//! ### Multiplicative
//! Each modifier multiplies independently with the base value.
//! ```text
//! Result = (1.0 + mag1) * (1.0 + mag2) * ... * base_value
//! Example: +10% and +20% → 1.32x multiplier
//! ```
//!
//! ### HighestOnly
//! Only the highest magnitude modifier in each stacking group applies.
//! ```text
//! Example: +10%, +20%, +15% in same group → only +20% applies
//! ```
//!
//! Final results are capped between 0.5 (50%) and 3.0 (300%).

use std::collections::HashMap;

use bigdecimal::BigDecimal;
use diesel::prelude::*;

use crate::db::DbConn;
use crate::domain::modifier::active_modifier::ActiveModifier;
use crate::domain::modifier::full_modifier::FullModifier;
use crate::domain::modifier::{Modifier, ModifierTarget, StackingBehaviour};
use crate::domain::player::resource::ResourceType;
use crate::domain::player::PlayerKey;
use crate::Result;

/// Get all active modifiers for a player
pub fn get_active_mods(conn: &mut DbConn, player_key: &PlayerKey) -> Result<Vec<ActiveModifier>> {
	use crate::schema::active_modifiers::dsl as am;

	am::active_modifiers
		.filter(am::player_id.eq(player_key))
		.select(ActiveModifier::as_select())
		.load(conn)
		.map_err(Into::into)
}

/// Get all active modifiers with their full modifier details for a player
pub fn get_full_mods(conn: &mut DbConn, player_key: &PlayerKey) -> Result<Vec<FullModifier>> {
	use crate::schema::active_modifiers::dsl as am;
	use crate::schema::modifiers::dsl as m;

	let mods = am::active_modifiers
		.inner_join(m::modifiers)
		.filter(am::player_id.eq(player_key))
		.select((ActiveModifier::as_select(), Modifier::as_select()))
		.load::<(ActiveModifier, Modifier)>(conn)?;

	Ok(mods.into_iter().map(|(am, m)| m.into_full(am)).collect())
}

/// Calculate the total modifier multiplier for a specific target and resource
///
/// This is a pure function that doesn't use caching - suitable for handlers.
/// For background jobs that need caching, use ModifierService::get_total_multiplier()
pub fn calc_multiplier(
	conn: &mut DbConn,
	player_id: &PlayerKey,
	target_type: ModifierTarget,
	target_resource: Option<ResourceType>,
) -> Result<BigDecimal> {
	let player_mods = get_full_mods(conn, player_id)?;
	let modifiers: Vec<FullModifier> = player_mods
		.into_iter()
		.filter(|m| m.target_type == target_type && m.target_resource == target_resource)
		.collect();

	Ok(apply_stacking_rules(&modifiers))
}

/// Calculate the final modifier value for a collection of modifiers
///
/// This implements the stacking behavior logic:
/// - Additive: Sum all magnitudes, then add to base (1.0)
/// - Multiplicative: Multiply (1.0 + magnitude) for each modifier
/// - HighestOnly: Take the highest magnitude per stacking group
///
/// The result is capped between 0.5 (50%) and 3.0 (300%)
fn apply_stacking_rules(modifiers: &[FullModifier]) -> BigDecimal {
	let global_max_cap: BigDecimal = BigDecimal::from(3); // 300%
	let global_min_floor: BigDecimal =
		BigDecimal::try_from(0.5).expect("Failed to create a 0.5 numeric."); // 50%
	let base = BigDecimal::from(1);

	if modifiers.is_empty() {
		return base;
	}

	// Step 1: Group modifiers by their stacking behavior
	let mut additive_mods: Vec<&FullModifier> = Vec::new();
	let mut multiplicative_mods: Vec<&FullModifier> = Vec::new();
	let mut highest_only_groups: HashMap<String, Vec<&FullModifier>> = HashMap::new();

	for modifier in modifiers {
		match modifier.stacking_behaviour {
			StackingBehaviour::Additive => additive_mods.push(modifier),
			StackingBehaviour::Multiplicative => multiplicative_mods.push(modifier),
			StackingBehaviour::HighestOnly => {
				let group = modifier
					.stacking_group
					.clone()
					.unwrap_or_else(|| modifier.get_stacking_group());
				highest_only_groups.entry(group).or_default().push(modifier);
			}
		}
	}

	// Step 2: Calculate additive modifiers
	let additive_total = additive_mods
		.iter()
		.fold(BigDecimal::from(0), |acc, m| acc + &m.magnitude);

	// Step 3: Calculate highest-only modifiers
	let highest_only_values: Vec<BigDecimal> = highest_only_groups
		.values()
		.map(|group| {
			group
				.iter()
				.map(|m| &m.magnitude)
				.max_by(|a, b| a.cmp(b))
				.unwrap_or(&BigDecimal::from(0))
				.clone()
		})
		.collect();

	// Step 4: Calculate multiplicative effect
	let multiplicative_total = multiplicative_mods
		.iter()
		.map(|m| &m.magnitude)
		.chain(highest_only_values.iter())
		.fold(base.clone(), |acc, magnitude| {
			acc * (base.clone() + magnitude)
		});

	// Step 5: Combine all effects
	let total = (base + additive_total) * multiplicative_total;

	// Step 6: Apply global caps and floors using clamp
	total.clamp(global_min_floor, global_max_cap)
}
