// this isn't a table but instead a view. The problem is that diesel doesn't support views yet
diesel::table! {
    resource_generation (player_id) {
        player_id -> Uuid,
        population -> BigInt,
        food -> BigInt,
        wood -> BigInt,
        stone -> BigInt,
        gold -> BigInt,
        food_acc_cap -> BigInt,
        wood_acc_cap -> BigInt,
        stone_acc_cap -> BigInt,
        gold_acc_cap -> BigInt,
    }
}
