use crate::db::extractor::DatabaseConnection;
use crate::db::users::UserRepository;
use crate::db::Repository;
use crate::domain::user;
use crate::domain::user::{NewUser, User};
use crate::net::server::AppState;
use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, error, info, instrument};

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

pub type UsersResponse = Vec<UserResponse>;

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

#[instrument(skip(conn))]
async fn get_users(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<UsersResponse>, StatusCode> {
    let repo = UserRepository {};

    let result = repo.get_all(&mut conn).map_err(|err| {
        error!("Failed to fetch users: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    debug!("Fetched {} users successfully", result.len());

    let response: UsersResponse = result.into_iter().map(UserResponse::from).collect();

    Ok(Json(response))
}

#[instrument(skip(conn))]
async fn get_user_by_id(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(user_id): Path<user::PK>,
) -> Result<Json<UserResponse>, StatusCode> {
    let repo = UserRepository {};

    let user = repo.get_by_id(&mut conn, &user_id).map_err(|err| {
        error!("Failed to fetch user: {}", err);
        StatusCode::NOT_FOUND
    })?;
    debug!(?user, "Fetched user successfully");

    Ok(Json(user.into()))
}

#[instrument(skip(conn))]
async fn create_user(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    let repo = UserRepository {};

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

#[instrument(skip(conn))]
async fn update_user(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(user_id): Path<user::PK>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let repo = UserRepository {};

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

#[instrument(skip(conn))]
async fn delete_user(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(user_id): Path<user::PK>,
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
