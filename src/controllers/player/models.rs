use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::controllers::user::UpdateUserPayload;
use crate::domain::factions::FactionCode;
use crate::domain::player::{Player, PlayerKey};

#[derive(Serialize, Deserialize)]
pub struct PlayerProfileResponse {
    pub id: PlayerKey,
    pub username: String,
    pub email: Option<String>,
    pub faction: Option<FactionCode>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Player> for PlayerProfileResponse {
    fn from(value: Player) -> Self {
        let faction = if value.faction == FactionCode::Neutral {
            None
        } else {
            Some(value.faction)
        };
        Self {
            id: value.id,
            username: value.name,
            email: value.email,
            faction,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct JoinFactionPayload {
    faction: FactionCode,
}

impl From<JoinFactionPayload> for UpdateUserPayload {
    fn from(value: JoinFactionPayload) -> Self {
        Self {
            username: None,
            password: None,
            email: None,
            faction: Some(value.faction),
        }
    }
}
