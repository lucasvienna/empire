use std::str::FromStr;

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use empire::domain::faction::FactionCode;
use empire::domain::modifier::active_modifier::{ModifierSourceType, NewActiveModifier};
use empire::domain::modifier::{ModifierTarget, ModifierType, NewModifier};
use empire::domain::resource::ResourceType;
use empire::domain::user::{NewUser, User, UserName};
use empire::schema::{active_modifiers, modifiers, users};
use uuid::Uuid;

mod common;

#[tokio::test]
async fn test_timespan_validation() {
    let db_pool = common::init_server().db_pool;
    let mut conn = db_pool.get().unwrap();

    // Create a test user and modifier first
    let user_id = create_test_user(&mut conn);
    let modifier_id = create_test_modifier(&mut conn);

    // Test case 1: Valid timespan (expires_at > started_at)
    let valid_modifier = NewActiveModifier {
        user_id,
        modifier_id,
        started_at: None,
        expires_at: Some(Utc::now() + Duration::hours(1)),
        source_type: ModifierSourceType::Event,
        source_id: None,
    };

    let result = diesel::insert_into(active_modifiers::table)
        .values(&valid_modifier)
        .execute(&mut conn);
    assert!(result.is_ok(), "Failed to insert valid timespan modifier");

    // Test case 2: Invalid timespan (expires_at < started_at)
    let invalid_modifier = NewActiveModifier {
        user_id,
        modifier_id,
        started_at: None,
        expires_at: Some(Utc::now() - Duration::hours(1)),
        source_type: ModifierSourceType::Event,
        source_id: None,
    };

    let result = diesel::insert_into(active_modifiers::table)
        .values(&invalid_modifier)
        .execute(&mut conn);
    assert!(
        result.is_err(),
        "Should fail: Expiration time before start time"
    );

    // Test case 3: Valid null expiration
    let no_expiry_modifier = NewActiveModifier {
        user_id,
        modifier_id,
        started_at: None,
        expires_at: None,
        source_type: ModifierSourceType::Faction,
        source_id: None,
    };

    let result = diesel::insert_into(active_modifiers::table)
        .values(&no_expiry_modifier)
        .execute(&mut conn);
    assert!(
        result.is_ok(),
        "Failed to insert modifier without expiration"
    );
}

#[tokio::test]
async fn test_cascade_deletion() {
    let db_pool = common::init_server().db_pool;
    let mut conn = db_pool.get().unwrap();

    // Create test data
    let user_id = create_test_user(&mut conn);
    let modifier_id = create_test_modifier(&mut conn);

    // Create an active modifier
    let active_modifier = NewActiveModifier {
        user_id,
        modifier_id,
        started_at: None,
        expires_at: None,
        source_type: ModifierSourceType::Event,
        source_id: None,
    };

    diesel::insert_into(active_modifiers::table)
        .values(&active_modifier)
        .execute(&mut conn)
        .expect("Failed to insert test active modifier");

    // Test case 1: Cascade on user deletion
    let delete_user_result =
        diesel::delete(users::table.filter(users::id.eq(&user_id))).execute(&mut conn);
    assert!(delete_user_result.is_ok(), "Failed to delete user");

    // Verify active modifier was deleted
    let remaining_modifiers = active_modifiers::table
        .filter(active_modifiers::user_id.eq(&user_id))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();
    assert_eq!(
        remaining_modifiers, 0,
        "Active modifier should be deleted with user"
    );

    // Create new data for the modifier cascade test
    let user_id = create_test_user(&mut conn);
    let modifier_id = create_test_modifier(&mut conn);

    let active_modifier = NewActiveModifier {
        user_id,
        modifier_id,
        started_at: None,
        expires_at: None,
        source_type: ModifierSourceType::Event,
        source_id: None,
    };

    diesel::insert_into(active_modifiers::table)
        .values(&active_modifier)
        .execute(&mut conn)
        .expect("Failed to insert test active modifier");

    // Test case 2: Cascade on modifier deletion
    let delete_modifier_result =
        diesel::delete(modifiers::table.filter(modifiers::id.eq(&modifier_id))).execute(&mut conn);
    assert!(delete_modifier_result.is_ok(), "Failed to delete modifier");

    // Verify active modifier was deleted
    let remaining_modifiers = active_modifiers::table
        .filter(active_modifiers::modifier_id.eq(&modifier_id))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();
    assert_eq!(
        remaining_modifiers, 0,
        "Active modifier should be deleted with base modifier"
    );
}

// Helper function to create a test user
fn create_test_user(conn: &mut PgConnection) -> Uuid {
    let new_user = NewUser {
        name: UserName::parse("test_user".to_string()).unwrap(),
        pwd_hash: "test_hash".to_string(),
        email: None,
        faction: FactionCode::Human,
    };

    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Failed to create test user");

    user.id
}

// Helper function to create a test modifier
fn create_test_modifier(conn: &mut PgConnection) -> Uuid {
    let new_modifier = NewModifier {
        name: format!("test_modifier_{}", Uuid::new_v4()),
        description: "Test modifier".to_string(),
        modifier_type: ModifierType::Percentage,
        magnitude: BigDecimal::from_str("0.15").unwrap(),
        target_type: ModifierTarget::Resource,
        target_resource: Some(ResourceType::Wood),
        stacking_group: None,
    };

    diesel::insert_into(modifiers::table)
        .values(&new_modifier)
        .execute(conn)
        .expect("Failed to create test modifier");

    modifiers::table
        .select(modifiers::id)
        .filter(modifiers::name.eq(&new_modifier.name))
        .first(conn)
        .expect("Failed to retrieve created modifier")
}
