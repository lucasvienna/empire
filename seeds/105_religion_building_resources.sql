-- Religion Building Resources Seed Data
-- Religion buildings don't produce resources or add caps
-- These entries prevent NULL constraint violations in the update_player_resource_caps_trigger
-- Building IDs: Church (16), Shamanic Altar (33), Shrine (50), Temple (67), Speaker's Hut (84)

INSERT INTO building_resource (building_id, building_level, food_cap, wood_cap, stone_cap, gold_cap)
VALUES
    -- ===== HUMAN FACTION =====
    -- Church (building_id 16)
    (16, 0,  0, 0, 0, 0),
    (16, 1,  0, 0, 0, 0),
    (16, 2,  0, 0, 0, 0),
    (16, 3,  0, 0, 0, 0),
    (16, 4,  0, 0, 0, 0),
    (16, 5,  0, 0, 0, 0),
    (16, 6,  0, 0, 0, 0),
    (16, 7,  0, 0, 0, 0),
    (16, 8,  0, 0, 0, 0),
    (16, 9,  0, 0, 0, 0),
    (16, 10, 0, 0, 0, 0),

    -- ===== ORC FACTION =====
    -- Shamanic Altar (building_id 33)
    (33, 0,  0, 0, 0, 0),
    (33, 1,  0, 0, 0, 0),
    (33, 2,  0, 0, 0, 0),
    (33, 3,  0, 0, 0, 0),
    (33, 4,  0, 0, 0, 0),
    (33, 5,  0, 0, 0, 0),
    (33, 6,  0, 0, 0, 0),
    (33, 7,  0, 0, 0, 0),
    (33, 8,  0, 0, 0, 0),
    (33, 9,  0, 0, 0, 0),
    (33, 10, 0, 0, 0, 0),

    -- ===== ELF FACTION =====
    -- Shrine (building_id 50)
    (50, 0,  0, 0, 0, 0),
    (50, 1,  0, 0, 0, 0),
    (50, 2,  0, 0, 0, 0),
    (50, 3,  0, 0, 0, 0),
    (50, 4,  0, 0, 0, 0),
    (50, 5,  0, 0, 0, 0),
    (50, 6,  0, 0, 0, 0),
    (50, 7,  0, 0, 0, 0),
    (50, 8,  0, 0, 0, 0),
    (50, 9,  0, 0, 0, 0),
    (50, 10, 0, 0, 0, 0),

    -- ===== DWARF FACTION =====
    -- Temple (building_id 67)
    (67, 0,  0, 0, 0, 0),
    (67, 1,  0, 0, 0, 0),
    (67, 2,  0, 0, 0, 0),
    (67, 3,  0, 0, 0, 0),
    (67, 4,  0, 0, 0, 0),
    (67, 5,  0, 0, 0, 0),
    (67, 6,  0, 0, 0, 0),
    (67, 7,  0, 0, 0, 0),
    (67, 8,  0, 0, 0, 0),
    (67, 9,  0, 0, 0, 0),
    (67, 10, 0, 0, 0, 0),

    -- ===== GOBLIN FACTION =====
    -- Speaker's Hut (building_id 84)
    (84, 0,  0, 0, 0, 0),
    (84, 1,  0, 0, 0, 0),
    (84, 2,  0, 0, 0, 0),
    (84, 3,  0, 0, 0, 0),
    (84, 4,  0, 0, 0, 0),
    (84, 5,  0, 0, 0, 0),
    (84, 6,  0, 0, 0, 0),
    (84, 7,  0, 0, 0, 0),
    (84, 8,  0, 0, 0, 0),
    (84, 9,  0, 0, 0, 0),
    (84, 10, 0, 0, 0, 0)
ON CONFLICT (building_id, building_level) DO NOTHING;
