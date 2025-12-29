-- Neutral Building Resources Seed Data
-- Neutral buildings don't produce resources or add caps
-- These entries prevent NULL constraint violations in the update_player_resource_caps_trigger
-- Building IDs: Guild Hall (86), Market (87), Embassy (88)

INSERT INTO building_resource (building_id, building_level, food_cap, wood_cap, stone_cap, gold_cap)
VALUES
    -- Guild Hall (building_id 86)
    (86, 0,  0, 0, 0, 0),
    (86, 1,  0, 0, 0, 0),
    (86, 2,  0, 0, 0, 0),
    (86, 3,  0, 0, 0, 0),
    (86, 4,  0, 0, 0, 0),
    (86, 5,  0, 0, 0, 0),
    (86, 6,  0, 0, 0, 0),
    (86, 7,  0, 0, 0, 0),
    (86, 8,  0, 0, 0, 0),
    (86, 9,  0, 0, 0, 0),
    (86, 10, 0, 0, 0, 0),

    -- Market (building_id 87)
    (87, 0,  0, 0, 0, 0),
    (87, 1,  0, 0, 0, 0),
    (87, 2,  0, 0, 0, 0),
    (87, 3,  0, 0, 0, 0),
    (87, 4,  0, 0, 0, 0),
    (87, 5,  0, 0, 0, 0),
    (87, 6,  0, 0, 0, 0),
    (87, 7,  0, 0, 0, 0),
    (87, 8,  0, 0, 0, 0),
    (87, 9,  0, 0, 0, 0),
    (87, 10, 0, 0, 0, 0),

    -- Embassy (building_id 88)
    (88, 0,  0, 0, 0, 0),
    (88, 1,  0, 0, 0, 0),
    (88, 2,  0, 0, 0, 0),
    (88, 3,  0, 0, 0, 0),
    (88, 4,  0, 0, 0, 0),
    (88, 5,  0, 0, 0, 0),
    (88, 6,  0, 0, 0, 0),
    (88, 7,  0, 0, 0, 0),
    (88, 8,  0, 0, 0, 0),
    (88, 9,  0, 0, 0, 0),
    (88, 10, 0, 0, 0, 0)
ON CONFLICT (building_id, building_level) DO NOTHING;
