use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension, Json};
use tracing::{debug, error, info, instrument};

use crate::controllers::player::{JoinFactionPayload, PlayerProfileResponse};
use crate::controllers::user::{UpdateUserPayload, UserBody};
use crate::domain::app_state::AppState;
use crate::domain::auth::AuthenticatedUser;
use crate::game::player_service::PlayerService;

#[instrument(skip(srv, player), fields(player_id = %player.id))]
#[debug_handler(state = AppState)]
pub(super) async fn get_player_profile(
    State(srv): State<PlayerService>,
    player: Extension<AuthenticatedUser>,
) -> crate::Result<impl IntoResponse, StatusCode> {
    debug!("Starting player profile retrieval");
    let profile = srv
        .get_player(&player.id)
        .map(PlayerProfileResponse::from)
        .map_err(|err| {
            error!("Failed to fetch user profile");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    info!(?profile, "Fetched user profile");
    Ok(Json(profile))
}

#[instrument(skip(srv, player), fields(player_id = %player.id))]
#[debug_handler(state = AppState)]
pub(super) async fn update_player_profile(
    State(srv): State<PlayerService>,
    player: Extension<AuthenticatedUser>,
    Json(payload): Json<UpdateUserPayload>,
) -> crate::Result<impl IntoResponse, StatusCode> {
    debug!("Starting player profile update");
    let profile = srv
        .update_player(player.id, payload)
        .map(PlayerProfileResponse::from)
        .map_err(|_| {
            error!("Failed to update user profile");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    info!(?profile, "Updated user profile");
    Ok((StatusCode::ACCEPTED, Json(profile)))
}

#[instrument(skip(srv, player), fields(player_id = %player.id))]
#[debug_handler(state = AppState)]
pub(super) async fn join_faction(
    State(srv): State<PlayerService>,
    player: Extension<AuthenticatedUser>,
    Json(payload): Json<JoinFactionPayload>,
) -> crate::Result<impl IntoResponse, StatusCode> {
    debug!("Starting player faction join");
    let body = srv
        .update_player(player.id, payload.into())
        .map(UserBody::from)
        .map_err(|_| {
            error!("Failed to join faction");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    info!(faction = %body.faction, "Joined faction successfully");
    Ok((StatusCode::ACCEPTED, Json(body)))
}
