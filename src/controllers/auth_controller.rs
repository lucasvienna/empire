use std::fmt::Debug;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{debug_handler, Extension, Json, Router};
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use cookie::{time, Cookie, SameSite};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, instrument, warn};

use crate::auth::session_service::SessionService;
use crate::auth::utils::hash_password;
use crate::configuration::Settings;
use crate::db::players::PlayerRepository;
use crate::db::Repository;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::auth::AuthError;
use crate::domain::factions::FactionCode;
use crate::domain::player;
use crate::domain::player::session::PlayerSession;
use crate::domain::player::{NewPlayer, Player, PlayerKey};
use crate::net::{SessionToken, SESSION_COOKIE_NAME, TOKEN_COOKIE_NAME};
use crate::ErrorKind;

#[derive(Serialize, Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

impl Debug for RegisterPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterPayload")
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .field("email", &self.email)
            .finish()
    }
}

impl TryFrom<RegisterPayload> for NewPlayer {
    type Error = crate::Error;

    fn try_from(value: RegisterPayload) -> Result<Self, Self::Error> {
        let name = player::UserName::parse(value.username)?;
        let email: Option<player::UserEmail> = match value.email {
            None => None,
            Some(email) => Some(player::UserEmail::parse(email)?),
        };
        let pwd_hash = hash_password(&value.password)
            .map_err(|_| (ErrorKind::InternalError, "Failed to hash password"))?;
        Ok(Self {
            name,
            pwd_hash,
            email,
            faction: FactionCode::Neutral,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

impl Debug for LoginPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoginPayload")
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

#[instrument(skip(pool))]
#[debug_handler(state = AppState)]
async fn register(
    State(pool): State<AppPool>,
    Json(payload): Json<RegisterPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repo = PlayerRepository::new(&pool);
    let new_user = NewPlayer::try_from(payload).map_err(|err| {
        error!("Failed to parse player: {}", err);
        let body = json!({ "status": "error", "message": err.to_string() });
        (StatusCode::BAD_REQUEST, Json(body))
    })?;

    match repo.exists_by_name(&new_user.name) {
        Ok(exists) => {
            if exists {
                let body = json!({ "status": "error", "message": "Username already taken" });
                return Err((StatusCode::CONFLICT, Json(body)));
            }
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

    Ok(StatusCode::CREATED)
}

#[instrument(skip(pool, session_service, jar, settings, payload), fields(username = %payload.username))]
#[debug_handler(state = AppState)]
async fn login(
    State(pool): State<AppPool>,
    State(session_service): State<SessionService>,
    jar: CookieJar,
    settings: Settings,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, AuthError> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let repo = PlayerRepository::new(&pool);
    let user = repo.get_by_name(&payload.username).map_err(|err| {
        warn!("Invalid player: {}", err);
        AuthError::WrongCredentials
    })?;

    let argon2 = Argon2::default();
    let hash = PasswordHash::new(&user.pwd_hash).map_err(|_| AuthError::ArgonError)?;
    if argon2
        .verify_password(payload.password.as_ref(), &hash)
        .is_err()
    {
        warn!("Invalid password for player: {}", user.name);
        return Err(AuthError::WrongCredentials);
    }

    let session_token = session_service.generate_session_token();
    let session = session_service
        .create_session(session_token.clone(), &user.id)
        .map_err(|_| AuthError::TokenCreation)?;
    info!(
        "Player {} logged in. Session valid until {}",
        &user.id, session.expires_at
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

#[instrument(skip(jar))]
#[debug_handler(state = AppState)]
async fn logout(
    State(srv): State<SessionService>,
    session: Extension<PlayerSession>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AuthError> {
    let jar = jar
        .remove(Cookie::from(SESSION_COOKIE_NAME))
        .remove(Cookie::from(TOKEN_COOKIE_NAME));
    srv.invalidate_session(&session.id);

    let body = json!({ "status": "ok" });
    Ok((jar, Json(body)))
}

#[derive(Serialize, Deserialize)]
pub struct PlayerDtoResponse {
    player: PlayerDto,
    session: SessionDto,
}

#[derive(Serialize, Deserialize)]
struct PlayerDto {
    id: PlayerKey,
    name: String,
    email: Option<String>,
    faction: String,
}

#[derive(Serialize, Deserialize)]
struct SessionDto {
    token: String,
    expires_at: DateTime<Utc>,
}

#[instrument(skip(jar))]
#[debug_handler(state = AppState)]
async fn session(
    State(srv): State<SessionService>,
    player: Extension<Player>,
    session: Extension<PlayerSession>,
    token: Extension<SessionToken>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AuthError> {
    let session = SessionDto {
        token: token.to_string(),
        expires_at: session.expires_at,
    };
    let player = PlayerDto {
        id: player.id,
        name: player.name.clone(),
        email: player.email.clone(),
        faction: player.faction.to_string(),
    };

    Ok((jar, Json(PlayerDtoResponse { session, player })))
}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

pub fn protected_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/logout", get(logout))
        .route("/session", get(session))
}
