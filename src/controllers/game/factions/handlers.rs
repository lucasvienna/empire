use std::collections::HashMap;
use std::str::FromStr;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use tracing::{debug, info, instrument};

use crate::controllers::game::factions::models::{FactionDetails, FactionResponse};
use crate::controllers::game::factions::FactionBonus;
use crate::db::extractor::DatabaseConnection;
use crate::db::factions;
use crate::domain::app_state::AppState;
use crate::domain::factions::FactionKey;
use crate::Result;

/// GET `/game/factions`
/// List all available factions with their bonuses
#[instrument(skip(conn))]
#[debug_handler(state = AppState)]
pub(super) async fn get_factions(
	DatabaseConnection(mut conn): DatabaseConnection,
) -> impl IntoResponse {
	debug!("Getting all available factions");
	let map: HashMap<FactionKey, Vec<FactionBonus>> = HashMap::new();
	let faction_bonuses = factions::get_bonuses(&mut conn, None)
		.unwrap_or_default()
		.into_iter()
		.filter_map(|fb| {
			let fac_key = fb.name.split('_').next().unwrap_or_default();
			if fac_key.is_empty() {
				return None;
			}
			// remove whatever isn't a faction we know and ignore
			let fac_key = FactionKey::from_str(fac_key).unwrap_or_default();
			if fac_key == FactionKey::Neutral {
				return None;
			}
			Some((fac_key, FactionBonus::from(fb)))
		})
		.fold(
			map,
			|mut acc: HashMap<FactionKey, Vec<FactionBonus>>, (key, val)| {
				if let Some(arr) = acc.get_mut(&key) {
					arr.push(val);
				} else {
					acc.insert(key, vec![val]);
				}
				acc
			},
		);
	let factions: Vec<FactionResponse> = factions::get_all(&mut conn)
		.unwrap_or_default()
		.into_iter()
		.map(|val| {
			let mut res = FactionResponse::from(val);
			res.bonuses = faction_bonuses.get(&res.id).cloned().unwrap_or_default();
			res
		})
		.collect();
	info!("Retrieved factions list with {} items", factions.len());
	Json(factions)
}

/// GET `/game/factions/{faction_id}`
/// Get detailed information about a specific faction
#[instrument(skip(conn))]
#[debug_handler(state = AppState)]
pub(super) async fn get_faction(
	Path(faction_id): Path<FactionKey>,
	DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<impl IntoResponse, StatusCode> {
	debug!("Getting faction details");
	let mut faction = factions::get_by_id(&mut conn, &faction_id)
		.map(FactionDetails::from)
		.map_err(|_| StatusCode::NOT_FOUND)?;
	let mut bonuses = factions::get_bonuses(&mut conn, Some(&faction_id))
		.map(|mods| mods.into_iter().map(FactionBonus::from).collect())
		.unwrap_or_default();
	faction.bonuses.append(&mut bonuses);
	info!("Retrieved faction details");
	Ok(Json(faction))
}
