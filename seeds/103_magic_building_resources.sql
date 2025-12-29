-- Magic Building Resources Seed Data
-- Magic buildings don't produce resources or add caps
-- These entries prevent NULL constraint violations in the update_player_resource_caps_trigger
-- Building IDs: Mage Tower (14), The Circle (31), Arcanum (48), Hall of Runes (65), Mana Den (82)

INSERT INTO building_resource (building_id, building_level, food_cap, wood_cap, stone_cap, gold_cap)
VALUES
    -- ===== HUMAN FACTION =====
    -- Mage Tower (building_id 14)
    (14, 0,  0, 0, 0, 0),
    (14, 1,  0, 0, 0, 0),
    (14, 2,  0, 0, 0, 0),
    (14, 3,  0, 0, 0, 0),
    (14, 4,  0, 0, 0, 0),
    (14, 5,  0, 0, 0, 0),
    (14, 6,  0, 0, 0, 0),
    (14, 7,  0, 0, 0, 0),
    (14, 8,  0, 0, 0, 0),
    (14, 9,  0, 0, 0, 0),
    (14, 10, 0, 0, 0, 0),

    -- ===== ORC FACTION =====
    -- The Circle (building_id 31)
    (31, 0,  0, 0, 0, 0),
    (31, 1,  0, 0, 0, 0),
    (31, 2,  0, 0, 0, 0),
    (31, 3,  0, 0, 0, 0),
    (31, 4,  0, 0, 0, 0),
    (31, 5,  0, 0, 0, 0),
    (31, 6,  0, 0, 0, 0),
    (31, 7,  0, 0, 0, 0),
    (31, 8,  0, 0, 0, 0),
    (31, 9,  0, 0, 0, 0),
    (31, 10, 0, 0, 0, 0),

    -- ===== ELF FACTION =====
    -- Arcanum (building_id 48)
    (48, 0,  0, 0, 0, 0),
    (48, 1,  0, 0, 0, 0),
    (48, 2,  0, 0, 0, 0),
    (48, 3,  0, 0, 0, 0),
    (48, 4,  0, 0, 0, 0),
    (48, 5,  0, 0, 0, 0),
    (48, 6,  0, 0, 0, 0),
    (48, 7,  0, 0, 0, 0),
    (48, 8,  0, 0, 0, 0),
    (48, 9,  0, 0, 0, 0),
    (48, 10, 0, 0, 0, 0),

    -- ===== DWARF FACTION =====
    -- Hall of Runes (building_id 65)
    (65, 0,  0, 0, 0, 0),
    (65, 1,  0, 0, 0, 0),
    (65, 2,  0, 0, 0, 0),
    (65, 3,  0, 0, 0, 0),
    (65, 4,  0, 0, 0, 0),
    (65, 5,  0, 0, 0, 0),
    (65, 6,  0, 0, 0, 0),
    (65, 7,  0, 0, 0, 0),
    (65, 8,  0, 0, 0, 0),
    (65, 9,  0, 0, 0, 0),
    (65, 10, 0, 0, 0, 0),

    -- ===== GOBLIN FACTION =====
    -- Mana Den (building_id 82)
    (82, 0,  0, 0, 0, 0),
    (82, 1,  0, 0, 0, 0),
    (82, 2,  0, 0, 0, 0),
    (82, 3,  0, 0, 0, 0),
    (82, 4,  0, 0, 0, 0),
    (82, 5,  0, 0, 0, 0),
    (82, 6,  0, 0, 0, 0),
    (82, 7,  0, 0, 0, 0),
    (82, 8,  0, 0, 0, 0),
    (82, 9,  0, 0, 0, 0),
    (82, 10, 0, 0, 0, 0)
ON CONFLICT (building_id, building_level) DO NOTHING;
