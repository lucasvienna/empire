-- Research Building Levels Seed Data
-- Research buildings: Academy (military), University (economic/science), Laboratory (magic)
-- These buildings unlock tech trees and research capabilities
-- Cost ~70% of Keep with polynomial scaling for balanced progression

-- ===== HUMAN FACTION (building_id 1-17) =====

-- Academy (building_id 7)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (7, 0,  0,    0,     0,     0,     0    ), -- Academy 0
       (7, 1,  120,  1000,  1000,  700,   700  ), -- Academy 1
       (7, 2,  240,  1700,  1700,  1200,  1200 ), -- Academy 2
       (7, 3,  360,  2700,  2700,  1900,  1900 ), -- Academy 3
       (7, 4,  480,  4000,  4000,  2800,  2800 ), -- Academy 4
       (7, 5,  600,  5600,  5600,  3900,  3900 ), -- Academy 5
       (7, 6,  840,  7500,  7500,  5200,  5200 ), -- Academy 6
       (7, 7,  1080, 9700,  9700,  6800,  6800 ), -- Academy 7
       (7, 8,  1440, 12200, 12200, 8500,  8500 ), -- Academy 8
       (7, 9,  1800, 15000, 15000, 10500, 10500), -- Academy 9
       (7, 10, 2400, 18100, 18100, 12700, 12700)  -- Academy 10
ON CONFLICT (building_id, level) DO NOTHING;

-- University (building_id 8)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (8, 0,  0,    0,     0,     0,     0    ), -- University 0
       (8, 1,  120,  1000,  1000,  700,   700  ), -- University 1
       (8, 2,  240,  1700,  1700,  1200,  1200 ), -- University 2
       (8, 3,  360,  2700,  2700,  1900,  1900 ), -- University 3
       (8, 4,  480,  4000,  4000,  2800,  2800 ), -- University 4
       (8, 5,  600,  5600,  5600,  3900,  3900 ), -- University 5
       (8, 6,  840,  7500,  7500,  5200,  5200 ), -- University 6
       (8, 7,  1080, 9700,  9700,  6800,  6800 ), -- University 7
       (8, 8,  1440, 12200, 12200, 8500,  8500 ), -- University 8
       (8, 9,  1800, 15000, 15000, 10500, 10500), -- University 9
       (8, 10, 2400, 18100, 18100, 12700, 12700)  -- University 10
ON CONFLICT (building_id, level) DO NOTHING;

-- Laboratory (building_id 9)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (9, 0,  0,    0,     0,     0,     0    ), -- Laboratory 0
       (9, 1,  120,  1000,  1000,  700,   700  ), -- Laboratory 1
       (9, 2,  240,  1700,  1700,  1200,  1200 ), -- Laboratory 2
       (9, 3,  360,  2700,  2700,  1900,  1900 ), -- Laboratory 3
       (9, 4,  480,  4000,  4000,  2800,  2800 ), -- Laboratory 4
       (9, 5,  600,  5600,  5600,  3900,  3900 ), -- Laboratory 5
       (9, 6,  840,  7500,  7500,  5200,  5200 ), -- Laboratory 6
       (9, 7,  1080, 9700,  9700,  6800,  6800 ), -- Laboratory 7
       (9, 8,  1440, 12200, 12200, 8500,  8500 ), -- Laboratory 8
       (9, 9,  1800, 15000, 15000, 10500, 10500), -- Laboratory 9
       (9, 10, 2400, 18100, 18100, 12700, 12700)  -- Laboratory 10
ON CONFLICT (building_id, level) DO NOTHING;

-- ===== ORC FACTION (building_id 18-34) =====

-- Academy (building_id 24)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (24, 0,  0,    0,     0,     0,     0    ), -- Academy 0
       (24, 1,  120,  1000,  1000,  700,   700  ), -- Academy 1
       (24, 2,  240,  1700,  1700,  1200,  1200 ), -- Academy 2
       (24, 3,  360,  2700,  2700,  1900,  1900 ), -- Academy 3
       (24, 4,  480,  4000,  4000,  2800,  2800 ), -- Academy 4
       (24, 5,  600,  5600,  5600,  3900,  3900 ), -- Academy 5
       (24, 6,  840,  7500,  7500,  5200,  5200 ), -- Academy 6
       (24, 7,  1080, 9700,  9700,  6800,  6800 ), -- Academy 7
       (24, 8,  1440, 12200, 12200, 8500,  8500 ), -- Academy 8
       (24, 9,  1800, 15000, 15000, 10500, 10500), -- Academy 9
       (24, 10, 2400, 18100, 18100, 12700, 12700)  -- Academy 10
