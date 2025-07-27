use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension, Json};
use tracing::instrument;

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
    let player_ = srv
        .get_player(&player.id)
        .map(PlayerProfileResponse::from)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(player_))
}

#[instrument(skip(srv, player), fields(player_id = %player.id))]
#[debug_handler(state = AppState)]
pub(super) async fn update_player_profile(
    State(srv): State<PlayerService>,
    player: Extension<AuthenticatedUser>,
    Json(payload): Json<UpdateUserPayload>,
) -> crate::Result<impl IntoResponse, StatusCode> {
    let update = srv
        .update_player(player.id, payload)
        .map(PlayerProfileResponse::from)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::ACCEPTED, Json(update)))
}

#[instrument(skip(srv, player), fields(player_id = %player.id))]
#[debug_handler(state = AppState)]
pub(super) async fn join_faction(
    State(srv): State<PlayerService>,
    player: Extension<AuthenticatedUser>,
    Json(payload): Json<JoinFactionPayload>,
) -> crate::Result<impl IntoResponse, StatusCode> {
    let body = srv
        .update_player(player.id, payload.into())
        .map(UserBody::from)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::ACCEPTED, Json(body)))
}
