use diesel::prelude::*;
use empire::db::users::UserRepository;
use empire::db::{DbConn, Repository};
use empire::domain::resource_accumulator::ResourceAccumulator;
use empire::domain::user::{NewUser, User, UserName};
use empire::game::resource_service::ResourceService;
use empire::schema::resources_accumulator as ra;
use empire::services::auth_service::hash_password;

mod common;

#[tokio::test]
async fn test_collect_resource() {
    let db_pool = common::init_server().db_pool;
    let mut conn = db_pool.get().unwrap();
    let user = create_test_user(db_pool.get().unwrap());
    diesel::update(ra::table.filter(ra::user_id.eq(&user.id)))
        .set((
            ra::food.eq(1000),
            ra::wood.eq(850),
            ra::stone.eq(860),
            ra::gold.eq(899),
        ))
        .returning(ResourceAccumulator::as_returning())
        .get_result(&mut conn)
        .expect("Failed to update resource accumulator");

    let mut srv = ResourceService::new(db_pool.get().unwrap());
    let res = srv
        .collect_resources(&user.id)
        .expect("Failed to collect resources");

    assert_eq!(res.food, 1000);
    assert_eq!(res.wood, 950);
    assert_eq!(res.stone, 960);
    assert_eq!(res.gold, 999);
}

/// Create a user. Uses internal DB functions.
fn create_test_user(mut conn: DbConn) -> User {
    let user_repo = UserRepository {};
    user_repo
        .create(
            &mut conn,
            &NewUser {
                name: UserName::parse("test_user".to_string()).unwrap(),
                pwd_hash: hash_password(b"1234").unwrap(),
                email: None,
                faction: 2,
            },
        )
        .unwrap()
}
