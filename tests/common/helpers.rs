use axum_extra::headers;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use empire::auth::utils::hash_password;
use empire::db::players::PlayerRepository;
use empire::db::Repository;
use empire::domain::app_state::AppPool;
use empire::domain::auth::{encode_token, Claims};
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

pub(super) fn create_test_user(pool: &AppPool, faction: Option<FactionCode>) -> Player {
    let user_repo = PlayerRepository::new(pool);
    user_repo
        .create(NewPlayer {
            name: UserName::parse("test_game_user".to_string()).unwrap(),
            pwd_hash: hash_password(b"1234").unwrap(),
            email: None,
            faction: faction.unwrap_or(FactionCode::Neutral),
        })
        .expect("Failed to create player")
}
