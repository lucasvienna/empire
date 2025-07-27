use serde::{Deserialize, Serialize};

use crate::controllers::user::UpdateUserPayload;
use crate::domain::factions::FactionCode;
use crate::domain::player::PlayerKey;

#[derive(Serialize, Deserialize)]
pub struct PlayerProfileResponse {
    pub id: PlayerKey,
    pub username: String,
    pub level: u32,
    pub experience: u64,
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
