use std::sync::Arc;

use diesel::prelude::*;
use diesel::update;
use empire::db::players::PlayerRepository;
use empire::db::Repository;
use empire::domain::app_state::AppPool;
use empire::domain::factions::FactionCode;
use empire::domain::player::accumulator::PlayerAccumulator;
use empire::domain::player::{NewPlayer, Player, UserName};
use empire::game::resource_service::ResourceService;
use empire::game::service::ApiService;
use empire::schema::{player_accumulator as acc, player_resource as rsc};
use empire::services::auth_service::hash_password;

mod common;

#[tokio::test]
async fn test_collect_resource() {
    let db_pool = Arc::new(common::init_server().db_pool);
    let mut conn = db_pool.get().unwrap();
    let user = create_test_user(&db_pool);
    update(acc::table.filter(acc::player_id.eq(&user.id)))
        .set((
            acc::food.eq(1000),
            acc::wood.eq(850),
            acc::stone.eq(901),
            acc::gold.eq(899),
        ))
        .returning(PlayerAccumulator::as_returning())
        .get_result(&mut conn)
        .expect("Failed to update resource accumulator");
    update(rsc::table.filter(rsc::player_id.eq(&user.id)))
        .set((
            rsc::food_cap.eq(1000),
            rsc::wood_cap.eq(1000),
            rsc::stone_cap.eq(1000),
            rsc::gold_cap.eq(1000),
        ))
        .execute(&mut conn)
        .expect("Failed to update resources");

    let srv = ResourceService::new(&db_pool);
    let res = srv
        .collect_resources(&user.id)
        .expect("Failed to collect resources");

    assert_eq!(res.food, 1000);
    assert_eq!(res.wood, 950);
    assert_eq!(res.stone, 1000);
    assert_eq!(res.gold, 999);

    // Verify that the accumulators were drained correctly
    let updated_accumulator: PlayerAccumulator = acc::table
        .filter(acc::player_id.eq(&user.id))
        .first(&mut conn)
        .expect("Failed to query resource accumulator");

    assert_eq!(updated_accumulator.food, 100);
    assert_eq!(updated_accumulator.wood, 0);
    assert_eq!(updated_accumulator.stone, 1);
    assert_eq!(updated_accumulator.gold, 0);
}

/// Create a player. Uses internal DB functions.
fn create_test_user(pool: &AppPool) -> Player {
    let repo = PlayerRepository::new(pool);
    repo.create(NewPlayer {
        name: UserName::parse("test_user".to_string()).unwrap(),
        pwd_hash: hash_password(b"1234").unwrap(),
        email: None,
        faction: FactionCode::Human,
    })
    .expect("Failed to create player")
}
