use axum::extract::FromRef;
use axum::http::StatusCode;
use chrono::Utc;
use tracing::{debug, error, info, warn};

use crate::auth::utils::hash_password;
use crate::controllers::UpdateUserPayload;
use crate::db::players::PlayerRepository;
use crate::db::Repository;
use crate::domain::app_state::AppState;
use crate::domain::factions::FactionCode;
use crate::domain::player;
use crate::domain::player::{Player, PlayerKey, UpdatePlayer};
use crate::game::resources::resource_scheduler::ProductionScheduler;
use crate::{Error, ErrorKind, Result};

pub struct PlayerService {
    repo: PlayerRepository,
    prod_scheduler: ProductionScheduler,
}

impl FromRef<AppState> for PlayerService {
    fn from_ref(state: &AppState) -> Self {
        Self {
            repo: PlayerRepository::new(&state.db_pool),
            prod_scheduler: ProductionScheduler::new(&state.job_queue),
        }
    }
}

impl PlayerService {
    pub fn update_user(
        &self,
        player_key: PlayerKey,
        payload: UpdateUserPayload,
    ) -> Result<Player, StatusCode> {
        let changeset: UpdatePlayer =
            match UpdatePlayer::try_from(UpdateUserId(player_key, payload)) {
                Ok(update) => update,
                Err(err) => {
                    warn!(player_id = %player_key, error = %err, "User update validation failed");
                    return Err(StatusCode::BAD_REQUEST);
                }
            };

        let user = self.repo.get_by_id(&player_key).map_err(|err| {
            error!(player_id = %player_key, error = %err, "User not found for update");
            StatusCode::NOT_FOUND
        })?;

        debug!(player_id = %player_key, "Found existing user, applying changes");

        let updated_user = self.repo.update(&changeset).map_err(|err| {
            error!(player_id = %player_key, error = %err, "Failed to update player in database");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        // Track state changes for key fields
        let name_changed = changeset.name.is_some();
        let email_changed = changeset.email.is_some();
        let password_changed = changeset.pwd_hash.is_some();
        let faction_changed =
            changeset.faction.is_some() && changeset.faction != Some(user.faction);

        if faction_changed && user.faction == FactionCode::Neutral {
            debug!(
                player_id = %player_key,
                old_faction = ?user.faction,
                new_faction = ?updated_user.faction,
                "Faction changed from Neutral, scheduling production"
            );

            self.prod_scheduler
                .schedule_production(&player_key, Utc::now())
                .map_err(|err| {
                    error!(player_id = %player_key, error = %err, "Failed to schedule production after faction change");
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
        }

        info!(
            player_id = %player_key,
            name_changed = name_changed,
            email_changed = email_changed,
            password_changed = password_changed,
            faction_changed = faction_changed,
            "Completed user update successfully"
        );

        Ok(updated_user)
    }
}

/// Wrapper for player id and payload
struct UpdateUserId(PlayerKey, UpdateUserPayload);

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
