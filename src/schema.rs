// @generated automatically by Diesel CLI.

diesel::table! {
    building_levels (id) {
        id -> Int4,
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
    buildings (id) {
        id -> Int4,
        name -> Text,
        max_level -> Int4,
        max_count -> Int4,
        faction -> Int4,
        starter -> Bool,
    }
}

diesel::table! {
    factions (id) {
        id -> Int4,
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
    users (id) {
        id -> Uuid,
        name -> Text,
        faction -> Int4,
        data -> Nullable<Jsonb>,
    }
}

diesel::joinable!(building_levels -> buildings (building_id));
diesel::joinable!(buildings -> factions (faction));
diesel::joinable!(resources -> users (user_id));
diesel::joinable!(user_buildings -> buildings (building_id));
diesel::joinable!(user_buildings -> users (user_id));
diesel::joinable!(users -> factions (faction));

diesel::allow_tables_to_appear_in_same_query!(
    building_levels,
    buildings,
    factions,
    resources,
    user_buildings,
    users,
);
