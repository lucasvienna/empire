use std::fmt::Debug;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::auth::utils::hash_password;
use crate::domain::factions::FactionCode;
use crate::domain::player;
use crate::domain::player::{NewPlayer, PlayerKey};
use crate::ErrorKind;

#[derive(Serialize, Deserialize)]
pub struct RegisterPayload {
	pub username: String,
	pub password: String,
	pub email: Option<String>,
}

impl Debug for RegisterPayload {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RegisterPayload")
			.field("username", &self.username)
			.field("password", &"[redacted]")
			.field("email", &self.email)
			.finish()
	}
}

impl TryFrom<RegisterPayload> for NewPlayer {
	type Error = crate::Error;

	fn try_from(value: RegisterPayload) -> Result<Self, Self::Error> {
		let name = player::UserName::parse(value.username)?;
		let email: Option<player::UserEmail> = match value.email {
			None => None,
			Some(email) => Some(player::UserEmail::parse(email)?),
		};
		let pwd_hash = hash_password(&value.password)
			.map_err(|_| (ErrorKind::InternalError, "Failed to hash password"))?;
		Ok(Self {
			name,
			pwd_hash,
			email,
			faction: FactionCode::Neutral,
		})
	}
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
	pub username: String,
	pub password: String,
}

impl Debug for LoginPayload {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("LoginPayload")
			.field("username", &self.username)
			.field("password", &"[redacted]")
			.finish()
	}
}

#[derive(Serialize, Deserialize)]
pub struct PlayerDtoResponse {
	pub player: PlayerDto,
	pub session: SessionDto,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerDto {
	pub id: PlayerKey,
	pub name: String,
	pub email: Option<String>,
	pub faction: String,
}

#[derive(Serialize, Deserialize)]
pub struct SessionDto {
	pub token: String,
	pub expires_at: DateTime<Utc>,
}
