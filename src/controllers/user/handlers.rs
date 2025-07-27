use std::time::Instant;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use chrono::Utc;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::controllers::user::models::{NewUserPayload, UpdateUserPayload, UserBody, UserListBody};
use crate::db::players::PlayerRepository;
use crate::db::Repository;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::factions::FactionCode;
use crate::domain::player;
use crate::domain::player::NewPlayer;
use crate::game::player_service::PlayerService;
use crate::game::resources::resource_scheduler::ProductionScheduler;
use crate::Result;

// === CRUD HANDLERS === //
#[instrument(skip(pool))]
#[debug_handler(state = AppState)]
pub(super) async fn get_users(
    State(pool): State<AppPool>,
) -> Result<Json<UserListBody>, StatusCode> {
    debug!("Starting fetch all users");
    let repo = PlayerRepository::new(&pool);

    let result = repo.get_all().map_err(|err| {
        error!("Failed to fetch users: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let count = result.len();
    let response: UserListBody = result.into_iter().map(UserBody::from).collect();

    info!(count, "Completed fetch all users successfully");

    Ok(Json(response))
}

#[instrument(skip(pool), fields(player_id = ?player_id))]
#[debug_handler(state = AppState)]
pub(super) async fn get_user_by_id(
    State(pool): State<AppPool>,
    Path(player_id): Path<player::PlayerKey>,
) -> Result<Json<UserBody>, StatusCode> {
    debug!("Starting fetch user by ID");
    let repo = PlayerRepository::new(&pool);

    let user = repo.get_by_id(&player_id).map_err(|err| {
        // TODO: differentiate between not found and other errors
        error!(player_id = %player_id, "Failed to fetch player: {}", err);
        StatusCode::NOT_FOUND
    })?;

    info!(player_id = %player_id, "Completed fetch user successfully");
    trace!(?user, "User details");

    Ok(Json(user.into()))
}

#[instrument(skip(pool, prod_scheduler), fields(username = ?payload.username, faction = ?payload.faction))]
#[debug_handler(state = AppState)]
pub(super) async fn create_user(
    State(pool): State<AppPool>,
    State(prod_scheduler): State<ProductionScheduler>,
    Json(payload): Json<NewUserPayload>,
) -> Result<(StatusCode, Json<UserBody>), StatusCode> {
    // AIDEV-NOTE: Critical user creation path with production scheduling
    debug!("Starting user creation");
    let start = Instant::now();
    let repo = PlayerRepository::new(&pool);

    let new_user = match NewPlayer::try_from(payload) {
        Ok(new_user) => new_user,
        Err(err) => {
            warn!(error = %err, "User validation failed");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let created_user = repo.create(new_user).map_err(|err| {
        error!(error = %err, "Failed to insert player in database");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    debug!(player_id = %created_user.id, "User created, checking faction for production scheduling");

    if created_user.faction != FactionCode::Neutral {
        debug!(player_id = %created_user.id, faction = ?created_user.faction, "Scheduling initial production");
        prod_scheduler
            .schedule_production(&created_user.id, Utc::now())
            .map_err(|err| {
                error!(player_id = %created_user.id, error = %err, "Failed to schedule production");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    let duration = start.elapsed();
    info!(
        player_id = %created_user.id,
        username = %created_user.name,
        faction = ?created_user.faction,
        duration_ms = duration.as_millis(),
        "Completed user creation successfully"
    );

    Ok((StatusCode::CREATED, Json(created_user.into())))
}

#[instrument(skip(srv), fields(player_id = ?player_key))]
#[debug_handler(state = AppState)]
pub(super) async fn update_user(
    State(srv): State<PlayerService>,
    Path(player_key): Path<player::PlayerKey>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    // AIDEV-NOTE: User update with potential faction change requires production scheduling
    debug!("Starting user update");
    let start = Instant::now();

    // Track state changes for key fields
    let name_changed = payload.username.is_some();
    let email_changed = payload.email.is_some();
    let password_changed = payload.password.is_some();
    let faction_changed = payload.faction.is_some();

    let updated_user = srv.update_player(player_key, payload)?;
    let duration = start.elapsed();
    info!(
        player_id = %player_key,
        name_changed = name_changed,
        email_changed = email_changed,
        password_changed = password_changed,
        faction_changed = faction_changed,
        duration_ms = duration.as_millis(),
        "Completed user update successfully"
    );

    trace!(?updated_user, "Updated user details");

    Ok((StatusCode::ACCEPTED, Json(UserBody::from(updated_user))))
}

#[instrument(skip(pool), fields(player_id = ?player_id))]
#[debug_handler(state = AppState)]
pub(super) async fn delete_user(
    State(pool): State<AppPool>,
    Path(player_id): Path<player::PlayerKey>,
) -> Result<StatusCode, StatusCode> {
    debug!(player_id = %player_id, "Starting user deletion");
    let start = Instant::now();
    let repo = PlayerRepository::new(&pool);

    // First check if user exists
    if let Err(err) = repo.get_by_id(&player_id) {
        warn!(player_id = %player_id, error = %err, "Attempted to delete non-existent user");
        return Err(StatusCode::NOT_FOUND);
    }

    let count = repo.delete(&player_id).map_err(|err| {
        error!(player_id = %player_id, error = %err, "Failed to delete player from database");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let duration = start.elapsed();
    info!(
        player_id = %player_id,
        count = count,
        duration_ms = duration.as_millis(),
        "Completed user deletion successfully"
    );

    Ok(StatusCode::NO_CONTENT)
}
