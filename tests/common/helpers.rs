use axum_extra::headers;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use empire::auth::utils::hash_password;
use empire::db::{DbConn, players};
use empire::domain::auth::{Claims, encode_token};
use empire::domain::factions::FactionCode;
use empire::domain::player::{NewPlayer, Player, PlayerKey, UserName};

pub(super) fn get_bearer(player_id: &PlayerKey) -> Authorization<Bearer> {
	let now = chrono::Utc::now();
	let token = encode_token(Claims {
		sub: *player_id,
		iat: now.timestamp() as usize,
		exp: (now + chrono::Duration::minutes(5)).timestamp() as usize,
	})
	.unwrap();

	headers::Authorization::bearer(&token).unwrap()
}

pub(super) fn create_test_user(conn: &mut DbConn, faction: Option<FactionCode>) -> Player {
	players::create(
		conn,
		NewPlayer {
			name: UserName::parse("test_game_user".to_string()).unwrap(),
			pwd_hash: hash_password(b"1234").unwrap(),
			email: None,
			faction: faction.unwrap_or(FactionCode::Neutral),
		},
	)
	.expect("Failed to create player")
}
