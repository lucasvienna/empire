use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension, Json};
use tracing::{debug, error, info, instrument};

use crate::controllers::player::{JoinFactionPayload, PlayerProfileResponse};
use crate::controllers::user::{UpdateUserPayload, UserBody};
use crate::db::extractor::DatabaseConnection;
use crate::domain::app_state::AppState;
use crate::domain::auth::AuthenticatedUser;
use crate::game::player_operations;
use crate::game::resources::resource_scheduler::ProductionScheduler;

#[instrument(skip_all, fields(player_id = %player.id))]
#[debug_handler(state = AppState)]
pub(super) async fn get_player_profile(
	DatabaseConnection(mut conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
) -> crate::Result<impl IntoResponse, StatusCode> {
	debug!("Starting player profile retrieval");
	let profile = player_operations::get_player(&mut conn, &player.id)
		.map(PlayerProfileResponse::from)
		.map_err(|err| {
			error!("Failed to fetch user profile");
			StatusCode::INTERNAL_SERVER_ERROR
		})?;
	info!(?profile, "Fetched user profile");
	Ok(Json(profile))
}

#[instrument(skip(conn, scheduler, player), fields(player_id = %player.id))]
#[debug_handler(state = AppState)]
pub(super) async fn update_player_profile(
	DatabaseConnection(mut conn): DatabaseConnection,
	State(scheduler): State<ProductionScheduler>,
	player: Extension<AuthenticatedUser>,
	Json(payload): Json<UpdateUserPayload>,
) -> crate::Result<impl IntoResponse, StatusCode> {
	debug!("Starting player profile update");
	let profile = player_operations::update_player(&mut conn, &scheduler, player.id, payload)
		.map(PlayerProfileResponse::from)
		.map_err(|_| {
			error!("Failed to update user profile");
			StatusCode::INTERNAL_SERVER_ERROR
		})?;
	info!(?profile, "Updated user profile");
	Ok((StatusCode::ACCEPTED, Json(profile)))
}

#[instrument(skip(conn, scheduler, player), fields(player_id = %player.id))]
#[debug_handler(state = AppState)]
pub(super) async fn join_faction(
	DatabaseConnection(mut conn): DatabaseConnection,
	State(scheduler): State<ProductionScheduler>,
	player: Extension<AuthenticatedUser>,
	Json(payload): Json<JoinFactionPayload>,
) -> crate::Result<impl IntoResponse, StatusCode> {
	debug!("Starting player faction join");
	let body = player_operations::update_player(&mut conn, &scheduler, player.id, payload.into())
		.map(UserBody::from)
		.map_err(|_| {
			error!("Failed to join faction");
			StatusCode::INTERNAL_SERVER_ERROR
		})?;
	info!(faction = %body.faction, "Joined faction successfully");
	Ok((StatusCode::ACCEPTED, Json(body)))
}
