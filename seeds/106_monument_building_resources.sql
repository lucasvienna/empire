-- Monument Building Resources Seed Data
-- Monuments don't produce resources or add caps
-- These entries prevent NULL constraint violations in the update_player_resource_caps_trigger
-- Building IDs: Monument (17, 34, 51, 68, 85)

INSERT INTO building_resource (building_id, building_level, food_cap, wood_cap, stone_cap, gold_cap)
VALUES
    -- ===== HUMAN FACTION =====
    -- Monument (building_id 17)
    (17, 0,  0, 0, 0, 0),
    (17, 1,  0, 0, 0, 0),
    (17, 2,  0, 0, 0, 0),
    (17, 3,  0, 0, 0, 0),
    (17, 4,  0, 0, 0, 0),
    (17, 5,  0, 0, 0, 0),
    (17, 6,  0, 0, 0, 0),
    (17, 7,  0, 0, 0, 0),
    (17, 8,  0, 0, 0, 0),
    (17, 9,  0, 0, 0, 0),
    (17, 10, 0, 0, 0, 0),

    -- ===== ORC FACTION =====
    -- Monument (building_id 34)
    (34, 0,  0, 0, 0, 0),
    (34, 1,  0, 0, 0, 0),
    (34, 2,  0, 0, 0, 0),
    (34, 3,  0, 0, 0, 0),
    (34, 4,  0, 0, 0, 0),
    (34, 5,  0, 0, 0, 0),
    (34, 6,  0, 0, 0, 0),
    (34, 7,  0, 0, 0, 0),
    (34, 8,  0, 0, 0, 0),
    (34, 9,  0, 0, 0, 0),
    (34, 10, 0, 0, 0, 0),

    -- ===== ELF FACTION =====
    -- Monument (building_id 51)
    (51, 0,  0, 0, 0, 0),
    (51, 1,  0, 0, 0, 0),
    (51, 2,  0, 0, 0, 0),
    (51, 3,  0, 0, 0, 0),
    (51, 4,  0, 0, 0, 0),
    (51, 5,  0, 0, 0, 0),
    (51, 6,  0, 0, 0, 0),
    (51, 7,  0, 0, 0, 0),
    (51, 8,  0, 0, 0, 0),
    (51, 9,  0, 0, 0, 0),
    (51, 10, 0, 0, 0, 0),

    -- ===== DWARF FACTION =====
    -- Monument (building_id 68)
    (68, 0,  0, 0, 0, 0),
    (68, 1,  0, 0, 0, 0),
    (68, 2,  0, 0, 0, 0),
    (68, 3,  0, 0, 0, 0),
    (68, 4,  0, 0, 0, 0),
    (68, 5,  0, 0, 0, 0),
    (68, 6,  0, 0, 0, 0),
    (68, 7,  0, 0, 0, 0),
    (68, 8,  0, 0, 0, 0),
    (68, 9,  0, 0, 0, 0),
    (68, 10, 0, 0, 0, 0),

    -- ===== GOBLIN FACTION =====
    -- Monument (building_id 85)
    (85, 0,  0, 0, 0, 0),
    (85, 1,  0, 0, 0, 0),
    (85, 2,  0, 0, 0, 0),
    (85, 3,  0, 0, 0, 0),
    (85, 4,  0, 0, 0, 0),
    (85, 5,  0, 0, 0, 0),
    (85, 6,  0, 0, 0, 0),
    (85, 7,  0, 0, 0, 0),
    (85, 8,  0, 0, 0, 0),
    (85, 9,  0, 0, 0, 0),
    (85, 10, 0, 0, 0, 0)
ON CONFLICT (building_id, building_level) DO NOTHING;
