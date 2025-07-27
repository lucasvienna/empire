use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension, Json};
use tracing::instrument;

use crate::controllers::player::JoinFactionPayload;
use crate::controllers::user::UserBody;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::auth::AuthenticatedUser;
use crate::game::player_service::PlayerService;

#[instrument(skip(pool))]
#[debug_handler(state = AppState)]
pub(super) async fn get_player_profile(State(pool): State<AppPool>) -> impl IntoResponse {
    // Implementation placeholder
    StatusCode::NOT_IMPLEMENTED
}

#[instrument(skip(srv, player), fields(player_id = %player.id))]
#[debug_handler(state = AppState)]
pub async fn join_faction(
    State(srv): State<PlayerService>,
    player: Extension<AuthenticatedUser>,
    Json(payload): Json<JoinFactionPayload>,
) -> crate::Result<impl IntoResponse, StatusCode> {
    let player_key = player.id;
    let user = srv.update_user(player_key, payload.into());
    match user {
        Ok(usr) => {
            let body: UserBody = usr.into();
            Ok((StatusCode::ACCEPTED, Json(body)))
        }
        Err(err) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
