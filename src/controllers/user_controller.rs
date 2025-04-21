use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};

use crate::db::extractor::DatabaseConnection;
use crate::db::players::PlayerRepository;
use crate::db::Repository;
use crate::domain::app_state::AppState;
use crate::domain::factions::FactionCode;
use crate::domain::player;
use crate::domain::player::{NewPlayer, Player, UpdatePlayer};
use crate::services::auth_service::hash_password;
use crate::{Error, ErrorKind, Result};

/// Struct for creating a new player
#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserPayload {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub faction: FactionCode,
}

impl TryFrom<NewUserPayload> for NewPlayer {
    type Error = Error;

    fn try_from(req: NewUserPayload) -> Result<Self, Self::Error> {
        let email: Option<player::UserEmail> = match req.email {
            None => None,
            Some(email) => Some(player::UserEmail::parse(email)?),
        };
        let pwd_hash = hash_password(&req.password)
            .map_err(|_| (ErrorKind::InternalError, "Failed to hash password"))?;

        let user = Self {
            name: player::UserName::parse(req.username)?,
            pwd_hash,
            email,
            faction: req.faction,
        };
        Ok(user)
    }
}

/// Struct for updating player details
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserPayload {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub faction: Option<FactionCode>,
}

/// Wrapper for player id and payload
struct UpdateUserId(player::PlayerKey, UpdateUserPayload);

impl TryFrom<UpdateUserId> for UpdatePlayer {
    type Error = Error;

    fn try_from(payload: UpdateUserId) -> Result<Self, Self::Error> {
        let UpdateUserId(id, value) = payload;
        let name: Option<player::UserName> = match value.username {
            None => None,
            Some(username) => Some(player::UserName::parse(username)?),
        };
        let email: Option<player::UserEmail> = match value.email {
            None => None,
            Some(email) => Some(player::UserEmail::parse(email)?),
        };
        let pwd_hash = match value.password {
            None => None,
            Some(password) => {
                let pwd_hash = hash_password(&password)
                    .map_err(|_| (ErrorKind::InternalError, "Failed to hash password"))?;
                Some(pwd_hash)
            }
        };

        let update = Self {
            id,
            name,
            email,
            pwd_hash,
            faction: value.faction,
        };
        Ok(update)
    }
}

/// Struct for response data
#[derive(Serialize, Deserialize, Debug)]
pub struct UserBody {
    pub id: player::PlayerKey,
    pub username: String,
    pub email: Option<String>,
    pub faction: FactionCode,
}

pub type UserListBody = Vec<UserBody>;

impl From<Player> for UserBody {
    fn from(user: Player) -> Self {
        Self {
            id: user.id,
            username: user.name,
            email: user.email,
            faction: user.faction,
        }
    }
}

// === CRUD HANDLERS === //

#[instrument(skip(conn))]
#[debug_handler(state = AppState)]
async fn get_users(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<Json<UserListBody>, StatusCode> {
    let mut repo = PlayerRepository::from_connection(conn);

    let result = repo.get_all().map_err(|err| {
        error!("Failed to fetch users: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    debug!("Fetched {} users successfully", result.len());

    let response: UserListBody = result.into_iter().map(UserBody::from).collect();

    Ok(Json(response))
}

#[instrument(skip(conn))]
#[debug_handler(state = AppState)]
async fn get_user_by_id(
    DatabaseConnection(conn): DatabaseConnection,
    Path(player_id): Path<player::PlayerKey>,
) -> Result<Json<UserBody>, StatusCode> {
    let mut repo = PlayerRepository::from_connection(conn);

    let user = repo.get_by_id(&player_id).map_err(|err| {
        error!("Failed to fetch player: {}", err);
        StatusCode::NOT_FOUND
    })?;
    debug!(?user, "Fetched player successfully");

    Ok(Json(user.into()))
}

#[instrument(skip(conn))]
#[debug_handler(state = AppState)]
async fn create_user(
    DatabaseConnection(conn): DatabaseConnection,
    Json(payload): Json<NewUserPayload>,
) -> Result<(StatusCode, Json<UserBody>), StatusCode> {
    let mut repo = PlayerRepository::from_connection(conn);
    let new_user = match NewPlayer::try_from(payload) {
        Ok(new_user) => new_user,
        Err(err) => {
            error!("Failed to parse player: {}", err);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let created_user = repo.create(new_user).map_err(|err| {
        error!("Failed to insert player: {:#?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    info!(
        player_id = created_user.id.to_string(),
        "Created player successfully"
    );

    Ok((StatusCode::CREATED, Json(created_user.into())))
}

#[instrument(skip(conn))]
#[debug_handler(state = AppState)]
async fn update_user(
    DatabaseConnection(conn): DatabaseConnection,
    Path(player_id): Path<player::PlayerKey>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut repo = PlayerRepository::from_connection(conn);

    let changeset: UpdatePlayer = match UpdatePlayer::try_from(UpdateUserId(player_id, payload)) {
        Ok(update) => update,
        Err(err) => {
            error!("Failed to parse player: {}", err);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let user = repo
        .get_by_id(&player_id)
        .map_err(|err| StatusCode::NOT_FOUND)?;

    let updated_user = repo.update(&changeset).map_err(|err| {
        error!("Failed to update player: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    info!(?updated_user, "Updated player successfully");

    Ok((StatusCode::ACCEPTED, Json(UserBody::from(updated_user))))
}

#[instrument(skip(conn))]
#[debug_handler(state = AppState)]
async fn delete_user(
    DatabaseConnection(conn): DatabaseConnection,
    Path(player_id): Path<player::PlayerKey>,
) -> Result<StatusCode, StatusCode> {
    let mut repo = PlayerRepository::from_connection(conn);

    let count = repo.delete(&player_id).map_err(|err| {
        error!("Failed to delete player: {}", err);
        StatusCode::NOT_FOUND
    })?;
    info!(count, "Deleted player successfully");

    Ok(StatusCode::NO_CONTENT)
}

// === ROUTES === //

pub fn user_routes() -> Router<AppState> {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/", get(get_users).post(create_user))
            .route(
                "/{id}",
                get(get_user_by_id).put(update_user).delete(delete_user),
            ),
    )
}
