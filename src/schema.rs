// @generated automatically by Diesel CLI.

diesel::table! {
    building_levels (id) {
        id -> Integer,
        building_id -> Integer,
        level -> Integer,
        upgrade_time -> Text,
        req_food -> Nullable<Integer>,
        req_wood -> Nullable<Integer>,
        req_stone -> Nullable<Integer>,
        req_gold -> Nullable<Integer>,
    }
}

diesel::table! {
    buildings (id) {
        id -> Integer,
        name -> Text,
        max_level -> Integer,
        max_count -> Integer,
        faction -> Integer,
    }
}

diesel::table! {
    factions (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    resources (user_id) {
        user_id -> Integer,
        food -> Integer,
        wood -> Integer,
        stone -> Integer,
        gold -> Integer,
    }
}

diesel::table! {
    user_buildings (id) {
        id -> Integer,
        user_id -> Integer,
        building_id -> Integer,
        level -> Integer,
        upgrade_time -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        faction -> Integer,
        data -> Nullable<Binary>,
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
