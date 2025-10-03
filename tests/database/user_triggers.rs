use empire::db::players::PlayerRepository;
use empire::db::{player_buildings, Repository};
use empire::domain::factions::FactionCode;
use empire::domain::player::{NewPlayer, UserName};

use crate::common::TestHarness;

#[tokio::test]
async fn test_user_triggers() {
	let server = TestHarness::new();
	let player_repo = PlayerRepository::new(&server.app.db_pool);

	let new_player = player_repo
		.create(NewPlayer {
			name: UserName::parse("test123".parse().unwrap()).unwrap(),
			pwd_hash: "password1".to_string(),
			email: None,
			faction: FactionCode::Human,
		})
		.expect("Failed to create player");

	assert_eq!(
		new_player.faction,
		FactionCode::Human,
		"Faction should be human"
	);

	let mut conn = server.db_pool.get().expect("Failed to get connection");
	let new_buildings = player_buildings::get_player_buildings(&mut conn, &new_player.id)
		.expect("Failed to get player buildings");
	assert_ne!(
		new_buildings.len(),
		0,
		"Player should have starter buildings"
	);
}
