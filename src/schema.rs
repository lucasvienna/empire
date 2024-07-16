// @generated automatically by Diesel CLI.

diesel::table! {
    buildings (id) {
        id -> Integer,
        name -> Text,
        max_level -> Integer,
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
    resources (id) {
        id -> Integer,
        user -> Integer,
        food -> Integer,
        wood -> Integer,
        stone -> Integer,
        gold -> Integer,
    }
}

diesel::table! {
    user_buildings (user, building) {
        user -> Integer,
        building -> Integer,
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

diesel::joinable!(buildings -> factions (faction));
diesel::joinable!(resources -> users (user));
diesel::joinable!(user_buildings -> buildings (building));
diesel::joinable!(user_buildings -> users (user));
diesel::joinable!(users -> factions (faction));

diesel::allow_tables_to_appear_in_same_query!(
    buildings,
    factions,
    resources,
    user_buildings,
    users,
);