ON CONFLICT (building_id, level) DO NOTHING;

-- University (building_id 25)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (25, 0,  0,    0,     0,     0,     0    ), -- University 0
       (25, 1,  120,  1000,  1000,  700,   700  ), -- University 1
       (25, 2,  240,  1700,  1700,  1200,  1200 ), -- University 2
       (25, 3,  360,  2700,  2700,  1900,  1900 ), -- University 3
       (25, 4,  480,  4000,  4000,  2800,  2800 ), -- University 4
       (25, 5,  600,  5600,  5600,  3900,  3900 ), -- University 5
       (25, 6,  840,  7500,  7500,  5200,  5200 ), -- University 6
       (25, 7,  1080, 9700,  9700,  6800,  6800 ), -- University 7
       (25, 8,  1440, 12200, 12200, 8500,  8500 ), -- University 8
       (25, 9,  1800, 15000, 15000, 10500, 10500), -- University 9
       (25, 10, 2400, 18100, 18100, 12700, 12700)  -- University 10
ON CONFLICT (building_id, level) DO NOTHING;

-- Laboratory (building_id 26)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (26, 0,  0,    0,     0,     0,     0    ), -- Laboratory 0
       (26, 1,  120,  1000,  1000,  700,   700  ), -- Laboratory 1
       (26, 2,  240,  1700,  1700,  1200,  1200 ), -- Laboratory 2
       (26, 3,  360,  2700,  2700,  1900,  1900 ), -- Laboratory 3
       (26, 4,  480,  4000,  4000,  2800,  2800 ), -- Laboratory 4
       (26, 5,  600,  5600,  5600,  3900,  3900 ), -- Laboratory 5
       (26, 6,  840,  7500,  7500,  5200,  5200 ), -- Laboratory 6
       (26, 7,  1080, 9700,  9700,  6800,  6800 ), -- Laboratory 7
       (26, 8,  1440, 12200, 12200, 8500,  8500 ), -- Laboratory 8
       (26, 9,  1800, 15000, 15000, 10500, 10500), -- Laboratory 9
       (26, 10, 2400, 18100, 18100, 12700, 12700)  -- Laboratory 10
ON CONFLICT (building_id, level) DO NOTHING;

-- ===== ELF FACTION (building_id 35-51) =====

-- Academy (building_id 41)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (41, 0,  0,    0,     0,     0,     0    ), -- Academy 0
       (41, 1,  120,  1000,  1000,  700,   700  ), -- Academy 1
       (41, 2,  240,  1700,  1700,  1200,  1200 ), -- Academy 2
       (41, 3,  360,  2700,  2700,  1900,  1900 ), -- Academy 3
       (41, 4,  480,  4000,  4000,  2800,  2800 ), -- Academy 4
       (41, 5,  600,  5600,  5600,  3900,  3900 ), -- Academy 5
       (41, 6,  840,  7500,  7500,  5200,  5200 ), -- Academy 6
       (41, 7,  1080, 9700,  9700,  6800,  6800 ), -- Academy 7
       (41, 8,  1440, 12200, 12200, 8500,  8500 ), -- Academy 8
       (41, 9,  1800, 15000, 15000, 10500, 10500), -- Academy 9
       (41, 10, 2400, 18100, 18100, 12700, 12700)  -- Academy 10
ON CONFLICT (building_id, level) DO NOTHING;

-- University (building_id 42)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (42, 0,  0,    0,     0,     0,     0    ), -- University 0
       (42, 1,  120,  1000,  1000,  700,   700  ), -- University 1
       (42, 2,  240,  1700,  1700,  1200,  1200 ), -- University 2
       (42, 3,  360,  2700,  2700,  1900,  1900 ), -- University 3
       (42, 4,  480,  4000,  4000,  2800,  2800 ), -- University 4
       (42, 5,  600,  5600,  5600,  3900,  3900 ), -- University 5
       (42, 6,  840,  7500,  7500,  5200,  5200 ), -- University 6
       (42, 7,  1080, 9700,  9700,  6800,  6800 ), -- University 7
       (42, 8,  1440, 12200, 12200, 8500,  8500 ), -- University 8
       (42, 9,  1800, 15000, 15000, 10500, 10500), -- University 9
       (42, 10, 2400, 18100, 18100, 12700, 12700)  -- University 10
