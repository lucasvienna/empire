use serde::{Deserialize, Serialize};

use crate::auth::utils::hash_password;
use crate::domain::factions::FactionCode;
use crate::domain::player;
use crate::domain::player::{NewPlayer, Player};
use crate::{Error, ErrorKind};

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

	fn try_from(req: NewUserPayload) -> crate::Result<Self, Self::Error> {
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
