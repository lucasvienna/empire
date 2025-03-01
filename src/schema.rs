// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "faction_code"))]
    pub struct FactionCode;
}

diesel::table! {
    building_levels (id) {
        id -> Uuid,
        building_id -> Int4,
        level -> Int4,
        upgrade_time -> Text,
        req_food -> Nullable<Int4>,
        req_wood -> Nullable<Int4>,
        req_stone -> Nullable<Int4>,
        req_gold -> Nullable<Int4>,
    }
}

diesel::table! {
    building_resources (id) {
        id -> Uuid,
        building_id -> Int4,
        building_level -> Int4,
        population -> Int4,
        food -> Int4,
        wood -> Int4,
        stone -> Int4,
        gold -> Int4,
        food_cap -> Int4,
        wood_cap -> Int4,
        stone_cap -> Int4,
        gold_cap -> Int4,
        food_acc_cap -> Int4,
        wood_acc_cap -> Int4,
        stone_acc_cap -> Int4,
        gold_acc_cap -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FactionCode;

    buildings (id) {
        id -> Int4,
        name -> Text,
        max_level -> Int4,
        max_count -> Int4,
        faction -> FactionCode,
        starter -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FactionCode;

    factions (id) {
        id -> FactionCode,
        name -> Text,
    }
}

diesel::table! {
    resources (user_id) {
        user_id -> Uuid,
        food -> Int4,
        wood -> Int4,
        stone -> Int4,
        gold -> Int4,
        food_cap -> Int4,
        wood_cap -> Int4,
        stone_cap -> Int4,
        gold_cap -> Int4,
    }
}

diesel::table! {
    resources_accumulator (user_id) {
        user_id -> Uuid,
        food -> Int4,
        wood -> Int4,
        stone -> Int4,
        gold -> Int4,
    }
}

diesel::table! {
    user_buildings (id) {
        id -> Uuid,
        user_id -> Uuid,
        building_id -> Int4,
        level -> Int4,
        upgrade_time -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FactionCode;

    users (id) {
        id -> Uuid,
        name -> Text,
        pwd_hash -> Text,
        #[max_length = 254]
        email -> Nullable<Varchar>,
        faction -> FactionCode,
    }
}

diesel::joinable!(building_levels -> buildings (building_id));
diesel::joinable!(building_resources -> buildings (building_id));
diesel::joinable!(buildings -> factions (faction));
diesel::joinable!(resources -> users (user_id));
diesel::joinable!(resources_accumulator -> users (user_id));
diesel::joinable!(user_buildings -> buildings (building_id));
diesel::joinable!(user_buildings -> users (user_id));
diesel::joinable!(users -> factions (faction));

diesel::allow_tables_to_appear_in_same_query!(
    building_levels,
    building_resources,
    buildings,
    factions,
    resources,
    resources_accumulator,
    user_buildings,
    users,
);
