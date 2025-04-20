use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};

use crate::db::extractor::DatabaseConnection;
use crate::db::users::UserRepository;
use crate::db::Repository;
use crate::domain::app_state::AppState;
use crate::domain::factions::FactionCode;
use crate::domain::user;
use crate::domain::user::{NewUser, UpdateUser, User};
use crate::services::auth_service::hash_password;
use crate::{Error, ErrorKind, Result};

/// Struct for creating a new user
#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserPayload {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub faction: FactionCode,
}

impl TryFrom<NewUserPayload> for NewUser {
    type Error = Error;

    fn try_from(req: NewUserPayload) -> Result<Self, Self::Error> {
        let email: Option<user::UserEmail> = match req.email {
            None => None,
            Some(email) => Some(user::UserEmail::parse(email)?),
        };
        let pwd_hash = hash_password(&req.password)
            .map_err(|_| (ErrorKind::InternalError, "Failed to hash password"))?;

        let user = Self {
            name: user::UserName::parse(req.username)?,
            pwd_hash,
            email,
            faction: req.faction,
        };
        Ok(user)
    }
}

/// Struct for updating user details
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserPayload {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub faction: Option<FactionCode>,
}

/// Wrapper for user id and payload
struct UpdateUserId(user::UserKey, UpdateUserPayload);

impl TryFrom<UpdateUserId> for UpdateUser {
    type Error = Error;

    fn try_from(payload: UpdateUserId) -> Result<Self, Self::Error> {
        let UpdateUserId(id, value) = payload;
        let name: Option<user::UserName> = match value.username {
            None => None,
            Some(username) => Some(user::UserName::parse(username)?),
        };
        let email: Option<user::UserEmail> = match value.email {
            None => None,
            Some(email) => Some(user::UserEmail::parse(email)?),
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
    pub id: user::UserKey,
    pub username: String,
    pub email: Option<String>,
    pub faction: FactionCode,
}

pub type UserListBody = Vec<UserBody>;

impl From<User> for UserBody {
    fn from(user: User) -> Self {
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
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<UserListBody>, StatusCode> {
    let repo = UserRepository {};

    let result = repo.get_all(&mut conn).map_err(|err| {
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
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(user_id): Path<user::UserKey>,
) -> Result<Json<UserBody>, StatusCode> {
    let repo = UserRepository {};

    let user = repo.get_by_id(&mut conn, &user_id).map_err(|err| {
        error!("Failed to fetch user: {}", err);
        StatusCode::NOT_FOUND
    })?;
    debug!(?user, "Fetched user successfully");

    Ok(Json(user.into()))
}

#[instrument(skip(conn))]
#[debug_handler(state = AppState)]
async fn create_user(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(payload): Json<NewUserPayload>,
) -> Result<(StatusCode, Json<UserBody>), StatusCode> {
    let repo = UserRepository {};
    let new_user = match NewUser::try_from(payload) {
        Ok(new_user) => new_user,
        Err(err) => {
            error!("Failed to parse user: {}", err);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let created_user = repo.create(&mut conn, new_user).map_err(|err| {
        error!("Failed to insert user: {:#?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    info!(
        user_id = created_user.id.to_string(),
        "Created user successfully"
    );

    Ok((StatusCode::CREATED, Json(created_user.into())))
}

#[instrument(skip(conn))]
#[debug_handler(state = AppState)]
async fn update_user(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(user_id): Path<user::UserKey>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = UserRepository {};

    let update: UpdateUser = match UpdateUser::try_from(UpdateUserId(user_id, payload)) {
        Ok(update) => update,
        Err(err) => {
            error!("Failed to parse user: {}", err);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let user = repo
        .get_by_id(&mut conn, &user_id)
        .map_err(|err| StatusCode::NOT_FOUND)?;

    let updated_user = repo.update(&mut conn, &update).map_err(|err| {
        error!("Failed to update user: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    info!(?updated_user, "Updated user successfully");

    Ok((StatusCode::ACCEPTED, Json(UserBody::from(updated_user))))
}

#[instrument(skip(conn))]
#[debug_handler(state = AppState)]
async fn delete_user(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(user_id): Path<user::UserKey>,
) -> Result<StatusCode, StatusCode> {
    let repo = UserRepository {};

    let count = repo.delete(&mut conn, &user_id).map_err(|err| {
        error!("Failed to delete user: {}", err);
        StatusCode::NOT_FOUND
    })?;
    info!(count, "Deleted user successfully");

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
