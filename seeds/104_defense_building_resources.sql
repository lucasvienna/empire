-- Defense Building Resources Seed Data
-- Walls don't produce resources or add caps
-- These entries prevent NULL constraint violations in the update_player_resource_caps_trigger
-- Building IDs: Walls (15, 32, 49, 66, 83)

INSERT INTO building_resource (building_id, building_level, food_cap, wood_cap, stone_cap, gold_cap)
VALUES
    -- ===== HUMAN FACTION =====
    -- Walls (building_id 15)
    (15, 0,  0, 0, 0, 0),
    (15, 1,  0, 0, 0, 0),
    (15, 2,  0, 0, 0, 0),
    (15, 3,  0, 0, 0, 0),
    (15, 4,  0, 0, 0, 0),
    (15, 5,  0, 0, 0, 0),
    (15, 6,  0, 0, 0, 0),
    (15, 7,  0, 0, 0, 0),
    (15, 8,  0, 0, 0, 0),
    (15, 9,  0, 0, 0, 0),
    (15, 10, 0, 0, 0, 0),

    -- ===== ORC FACTION =====
    -- Walls (building_id 32)
    (32, 0,  0, 0, 0, 0),
    (32, 1,  0, 0, 0, 0),
    (32, 2,  0, 0, 0, 0),
    (32, 3,  0, 0, 0, 0),
    (32, 4,  0, 0, 0, 0),
    (32, 5,  0, 0, 0, 0),
    (32, 6,  0, 0, 0, 0),
    (32, 7,  0, 0, 0, 0),
    (32, 8,  0, 0, 0, 0),
    (32, 9,  0, 0, 0, 0),
    (32, 10, 0, 0, 0, 0),

    -- ===== ELF FACTION =====
    -- Walls (building_id 49)
    (49, 0,  0, 0, 0, 0),
    (49, 1,  0, 0, 0, 0),
    (49, 2,  0, 0, 0, 0),
    (49, 3,  0, 0, 0, 0),
    (49, 4,  0, 0, 0, 0),
    (49, 5,  0, 0, 0, 0),
    (49, 6,  0, 0, 0, 0),
    (49, 7,  0, 0, 0, 0),
    (49, 8,  0, 0, 0, 0),
    (49, 9,  0, 0, 0, 0),
    (49, 10, 0, 0, 0, 0),

    -- ===== DWARF FACTION =====
    -- Walls (building_id 66)
    (66, 0,  0, 0, 0, 0),
    (66, 1,  0, 0, 0, 0),
    (66, 2,  0, 0, 0, 0),
    (66, 3,  0, 0, 0, 0),
    (66, 4,  0, 0, 0, 0),
    (66, 5,  0, 0, 0, 0),
    (66, 6,  0, 0, 0, 0),
    (66, 7,  0, 0, 0, 0),
    (66, 8,  0, 0, 0, 0),
    (66, 9,  0, 0, 0, 0),
    (66, 10, 0, 0, 0, 0),

    -- ===== GOBLIN FACTION =====
    -- Walls (building_id 83)
    (83, 0,  0, 0, 0, 0),
    (83, 1,  0, 0, 0, 0),
    (83, 2,  0, 0, 0, 0),
    (83, 3,  0, 0, 0, 0),
    (83, 4,  0, 0, 0, 0),
    (83, 5,  0, 0, 0, 0),
    (83, 6,  0, 0, 0, 0),
    (83, 7,  0, 0, 0, 0),
    (83, 8,  0, 0, 0, 0),
    (83, 9,  0, 0, 0, 0),
    (83, 10, 0, 0, 0, 0)
ON CONFLICT (building_id, building_level) DO NOTHING;
