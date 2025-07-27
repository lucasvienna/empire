use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension, Json};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use cookie::{time, Cookie, SameSite};
use serde_json::json;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::auth::session_service::SessionService;
use crate::configuration::Settings;
use crate::controllers::auth::models::{
    LoginPayload, PlayerDto, PlayerDtoResponse, RegisterPayload, SessionDto,
};
use crate::db::players::PlayerRepository;
use crate::db::Repository;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::auth::{AuthError, AuthenticatedUser};
use crate::domain::player::session::PlayerSession;
use crate::domain::player::NewPlayer;
use crate::net::{SessionToken, SESSION_COOKIE_NAME, TOKEN_COOKIE_NAME};

#[instrument(skip(pool, payload), fields(username = %payload.username))]
#[debug_handler(state = AppState)]
pub(super) async fn register(
    State(pool): State<AppPool>,
    Json(payload): Json<RegisterPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    trace!("Starting player registration process");
    let repo = PlayerRepository::new(&pool);
    let new_user = NewPlayer::try_from(payload).map_err(|err| {
        error!("Failed to parse player during registration: {}", err);
        let body = json!({ "status": "error", "message": err.to_string() });
        (StatusCode::BAD_REQUEST, Json(body))
    })?;

    debug!("Player data parsed successfully");

    match repo.exists_by_name(&new_user.name) {
        Ok(exists) => {
            if exists {
                warn!(
                    "Registration attempted with existing username: {}",
                    new_user.name
                );
                let body = json!({ "status": "error", "message": "Username already taken" });
                return Err((StatusCode::CONFLICT, Json(body)));
            }
            debug!("Username {} is available for registration", new_user.name);
        }
        Err(err) => {
            error!("Failed to check if player exists: {}", err);
            let body = json!({ "status": "error", "message": "Please try again later" });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(body)));
        }
    }

    let created_user = repo.create(new_user).map_err(|err| {
        error!("Failed to insert player: {:#?}", err);
        let body = json!({ "status": "error", "message": err.to_string() });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
    })?;
    info!(
        player_id = created_user.id.to_string(),
        "Created player successfully"
    );

    let body = json!({ "status": "success", "message": "Player registered successfully" });
    info!(
        player_id = created_user.id.to_string(),
        "Player registration completed successfully"
    );

    Ok((StatusCode::CREATED, Json(body)))
}

#[instrument(skip(pool, session_service, jar, settings, payload), fields(username = %payload.username))]
#[debug_handler(state = AppState)]
pub(super) async fn login(
    State(pool): State<AppPool>,
    State(session_service): State<SessionService>,
    jar: CookieJar,
    settings: Settings,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, AuthError> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    trace!("Beginning authentication for user: {}", payload.username);
    let repo = PlayerRepository::new(&pool);
    let user = repo.get_by_name(&payload.username).map_err(|err| {
        warn!("User login failed - player not found: {}", err);
        AuthError::WrongCredentials
    })?;

    debug!("Found player record for login attempt");

    trace!("Verifying password for player {}", user.name);
    let argon2 = Argon2::default();
    let hash = PasswordHash::new(&user.pwd_hash).map_err(|e| {
        error!(
            "Failed to parse password hash for player {}: {:?}",
            user.name, e
        );
        AuthError::ArgonError
    })?;

    if argon2
        .verify_password(payload.password.as_ref(), &hash)
        .is_err()
    {
        warn!(
            player_id = %user.id,
            "Authentication failed - invalid password for player: {}",
            user.name
        );
        return Err(AuthError::WrongCredentials);
    }

    debug!("Password verified successfully for player {}", user.name);

    trace!("Generating session token for player {}", user.id);
    let session_token = session_service.generate_session_token();

    debug!("Creating session for player {}", user.id);
    let session = session_service
        .create_session(session_token.clone(), &user.id)
        .map_err(|e| {
            error!("Failed to create session for player {}: {:?}", user.id, e);
            AuthError::TokenCreation
        })?;

    info!(
        player_id = %user.id,
        session_id = %session.id,
        expires_at = %session.expires_at,
        "Player successfully logged in"
    );

    let max_age = session.expires_at - Utc::now();
    let cookie = Cookie::build((SESSION_COOKIE_NAME, session_token))
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(time::Duration::seconds(max_age.num_seconds()))
        .build();

    Ok(jar.add(cookie))
}

#[instrument(skip(jar, srv, maybe_session))]
#[debug_handler(state = AppState)]
pub(super) async fn logout(
    State(srv): State<SessionService>,
    _player: Extension<AuthenticatedUser>,
    maybe_session: Option<Extension<PlayerSession>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AuthError> {
    if let Some(session) = maybe_session {
        debug!("Processing logout request for session {}", session.id);

        let jar = jar
            .remove(Cookie::from(SESSION_COOKIE_NAME))
            .remove(Cookie::from(TOKEN_COOKIE_NAME));

        trace!("Invalidating session {}", session.id);
        srv.invalidate_session(&session.id);
        info!(
            "Player logged out successfully, session {} invalidated",
            session.id
        );

        let body = json!({ "status": "ok" });
        Ok((jar, Json(body)))
    } else {
        // User is authenticated via JWT, which doesn't have a server-side session to invalidate
        let jar = jar.remove(Cookie::from(TOKEN_COOKIE_NAME));
        let body = json!({ "status": "ok", "message": "JWT token removed" });
        Ok((jar, Json(body)))
    }
}

#[instrument(skip(jar, srv, maybe_token), fields(player_id = %player.id))]
#[debug_handler(state = AppState)]
pub(super) async fn session(
    State(srv): State<SessionService>,
    player: Extension<AuthenticatedUser>,
    maybe_session: Option<Extension<PlayerSession>>,
    maybe_token: Option<Extension<SessionToken>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AuthError> {
    if let (Some(session), Some(token)) = (maybe_session, maybe_token) {
        trace!("Preparing session info response for player {}", player.id);
        let session_dto = SessionDto {
            token: token.to_string(),
            expires_at: session.expires_at,
        };
        let player_dto = PlayerDto {
            id: player.id,
            name: player.name.clone(),
            email: player.email.clone(),
            faction: player.faction.to_string(),
        };

        debug!(
            "Session info retrieved successfully for player {}",
            player.id
        );
        Ok((
            jar,
            Json(PlayerDtoResponse {
                session: session_dto,
                player: player_dto,
            }),
        ))
    } else {
        // User is authenticated via JWT, which is stateless
        Err(AuthError::MismatchedModality)
    }
}
