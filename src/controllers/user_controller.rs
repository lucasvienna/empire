use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::db::users::UserRepository;
use crate::db::Repository;
use crate::models::user::{NewUser, User};
use crate::net::server::AppState;

/// Struct for creating a new user
#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub faction: i32,
}

/// Struct for updating user details
#[derive(Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub faction: i32,
}

/// Struct for response data
#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub faction: i32,
}

// === CRUD HANDLERS === //

// Get all users
async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let repo = UserRepository {};
    let mut conn = state.db_pool.get().map_err(|err| {
        error!("Failed to get a database connection: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let result = repo.get_all(&mut conn).map_err(|err| {
        error!("Failed to fetch users: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response: Vec<UserResponse> = result
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            username: user.name,
            faction: user.faction,
        })
        .collect();

    Ok(Json(response))
}

// Get a user by ID
async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserResponse>, StatusCode> {
    let repo = UserRepository {};
    let mut conn = state.db_pool.get().map_err(|err| {
        error!("Failed to get a database connection: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = repo.get_by_id(&mut conn, &user_id).map_err(|err| {
        error!("Failed to fetch user: {}", err);
        StatusCode::NOT_FOUND
    })?;

    Ok(Json(UserResponse {
        id: user.id,
        username: user.name,
        faction: user.faction,
    }))
}

// Create a new user
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    let repo = UserRepository {};
    let mut conn = state.db_pool.get().map_err(|err| {
        error!("Failed to get a database connection: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let new_user = NewUser {
        name: payload.username.as_str(),
        faction: payload.faction,
        data: None,
    };

    let created_user = repo.create(&mut conn, &new_user).map_err(|err| {
        error!("Failed to create user: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        StatusCode::CREATED,
        Json(UserResponse {
            id: created_user.id,
            username: created_user.name,
            faction: created_user.faction,
        }),
    ))
}

// Update an existing user
async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let repo = UserRepository {};
    let mut conn = state.db_pool.get().map_err(|err| {
        error!("Failed to get a database connection: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = User {
        id: user_id,
        name: payload.username.clone(),
        faction: payload.faction,
        data: None,
    };

    let updated_user = repo.update(&mut conn, &user).map_err(|err| {
        error!("Failed to update user: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(UserResponse {
        id: updated_user.id,
        username: updated_user.name,
        faction: updated_user.faction,
    }))
}

// Delete a user
async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let repo = UserRepository {};
    let mut conn = state.db_pool.get().map_err(|err| {
        error!("Failed to get a database connection: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    repo.delete(&mut conn, &user_id).map_err(|err| {
        error!("Failed to delete user: {}", err);
        StatusCode::NOT_FOUND
    })?;

    Ok(StatusCode::NO_CONTENT)
}

// === ROUTES === //

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
}
