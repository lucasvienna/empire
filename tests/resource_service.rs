use diesel::prelude::*;
use diesel::update;
use empire::db::users::UserRepository;
use empire::db::{DbConn, Repository};
use empire::domain::accumulator::UserAccumulator;
use empire::domain::faction::FactionCode;
use empire::domain::user::{NewUser, User, UserName};
use empire::game::resource_service::ResourceService;
use empire::schema::{user_accumulator as acc, user_resources as rsc};
use empire::services::auth_service::hash_password;

mod common;

#[tokio::test]
async fn test_collect_resource() {
    let db_pool = common::init_server().db_pool;
    let mut conn = db_pool.get().unwrap();
    let user = create_test_user(db_pool.get().unwrap());
    update(acc::table.filter(acc::user_id.eq(&user.id)))
        .set((
            acc::food.eq(1000),
            acc::wood.eq(850),
            acc::stone.eq(901),
            acc::gold.eq(899),
        ))
        .returning(UserAccumulator::as_returning())
        .get_result(&mut conn)
        .expect("Failed to update resource accumulator");
    update(rsc::table.filter(rsc::user_id.eq(&user.id)))
        .set((
            rsc::food_cap.eq(1000),
            rsc::wood_cap.eq(1000),
            rsc::stone_cap.eq(1000),
            rsc::gold_cap.eq(1000),
        ))
        .execute(&mut conn)
        .expect("Failed to update resources");

    let mut srv = ResourceService::new(db_pool.get().unwrap());
    let res = srv
        .collect_resources(&user.id)
        .expect("Failed to collect resources");

    assert_eq!(res.food, 1000);
    assert_eq!(res.wood, 950);
    assert_eq!(res.stone, 1000);
    assert_eq!(res.gold, 999);

    // Verify that the accumulators were drained correctly
    let updated_accumulator: UserAccumulator = acc::table
        .filter(acc::user_id.eq(&user.id))
        .first(&mut conn)
        .expect("Failed to query resource accumulator");

    assert_eq!(updated_accumulator.food, 100);
    assert_eq!(updated_accumulator.wood, 0);
    assert_eq!(updated_accumulator.stone, 1);
    assert_eq!(updated_accumulator.gold, 0);
}

/// Create a user. Uses internal DB functions.
fn create_test_user(mut conn: DbConn) -> User {
    let user_repo = UserRepository {};
    user_repo
        .create(
            &mut conn,
            NewUser {
                name: UserName::parse("test_user".to_string()).unwrap(),
                pwd_hash: hash_password(b"1234").unwrap(),
                email: None,
                faction: FactionCode::Human,
            },
        )
        .unwrap()
}
