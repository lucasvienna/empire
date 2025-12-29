-- Religion Building Levels Seed Data
-- Religion buildings: Church (Human), Shamanic Altar (Orc), Shrine (Elf), Temple (Dwarf), Speaker's Hut (Goblin)
-- These buildings provide faith-based bonuses and abilities
-- Cost pattern: balanced with emphasis on gold (offerings/decorations)

INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES
-- ===== HUMAN FACTION =====

-- Church (building_id 16)
(16, 0,  0,    0,    0,    0,    0   ), -- Church 0
(16, 1,  90,   200,  300,  300,  500 ), -- Church 1
(16, 2,  180,  400,  600,  600,  1000), -- Church 2
(16, 3,  270,  600,  900,  900,  1500), -- Church 3
(16, 4,  360,  800,  1200, 1200, 2000), -- Church 4
(16, 5,  450,  1000, 1500, 1500, 2500), -- Church 5
(16, 6,  600,  1200, 1800, 1800, 3000), -- Church 6
(16, 7,  780,  1400, 2100, 2100, 3500), -- Church 7
(16, 8,  1020, 1600, 2400, 2400, 4000), -- Church 8
(16, 9,  1320, 1800, 2700, 2700, 4500), -- Church 9
(16, 10, 1800, 2000, 3000, 3000, 5000), -- Church 10

-- ===== ORC FACTION =====

-- Shamanic Altar (building_id 33)
(33, 0,  0,    0,    0,    0,    0   ), -- Shamanic Altar 0
(33, 1,  90,   200,  300,  300,  500 ), -- Shamanic Altar 1
(33, 2,  180,  400,  600,  600,  1000), -- Shamanic Altar 2
(33, 3,  270,  600,  900,  900,  1500), -- Shamanic Altar 3
(33, 4,  360,  800,  1200, 1200, 2000), -- Shamanic Altar 4
(33, 5,  450,  1000, 1500, 1500, 2500), -- Shamanic Altar 5
(33, 6,  600,  1200, 1800, 1800, 3000), -- Shamanic Altar 6
(33, 7,  780,  1400, 2100, 2100, 3500), -- Shamanic Altar 7
(33, 8,  1020, 1600, 2400, 2400, 4000), -- Shamanic Altar 8
(33, 9,  1320, 1800, 2700, 2700, 4500), -- Shamanic Altar 9
(33, 10, 1800, 2000, 3000, 3000, 5000), -- Shamanic Altar 10

-- ===== ELF FACTION =====

-- Shrine (building_id 50)
(50, 0,  0,    0,    0,    0,    0   ), -- Shrine 0
(50, 1,  90,   200,  300,  300,  500 ), -- Shrine 1
(50, 2,  180,  400,  600,  600,  1000), -- Shrine 2
(50, 3,  270,  600,  900,  900,  1500), -- Shrine 3
(50, 4,  360,  800,  1200, 1200, 2000), -- Shrine 4
(50, 5,  450,  1000, 1500, 1500, 2500), -- Shrine 5
(50, 6,  600,  1200, 1800, 1800, 3000), -- Shrine 6
(50, 7,  780,  1400, 2100, 2100, 3500), -- Shrine 7
(50, 8,  1020, 1600, 2400, 2400, 4000), -- Shrine 8
(50, 9,  1320, 1800, 2700, 2700, 4500), -- Shrine 9
(50, 10, 1800, 2000, 3000, 3000, 5000), -- Shrine 10

-- ===== DWARF FACTION =====

-- Temple (building_id 67)
(67, 0,  0,    0,    0,    0,    0   ), -- Temple 0
(67, 1,  90,   200,  300,  300,  500 ), -- Temple 1
(67, 2,  180,  400,  600,  600,  1000), -- Temple 2
(67, 3,  270,  600,  900,  900,  1500), -- Temple 3
(67, 4,  360,  800,  1200, 1200, 2000), -- Temple 4
(67, 5,  450,  1000, 1500, 1500, 2500), -- Temple 5
(67, 6,  600,  1200, 1800, 1800, 3000), -- Temple 6
(67, 7,  780,  1400, 2100, 2100, 3500), -- Temple 7
(67, 8,  1020, 1600, 2400, 2400, 4000), -- Temple 8
(67, 9,  1320, 1800, 2700, 2700, 4500), -- Temple 9
(67, 10, 1800, 2000, 3000, 3000, 5000), -- Temple 10

-- ===== GOBLIN FACTION =====

-- Speaker's Hut (building_id 84)
(84, 0,  0,    0,    0,    0,    0   ), -- Speaker's Hut 0
(84, 1,  90,   200,  300,  300,  500 ), -- Speaker's Hut 1
(84, 2,  180,  400,  600,  600,  1000), -- Speaker's Hut 2
(84, 3,  270,  600,  900,  900,  1500), -- Speaker's Hut 3
(84, 4,  360,  800,  1200, 1200, 2000), -- Speaker's Hut 4
(84, 5,  450,  1000, 1500, 1500, 2500), -- Speaker's Hut 5
(84, 6,  600,  1200, 1800, 1800, 3000), -- Speaker's Hut 6
(84, 7,  780,  1400, 2100, 2100, 3500), -- Speaker's Hut 7
(84, 8,  1020, 1600, 2400, 2400, 4000), -- Speaker's Hut 8
(84, 9,  1320, 1800, 2700, 2700, 4500), -- Speaker's Hut 9
(84, 10, 1800, 2000, 3000, 3000, 5000)  -- Speaker's Hut 10
ON CONFLICT (building_id, level) DO NOTHING;
