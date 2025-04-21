use std::fmt::Debug;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use axum_extra::extract::CookieJar;
use cookie::{Cookie, SameSite};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, instrument, warn};

use crate::configuration::Settings;
use crate::db::extractor::DatabaseConnection;
use crate::db::players::PlayerRepository;
use crate::db::Repository;
use crate::domain::app_state::AppState;
use crate::domain::auth::{AuthBody, AuthError, Claims};
use crate::domain::factions::FactionCode;
use crate::domain::player;
use crate::domain::player::NewPlayer;
use crate::services::auth_service::{create_token_for_user, hash_password};
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

#[instrument(skip(conn, settings))]
#[debug_handler(state = AppState)]
async fn register(
    DatabaseConnection(conn): DatabaseConnection,
    settings: Settings,
    Json(payload): Json<RegisterPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut repo = PlayerRepository::from_connection(conn);
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

#[instrument(skip(conn, jar, settings))]
#[debug_handler(state = AppState)]
async fn login(
    DatabaseConnection(conn): DatabaseConnection,
    jar: CookieJar,
    settings: Settings,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, AuthError> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let mut repo = PlayerRepository::from_connection(conn);
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

    let token = create_token_for_user(user, &settings.jwt)?;
    let max_age = cookie::time::Duration::seconds(settings.jwt.expires_in as i64);
    let cookie = Cookie::build(("token", token.clone()))
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(max_age)
        .build();

    let body = AuthBody::new(token);
    let jar = jar.add(cookie);

    Ok((jar, Json(body)))
}

#[instrument(skip(jar))]
#[debug_handler(state = AppState)]
async fn logout(claims: Claims, jar: CookieJar) -> Result<impl IntoResponse, AuthError> {
    let jar = jar.remove(Cookie::from("token"));
    let body = json!({ "status": "ok" });

    Ok((jar, Json(body)))
}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", get(logout))
}