ON CONFLICT (building_id, level) DO NOTHING;

-- Laboratory (building_id 43)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (43, 0,  0,    0,     0,     0,     0    ), -- Laboratory 0
       (43, 1,  120,  1000,  1000,  700,   700  ), -- Laboratory 1
       (43, 2,  240,  1700,  1700,  1200,  1200 ), -- Laboratory 2
       (43, 3,  360,  2700,  2700,  1900,  1900 ), -- Laboratory 3
       (43, 4,  480,  4000,  4000,  2800,  2800 ), -- Laboratory 4
       (43, 5,  600,  5600,  5600,  3900,  3900 ), -- Laboratory 5
       (43, 6,  840,  7500,  7500,  5200,  5200 ), -- Laboratory 6
       (43, 7,  1080, 9700,  9700,  6800,  6800 ), -- Laboratory 7
       (43, 8,  1440, 12200, 12200, 8500,  8500 ), -- Laboratory 8
       (43, 9,  1800, 15000, 15000, 10500, 10500), -- Laboratory 9
       (43, 10, 2400, 18100, 18100, 12700, 12700)  -- Laboratory 10
ON CONFLICT (building_id, level) DO NOTHING;

-- ===== DWARF FACTION (building_id 52-68) =====

-- Academy (building_id 58)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (58, 0,  0,    0,     0,     0,     0    ), -- Academy 0
       (58, 1,  120,  1000,  1000,  700,   700  ), -- Academy 1
       (58, 2,  240,  1700,  1700,  1200,  1200 ), -- Academy 2
       (58, 3,  360,  2700,  2700,  1900,  1900 ), -- Academy 3
       (58, 4,  480,  4000,  4000,  2800,  2800 ), -- Academy 4
       (58, 5,  600,  5600,  5600,  3900,  3900 ), -- Academy 5
       (58, 6,  840,  7500,  7500,  5200,  5200 ), -- Academy 6
       (58, 7,  1080, 9700,  9700,  6800,  6800 ), -- Academy 7
       (58, 8,  1440, 12200, 12200, 8500,  8500 ), -- Academy 8
       (58, 9,  1800, 15000, 15000, 10500, 10500), -- Academy 9
       (58, 10, 2400, 18100, 18100, 12700, 12700)  -- Academy 10
ON CONFLICT (building_id, level) DO NOTHING;

-- University (building_id 59)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (59, 0,  0,    0,     0,     0,     0    ), -- University 0
       (59, 1,  120,  1000,  1000,  700,   700  ), -- University 1
       (59, 2,  240,  1700,  1700,  1200,  1200 ), -- University 2
       (59, 3,  360,  2700,  2700,  1900,  1900 ), -- University 3
       (59, 4,  480,  4000,  4000,  2800,  2800 ), -- University 4
       (59, 5,  600,  5600,  5600,  3900,  3900 ), -- University 5
       (59, 6,  840,  7500,  7500,  5200,  5200 ), -- University 6
       (59, 7,  1080, 9700,  9700,  6800,  6800 ), -- University 7
       (59, 8,  1440, 12200, 12200, 8500,  8500 ), -- University 8
       (59, 9,  1800, 15000, 15000, 10500, 10500), -- University 9
       (59, 10, 2400, 18100, 18100, 12700, 12700)  -- University 10
ON CONFLICT (building_id, level) DO NOTHING;

