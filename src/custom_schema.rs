// this isn't a table but instead a view. The problem is that diesel doesn't support views yet
diesel::table! {
    resource_generation (user_id) {
        user_id -> Uuid,
        population -> Int4,
        food -> Int4,
        wood -> Int4,
        stone -> Int4,
        gold -> Int4,
        food_acc_cap -> Int4,
        wood_acc_cap -> Int4,
        stone_acc_cap -> Int4,
        gold_acc_cap -> Int4,
    }
}
