use crate::db::users::UserRepository;
use crate::db::Repository;
use crate::models::user;
use crate::models::user::{NewUser, User};
use crate::net::server::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, error, info, instrument, trace};

/// Struct for creating a new user
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub faction: i32,
}

impl From<CreateUserRequest> for NewUser {
    fn from(user: CreateUserRequest) -> Self {
        Self {
            name: user.username,
            faction: user.faction,
            data: Some(Value::default()),
        }
    }
}

/// Struct for updating user details
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserRequest {
    pub username: String,
    pub faction: i32,
}

/// Struct for response data
#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: user::PK,
    pub username: String,
    pub faction: i32,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.name,
            faction: user.faction,
        }
    }
}

// === CRUD HANDLERS === //

#[instrument(skip(state))]
async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let repo = UserRepository {};
    let mut conn = state.db_pool.get().map_err(|err| {
        error!("Failed to get a database connection: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    trace!("Acquired a database connection.");

    let result = repo.get_all(&mut conn).map_err(|err| {
        error!("Failed to fetch users: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    debug!("Fetched {} users successfully", result.len());

    let response: Vec<UserResponse> = result.into_iter().map(UserResponse::from).collect();

    Ok(Json(response))
}

#[instrument(skip(state))]
async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<user::PK>,
) -> Result<Json<UserResponse>, StatusCode> {
    let repo = UserRepository {};
    let mut conn = state.db_pool.get().map_err(|err| {
        error!("Failed to acquire a database connection: {:#?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    trace!("Acquired a database connection.");

    let user = repo.get_by_id(&mut conn, &user_id).map_err(|err| {
        error!("Failed to fetch user: {}", err);
        StatusCode::NOT_FOUND
    })?;
    debug!(?user, "Fetched user successfully");

    Ok(Json(user.into()))
}

#[instrument(skip(state))]
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    let repo = UserRepository {};
    let mut conn = state.db_pool.get().map_err(|err| {
        error!("Failed to get a database connection: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    trace!("Acquired a database connection.");

    let created_user = repo.create(&mut conn, &payload.into()).map_err(|err| {
        error!("Failed to insert user: {:#?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    info!(
        user_id = created_user.id.to_string(),
        "Created user successfully"
    );

    Ok((StatusCode::CREATED, Json(created_user.into())))
}

#[instrument(skip(state))]
async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<user::PK>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let repo = UserRepository {};
    let mut conn = state.db_pool.get().map_err(|err| {
        error!("Failed to get a database connection: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    trace!("Acquired a database connection.");

    let user = User {
        id: user_id,
        name: payload.username,
        faction: payload.faction,
        data: Some(Value::default()),
    };

    let updated_user = repo.update(&mut conn, &user).map_err(|err| {
        error!("Failed to update user: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    info!(?updated_user, "Updated user successfully");

    Ok(Json(updated_user.into()))
}

#[instrument(skip(state))]
async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<user::PK>,
) -> Result<StatusCode, StatusCode> {
    let repo = UserRepository {};
    let mut conn = state.db_pool.get().map_err(|err| {
        error!("Failed to get a database connection: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    trace!("Acquired a database connection.");

    let count = repo.delete(&mut conn, &user_id).map_err(|err| {
        error!("Failed to delete user: {}", err);
        StatusCode::NOT_FOUND
    })?;
    info!(count, "Deleted user successfully");

    Ok(StatusCode::NO_CONTENT)
}

// === ROUTES === //

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/{id}",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
}