-- Laboratory (building_id 60)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (60, 0,  0,    0,     0,     0,     0    ), -- Laboratory 0
       (60, 1,  120,  1000,  1000,  700,   700  ), -- Laboratory 1
       (60, 2,  240,  1700,  1700,  1200,  1200 ), -- Laboratory 2
       (60, 3,  360,  2700,  2700,  1900,  1900 ), -- Laboratory 3
       (60, 4,  480,  4000,  4000,  2800,  2800 ), -- Laboratory 4
       (60, 5,  600,  5600,  5600,  3900,  3900 ), -- Laboratory 5
       (60, 6,  840,  7500,  7500,  5200,  5200 ), -- Laboratory 6
       (60, 7,  1080, 9700,  9700,  6800,  6800 ), -- Laboratory 7
       (60, 8,  1440, 12200, 12200, 8500,  8500 ), -- Laboratory 8
       (60, 9,  1800, 15000, 15000, 10500, 10500), -- Laboratory 9
       (60, 10, 2400, 18100, 18100, 12700, 12700)  -- Laboratory 10
ON CONFLICT (building_id, level) DO NOTHING;

-- ===== GOBLIN FACTION (building_id 69-85) =====

-- Cadet School (building_id 75 - Goblin Academy equivalent)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (75, 0,  0,    0,     0,     0,     0    ), -- Cadet School 0
       (75, 1,  120,  1000,  1000,  700,   700  ), -- Cadet School 1
       (75, 2,  240,  1700,  1700,  1200,  1200 ), -- Cadet School 2
       (75, 3,  360,  2700,  2700,  1900,  1900 ), -- Cadet School 3
       (75, 4,  480,  4000,  4000,  2800,  2800 ), -- Cadet School 4
       (75, 5,  600,  5600,  5600,  3900,  3900 ), -- Cadet School 5
       (75, 6,  840,  7500,  7500,  5200,  5200 ), -- Cadet School 6
       (75, 7,  1080, 9700,  9700,  6800,  6800 ), -- Cadet School 7
       (75, 8,  1440, 12200, 12200, 8500,  8500 ), -- Cadet School 8
       (75, 9,  1800, 15000, 15000, 10500, 10500), -- Cadet School 9
       (75, 10, 2400, 18100, 18100, 12700, 12700)  -- Cadet School 10
ON CONFLICT (building_id, level) DO NOTHING;

-- Brainery (building_id 76 - Goblin University equivalent)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (76, 0,  0,    0,     0,     0,     0    ), -- Brainery 0
       (76, 1,  120,  1000,  1000,  700,   700  ), -- Brainery 1
       (76, 2,  240,  1700,  1700,  1200,  1200 ), -- Brainery 2
       (76, 3,  360,  2700,  2700,  1900,  1900 ), -- Brainery 3
       (76, 4,  480,  4000,  4000,  2800,  2800 ), -- Brainery 4
       (76, 5,  600,  5600,  5600,  3900,  3900 ), -- Brainery 5
       (76, 6,  840,  7500,  7500,  5200,  5200 ), -- Brainery 6
       (76, 7,  1080, 9700,  9700,  6800,  6800 ), -- Brainery 7
       (76, 8,  1440, 12200, 12200, 8500,  8500 ), -- Brainery 8
       (76, 9,  1800, 15000, 15000, 10500, 10500), -- Brainery 9
       (76, 10, 2400, 18100, 18100, 12700, 12700)  -- Brainery 10
ON CONFLICT (building_id, level) DO NOTHING;

-- Laboratory (building_id 77)
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES (77, 0,  0,    0,     0,     0,     0    ), -- Laboratory 0
       (77, 1,  120,  1000,  1000,  700,   700  ), -- Laboratory 1
       (77, 2,  240,  1700,  1700,  1200,  1200 ), -- Laboratory 2
       (77, 3,  360,  2700,  2700,  1900,  1900 ), -- Laboratory 3
       (77, 4,  480,  4000,  4000,  2800,  2800 ), -- Laboratory 4
       (77, 5,  600,  5600,  5600,  3900,  3900 ), -- Laboratory 5
       (77, 6,  840,  7500,  7500,  5200,  5200 ), -- Laboratory 6
       (77, 7,  1080, 9700,  9700,  6800,  6800 ), -- Laboratory 7
       (77, 8,  1440, 12200, 12200, 8500,  8500 ), -- Laboratory 8
       (77, 9,  1800, 15000, 15000, 10500, 10500), -- Laboratory 9
       (77, 10, 2400, 18100, 18100, 12700, 12700)  -- Laboratory 10
ON CONFLICT (building_id, level) DO NOTHING;
