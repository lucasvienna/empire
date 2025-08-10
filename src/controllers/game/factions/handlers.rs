use std::collections::HashMap;
use std::str::FromStr;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use tracing::{debug, info, instrument};

use crate::controllers::game::factions::models::{FactionDetails, FactionResponse};
use crate::controllers::game::factions::FactionBonus;
use crate::db::factions::FactionRepository;
use crate::db::Repository;
use crate::domain::app_state::AppState;
use crate::domain::factions::FactionKey;
use crate::Result;

/// GET /game/factions
/// List all available factions with their bonuses
#[instrument(skip(repo))]
#[debug_handler(state = AppState)]
pub(super) async fn get_factions(State(repo): State<FactionRepository>) -> impl IntoResponse {
	debug!("Getting all available factions");
	let map: HashMap<FactionKey, Vec<FactionBonus>> = HashMap::new();
	let faction_bonuses = repo
		.get_bonuses(None)
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
	let factions: Vec<FactionResponse> = repo
		.get_all()
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

/// GET /game/factions/{faction_id}  
/// Get detailed information about specific faction
#[instrument(skip(repo))]
#[debug_handler(state = AppState)]
pub(super) async fn get_faction(
	Path(faction_id): Path<FactionKey>,
	State(repo): State<FactionRepository>,
) -> Result<impl IntoResponse, StatusCode> {
	debug!("Getting faction details");
	let mut faction = repo
		.get_by_id(&faction_id)
		.map(FactionDetails::from)
		.map_err(|_| StatusCode::NOT_FOUND)?;
	let mut bonuses = repo
		.get_bonuses(Some(&faction_id))
		.map(|mods| mods.into_iter().map(FactionBonus::from).collect())
		.unwrap_or_default();
	faction.bonuses.append(&mut bonuses);
	info!("Retrieved faction details");
	Ok(Json(faction))
}
