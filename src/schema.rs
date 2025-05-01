// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "faction_code"))]
    pub struct FactionCode;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "job_status"))]
    pub struct JobStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "job_type"))]
    pub struct JobType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "modifier_action_type"))]
    pub struct ModifierActionType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "modifier_source_type"))]
    pub struct ModifierSourceType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "modifier_target"))]
    pub struct ModifierTarget;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "modifier_type"))]
    pub struct ModifierType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "resource_type"))]
    pub struct ResourceType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "stacking_behaviour"))]
    pub struct StackingBehaviour;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ModifierSourceType;

    active_modifiers (id) {
        id -> Uuid,
        player_id -> Uuid,
        modifier_id -> Uuid,
        started_at -> Timestamptz,
        expires_at -> Nullable<Timestamptz>,
        source_type -> ModifierSourceType,
        source_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FactionCode;

    building (id) {
        id -> Int4,
        name -> Text,
        max_level -> Int4,
        max_count -> Int4,
        faction -> FactionCode,
        starter -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    building_level (id) {
        id -> Uuid,
        building_id -> Int4,
        level -> Int4,
        upgrade_time -> Text,
        req_food -> Nullable<Int8>,
        req_wood -> Nullable<Int8>,
        req_stone -> Nullable<Int8>,
        req_gold -> Nullable<Int8>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    building_resource (id) {
        id -> Uuid,
        building_id -> Int4,
        building_level -> Int4,
        population -> Int8,
        food -> Int8,
        wood -> Int8,
        stone -> Int8,
        gold -> Int8,
        food_cap -> Int8,
        wood_cap -> Int8,
        stone_cap -> Int8,
        gold_cap -> Int8,
        food_acc_cap -> Int8,
        wood_acc_cap -> Int8,
        stone_acc_cap -> Int8,
        gold_acc_cap -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FactionCode;

    faction (id) {
        id -> FactionCode,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::JobType;
    use super::sql_types::JobStatus;

    job (id) {
        id -> Uuid,
        job_type -> JobType,
        status -> JobStatus,
        payload -> Jsonb,
        run_at -> Timestamptz,
        last_error -> Nullable<Text>,
        retries -> Int4,
        max_retries -> Int4,
        priority -> Int4,
        timeout_seconds -> Int4,
        locked_at -> Nullable<Timestamptz>,
        locked_by -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ModifierActionType;
    use super::sql_types::ModifierSourceType;

    modifier_history (id) {
        id -> Uuid,
        player_id -> Uuid,
        modifier_id -> Uuid,
        action_type -> ModifierActionType,
        occurred_at -> Timestamptz,
        magnitude -> Numeric,
        source_type -> ModifierSourceType,
        source_id -> Nullable<Uuid>,
        previous_state -> Nullable<Jsonb>,
        reason -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ModifierType;
    use super::sql_types::ModifierTarget;
    use super::sql_types::ResourceType;
    use super::sql_types::StackingBehaviour;

    modifiers (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        modifier_type -> ModifierType,
        magnitude -> Numeric,
        target_type -> ModifierTarget,
        target_resource -> Nullable<ResourceType>,
        stacking_behaviour -> StackingBehaviour,
        stacking_group -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FactionCode;

    player (id) {
        id -> Uuid,
        name -> Text,
        pwd_hash -> Text,
        #[max_length = 254]
        email -> Nullable<Varchar>,
        faction -> FactionCode,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    player_accumulator (id) {
        id -> Uuid,
        player_id -> Uuid,
        food -> Int8,
        wood -> Int8,
        stone -> Int8,
        gold -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    player_building (id) {
        id -> Uuid,
        player_id -> Uuid,
        building_id -> Int4,
        level -> Int4,
        upgrade_time -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    player_resource (id) {
        id -> Uuid,
        player_id -> Uuid,
        food -> Int8,
        wood -> Int8,
        stone -> Int8,
        gold -> Int8,
        food_cap -> Int8,
        wood_cap -> Int8,
        stone_cap -> Int8,
        gold_cap -> Int8,
        produced_at -> Timestamptz,
        collected_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(active_modifiers -> modifiers (modifier_id));
diesel::joinable!(active_modifiers -> player (player_id));
diesel::joinable!(building -> faction (faction));
diesel::joinable!(building_level -> building (building_id));
diesel::joinable!(building_resource -> building (building_id));
diesel::joinable!(modifier_history -> modifiers (modifier_id));
diesel::joinable!(modifier_history -> player (player_id));
diesel::joinable!(player -> faction (faction));
diesel::joinable!(player_accumulator -> player (player_id));
diesel::joinable!(player_building -> building (building_id));
diesel::joinable!(player_building -> player (player_id));
diesel::joinable!(player_resource -> player (player_id));

diesel::allow_tables_to_appear_in_same_query!(
    active_modifiers,
    building,
    building_level,
    building_resource,
    faction,
    job,
    modifier_history,
    modifiers,
    player,
    player_accumulator,
    player_building,
    player_resource,
);
