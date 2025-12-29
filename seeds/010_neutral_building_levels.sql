-- Neutral Building Levels Seed Data
-- Neutral buildings: Guild Hall (86), Market (87), Embassy (88)
-- These buildings are shared across all factions
-- Cost pattern: balanced, gold-focused (commerce and diplomacy)

INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES
-- Guild Hall (building_id 86) - Unlocks guild features
(86, 0,  0,    0,    0,    0,    0   ), -- Guild Hall 0
(86, 1,  90,   300,  300,  300,  600 ), -- Guild Hall 1
(86, 2,  180,  600,  600,  600,  1200), -- Guild Hall 2
(86, 3,  270,  900,  900,  900,  1800), -- Guild Hall 3
(86, 4,  360,  1200, 1200, 1200, 2400), -- Guild Hall 4
(86, 5,  450,  1500, 1500, 1500, 3000), -- Guild Hall 5
(86, 6,  600,  1800, 1800, 1800, 3600), -- Guild Hall 6
(86, 7,  780,  2100, 2100, 2100, 4200), -- Guild Hall 7
(86, 8,  1020, 2400, 2400, 2400, 4800), -- Guild Hall 8
(86, 9,  1320, 2700, 2700, 2700, 5400), -- Guild Hall 9
(86, 10, 1800, 3000, 3000, 3000, 6000), -- Guild Hall 10

-- Market (building_id 87) - Enables trading
(87, 0,  0,    0,    0,    0,    0   ), -- Market 0
(87, 1,  60,   200,  200,  100,  400 ), -- Market 1
(87, 2,  120,  400,  400,  200,  800 ), -- Market 2
(87, 3,  180,  600,  600,  300,  1200), -- Market 3
(87, 4,  240,  800,  800,  400,  1600), -- Market 4
(87, 5,  300,  1000, 1000, 500,  2000), -- Market 5
(87, 6,  360,  1200, 1200, 600,  2400), -- Market 6
(87, 7,  420,  1400, 1400, 700,  2800), -- Market 7
(87, 8,  480,  1600, 1600, 800,  3200), -- Market 8
(87, 9,  540,  1800, 1800, 900,  3600), -- Market 9
(87, 10, 600,  2000, 2000, 1000, 4000), -- Market 10

-- Embassy (building_id 88) - Diplomatic relations
(88, 0,  0,    0,    0,    0,    0   ), -- Embassy 0
(88, 1,  90,   200,  400,  400,  800 ), -- Embassy 1
(88, 2,  180,  400,  800,  800,  1600), -- Embassy 2
(88, 3,  270,  600,  1200, 1200, 2400), -- Embassy 3
(88, 4,  360,  800,  1600, 1600, 3200), -- Embassy 4
(88, 5,  450,  1000, 2000, 2000, 4000), -- Embassy 5
(88, 6,  600,  1200, 2400, 2400, 4800), -- Embassy 6
(88, 7,  780,  1400, 2800, 2800, 5600), -- Embassy 7
(88, 8,  1020, 1600, 3200, 3200, 6400), -- Embassy 8
(88, 9,  1320, 1800, 3600, 3600, 7200), -- Embassy 9
(88, 10, 1800, 2000, 4000, 4000, 8000)  -- Embassy 10
ON CONFLICT (building_id, level) DO NOTHING;
