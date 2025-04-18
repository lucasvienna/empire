use diesel::prelude::*;
use empire::domain::modifier::{ModifierTarget, ModifierType};
use empire::domain::resource::ResourceType;
use empire::schema::modifiers;

mod common;

#[derive(Insertable)]
#[diesel(table_name = modifiers)]
struct NewModifier {
    name: String,
    description: String,
    modifier_type: ModifierType,
    target_type: ModifierTarget,
    target_resource: Option<ResourceType>,
    stacking_group: Option<String>,
}

#[tokio::test]
async fn test_valid_modifier_constraints() {
    let db_pool = common::init_server().db_pool;
    let mut conn = db_pool.get().unwrap();

    // Test resource modifier with valid resource type
    let wood_modifier = NewModifier {
        name: "wood_boost".to_string(),
        description: "Increases wood production".to_string(),
        modifier_type: ModifierType::Percentage,
        target_type: ModifierTarget::Resource,
        target_resource: Some(ResourceType::Wood),
        stacking_group: None,
    };

    let result = diesel::insert_into(modifiers::table)
        .values(&wood_modifier)
        .execute(&mut conn);
    assert!(result.is_ok(), "Failed to insert valid resource modifier");

    // Test combat modifier with null resource
    let combat_modifier = NewModifier {
        name: "combat_boost".to_string(),
        description: "Increases combat effectiveness".to_string(),
        modifier_type: ModifierType::Multiplier,
        target_type: ModifierTarget::Combat,
        target_resource: None,
        stacking_group: None,
    };

    let result = diesel::insert_into(modifiers::table)
        .values(&combat_modifier)
        .execute(&mut conn);
    assert!(result.is_ok(), "Failed to insert valid combat modifier");
}

#[tokio::test]
async fn test_invalid_modifier_constraints() {
    let db_pool = common::init_server().db_pool;
    let mut conn = db_pool.get().unwrap();

    // Test resource modifier with null resource (should fail)
    let invalid_resource_modifier = NewModifier {
        name: "invalid_resource".to_string(),
        description: "Invalid resource modifier".to_string(),
        modifier_type: ModifierType::Percentage,
        target_type: ModifierTarget::Resource,
        target_resource: None,
        stacking_group: None,
    };

    let result = diesel::insert_into(modifiers::table)
        .values(&invalid_resource_modifier)
        .execute(&mut conn);
    assert!(
        result.is_err(),
        "Should fail: Resource modifier without resource type"
    );

    // Test combat modifier with resource type (should fail)
    let invalid_combat_modifier = NewModifier {
        name: "invalid_combat".to_string(),
        description: "Invalid combat modifier".to_string(),
        modifier_type: ModifierType::Multiplier,
        target_type: ModifierTarget::Combat,
        target_resource: Some(ResourceType::Wood),
        stacking_group: None,
    };

    let result = diesel::insert_into(modifiers::table)
        .values(&invalid_combat_modifier)
        .execute(&mut conn);
    assert!(
        result.is_err(),
        "Should fail: Combat modifier with resource type"
    );
}

#[tokio::test]
async fn test_unique_name_constraint() {
    let db_pool = common::init_server().db_pool;
    let mut conn = db_pool.get().unwrap();

    let modifier = NewModifier {
        name: "unique_test".to_string(),
        description: "Test modifier".to_string(),
        modifier_type: ModifierType::Percentage,
        target_type: ModifierTarget::Resource,
        target_resource: Some(ResourceType::Food),
        stacking_group: None,
    };

    // The first insert should succeed
    let result = diesel::insert_into(modifiers::table)
        .values(&modifier)
        .execute(&mut conn);
    assert!(result.is_ok(), "Failed to insert first modifier");

    // Second insert with same name should fail
    let duplicate_modifier = NewModifier {
        name: "unique_test".to_string(),
        description: "Different description".to_string(),
        modifier_type: ModifierType::Flat,
        target_type: ModifierTarget::Resource,
        target_resource: Some(ResourceType::Wood),
        stacking_group: None,
    };

    let result = diesel::insert_into(modifiers::table)
        .values(&duplicate_modifier)
        .execute(&mut conn);
    assert!(result.is_err(), "Should fail: Duplicate modifier name");
}
