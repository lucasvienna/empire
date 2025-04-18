use std::str::FromStr;

use bigdecimal::BigDecimal;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use empire::db::users::UserRepository;
use empire::db::{DbConn, Repository};
use empire::domain::faction::FactionCode;
use empire::domain::modifier_history::ModifierActionType;
use empire::domain::user::{NewUser, User, UserName};
use empire::schema::{modifier_history, modifiers, user_active_modifiers};
use empire::services::auth_service::hash_password;

mod common;

#[tokio::test]
async fn test_faction_modifier_on_create() {
    let app = common::spawn_app();
    let mut conn = app.db_pool.get().unwrap();

    // Create a user with Human faction
    let user = create_test_user(&mut conn, FactionCode::Human);

    // Verify active modifiers
    let active_modifiers: Vec<(String, BigDecimal)> = user_active_modifiers::table
        .inner_join(modifiers::table.on(modifiers::id.eq(&user_active_modifiers::modifier_id)))
        .filter(user_active_modifiers::user_id.eq(&user.id))
        .select((modifiers::name, user_active_modifiers::magnitude))
        .load::<(String, BigDecimal)>(&mut conn)
        .unwrap();

    assert_eq!(
        active_modifiers.len(),
        3,
        "Human faction should have 3 modifiers"
    );

    // Expected values
    let wood_prod = BigDecimal::from_str("1.1500").unwrap();
    let cavalry_training = BigDecimal::from_str("1.1500").unwrap();
    let cavalry_combat = BigDecimal::from_str("1.1500").unwrap();

    // Check each modifier
    let has_wood_bonus = active_modifiers
        .iter()
        .any(|(name, magnitude)| name == "human_wood_production" && magnitude == &wood_prod);
    let has_cavalry_training = active_modifiers.iter().any(|(name, magnitude)| {
        name == "human_cavalry_training" && magnitude == &cavalry_training
    });
    let has_cavalry_combat = active_modifiers
        .iter()
        .any(|(name, magnitude)| name == "human_cavalry_combat" && magnitude == &cavalry_combat);

    assert!(has_wood_bonus, "Missing or incorrect wood production bonus");
    assert!(
        has_cavalry_training,
        "Missing or incorrect cavalry training bonus"
    );
    assert!(
        has_cavalry_combat,
        "Missing or incorrect cavalry combat bonus"
    );

    // Verify modifier history
    let history: Vec<(String, ModifierActionType, BigDecimal)> = modifier_history::table
        .inner_join(modifiers::table.on(modifiers::id.eq(&modifier_history::modifier_id)))
        .filter(modifier_history::user_id.eq(&user.id))
        .select((
            modifiers::name,
            modifier_history::action_type,
            modifier_history::magnitude,
        ))
        .load::<(String, ModifierActionType, BigDecimal)>(&mut conn)
        .unwrap();

    assert_eq!(history.len(), 3, "Should have 3 history entries");
    assert!(
        history
            .iter()
            .all(|(_, action, _)| ModifierActionType::Applied.eq(action)),
        "All actions should be 'applied' for new user"
    );
}

#[tokio::test]
async fn test_faction_change() {
    let app = common::spawn_app();
    let mut conn = app.db_pool.get().unwrap();

    // Create user with Human faction
    let user = create_test_user(&mut conn, FactionCode::Human);

    // Change faction to Orc
    diesel::update(empire::schema::users::table.filter(empire::schema::users::id.eq(user.id)))
        .set(empire::schema::users::faction.eq(FactionCode::Orc))
        .execute(&mut conn)
        .unwrap();

    // Verify active modifiers
    let active_modifiers: Vec<(String, BigDecimal)> = user_active_modifiers::table
        .inner_join(modifiers::table.on(modifiers::id.eq(&user_active_modifiers::modifier_id)))
        .filter(user_active_modifiers::user_id.eq(&user.id))
        .select((modifiers::name, user_active_modifiers::magnitude))
        .load::<(String, BigDecimal)>(&mut conn)
        .unwrap();

    assert_eq!(active_modifiers.len(), 3, "Should have 3 Orc modifiers");

    // Expected values
    let stone_prod = BigDecimal::from_str("1.1500").unwrap();
    let infantry_training = BigDecimal::from_str("1.1500").unwrap();
    let infantry_combat = BigDecimal::from_str("1.1500").unwrap();

    // Check each modifier
    let has_stone_bonus = active_modifiers
        .iter()
        .any(|(name, magnitude)| name == "orc_stone_production" && magnitude == &stone_prod);
    let has_infantry_training = active_modifiers.iter().any(|(name, magnitude)| {
        name == "orc_infantry_training" && magnitude == &infantry_training
    });
    let has_infantry_combat = active_modifiers
        .iter()
        .any(|(name, magnitude)| name == "orc_infantry_combat" && magnitude == &infantry_combat);

    assert!(
        has_stone_bonus,
        "Missing or incorrect stone production bonus"
    );
    assert!(
        has_infantry_training,
        "Missing or incorrect infantry training bonus"
    );
    assert!(
        has_infantry_combat,
        "Missing or incorrect infantry combat bonus"
    );

    // Verify modifier history
    let history: Vec<(String, ModifierActionType, BigDecimal)> = modifier_history::table
        .inner_join(modifiers::table.on(modifiers::id.eq(&modifier_history::modifier_id)))
        .filter(modifier_history::user_id.eq(&user.id))
        .order_by(modifier_history::occurred_at.asc())
        .select((
            modifiers::name,
            modifier_history::action_type,
            modifier_history::magnitude,
        ))
        .load::<(String, ModifierActionType, BigDecimal)>(&mut conn)
        .unwrap();

    assert_eq!(
        history.len(),
        9,
        "Should have 6 history entries (6 applied + 3 removed)"
    );

    // First 3 entries should be initial Human modifiers being applied
    assert!(history
        .iter()
        .take(3)
        .all(|(_, action, _)| ModifierActionType::Applied.eq(action)));
    // Next 3 entries should be Human modifiers being removed and Orc modifiers being applied
    assert!(history
        .iter()
        .skip(3)
        .take(3)
        .all(|(_, action, _)| ModifierActionType::Removed.eq(action)));
    // Last 3 entries should be the new Orc modifiers being applied
    assert!(history
        .iter()
        .skip(3)
        .take(3)
        .all(|(_, action, _)| ModifierActionType::Removed.eq(action)));
}

// Helper function to create test users
fn create_test_user(conn: &mut DbConn, faction: FactionCode) -> User {
    let user_repo = UserRepository {};
    user_repo
        .create(
            conn,
            NewUser {
                name: UserName::parse("test_user".to_string()).unwrap(),
                pwd_hash: hash_password(b"1234").unwrap(),
                email: None,
                faction,
            },
        )
        .unwrap()
}
