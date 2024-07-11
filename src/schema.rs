// @generated automatically by Diesel CLI.

diesel::table! {
    buildings (id) {
        id -> Integer,
        name -> Text,
        max_level -> Integer,
        faction -> Nullable<Text>,
    }
}

diesel::table! {
    factions (id) {
        id -> Text,
        name -> Text,
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
        faction -> Text,
        data -> Nullable<Binary>,
    }
}

diesel::joinable!(user_buildings -> buildings (building));
diesel::joinable!(user_buildings -> users (user));
diesel::joinable!(users -> factions (faction));

diesel::allow_tables_to_appear_in_same_query!(buildings, factions, user_buildings, users,);
