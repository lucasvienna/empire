-- Magic Building Levels Seed Data
-- Magic buildings: Mage Tower (Human), The Circle (Orc), Arcanum (Elf), Hall of Runes (Dwarf), Mana Den (Goblin)
-- These buildings unlock magical abilities and spells
-- Cost pattern: balanced with emphasis on gold and stone (magical materials)

INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES
-- ===== HUMAN FACTION =====

-- Mage Tower (building_id 14)
(14, 0,  0,    0,    0,    0,     0    ), -- Mage Tower 0
(14, 1,  120,  500,  500,  1000,  1000 ), -- Mage Tower 1
(14, 2,  240,  1000, 1000, 2000,  2000 ), -- Mage Tower 2
(14, 3,  360,  1500, 1500, 3000,  3000 ), -- Mage Tower 3
(14, 4,  480,  2000, 2000, 4000,  4000 ), -- Mage Tower 4
(14, 5,  600,  2500, 2500, 5000,  5000 ), -- Mage Tower 5
(14, 6,  840,  3000, 3000, 6000,  6000 ), -- Mage Tower 6
(14, 7,  1080, 3500, 3500, 7000,  7000 ), -- Mage Tower 7
(14, 8,  1440, 4000, 4000, 8000,  8000 ), -- Mage Tower 8
(14, 9,  1800, 4500, 4500, 9000,  9000 ), -- Mage Tower 9
(14, 10, 2400, 5000, 5000, 10000, 10000), -- Mage Tower 10

-- ===== ORC FACTION =====

-- The Circle (building_id 31)
(31, 0,  0,    0,    0,    0,     0    ), -- The Circle 0
(31, 1,  120,  500,  500,  1000,  1000 ), -- The Circle 1
(31, 2,  240,  1000, 1000, 2000,  2000 ), -- The Circle 2
(31, 3,  360,  1500, 1500, 3000,  3000 ), -- The Circle 3
(31, 4,  480,  2000, 2000, 4000,  4000 ), -- The Circle 4
(31, 5,  600,  2500, 2500, 5000,  5000 ), -- The Circle 5
(31, 6,  840,  3000, 3000, 6000,  6000 ), -- The Circle 6
(31, 7,  1080, 3500, 3500, 7000,  7000 ), -- The Circle 7
(31, 8,  1440, 4000, 4000, 8000,  8000 ), -- The Circle 8
(31, 9,  1800, 4500, 4500, 9000,  9000 ), -- The Circle 9
(31, 10, 2400, 5000, 5000, 10000, 10000), -- The Circle 10

-- ===== ELF FACTION =====

-- Arcanum (building_id 48)
(48, 0,  0,    0,    0,    0,     0    ), -- Arcanum 0
(48, 1,  120,  500,  500,  1000,  1000 ), -- Arcanum 1
(48, 2,  240,  1000, 1000, 2000,  2000 ), -- Arcanum 2
(48, 3,  360,  1500, 1500, 3000,  3000 ), -- Arcanum 3
(48, 4,  480,  2000, 2000, 4000,  4000 ), -- Arcanum 4
(48, 5,  600,  2500, 2500, 5000,  5000 ), -- Arcanum 5
(48, 6,  840,  3000, 3000, 6000,  6000 ), -- Arcanum 6
(48, 7,  1080, 3500, 3500, 7000,  7000 ), -- Arcanum 7
(48, 8,  1440, 4000, 4000, 8000,  8000 ), -- Arcanum 8
(48, 9,  1800, 4500, 4500, 9000,  9000 ), -- Arcanum 9
(48, 10, 2400, 5000, 5000, 10000, 10000), -- Arcanum 10

-- ===== DWARF FACTION =====

-- Hall of Runes (building_id 65)
(65, 0,  0,    0,    0,    0,     0    ), -- Hall of Runes 0
(65, 1,  120,  500,  500,  1000,  1000 ), -- Hall of Runes 1
(65, 2,  240,  1000, 1000, 2000,  2000 ), -- Hall of Runes 2
(65, 3,  360,  1500, 1500, 3000,  3000 ), -- Hall of Runes 3
(65, 4,  480,  2000, 2000, 4000,  4000 ), -- Hall of Runes 4
(65, 5,  600,  2500, 2500, 5000,  5000 ), -- Hall of Runes 5
(65, 6,  840,  3000, 3000, 6000,  6000 ), -- Hall of Runes 6
(65, 7,  1080, 3500, 3500, 7000,  7000 ), -- Hall of Runes 7
(65, 8,  1440, 4000, 4000, 8000,  8000 ), -- Hall of Runes 8
(65, 9,  1800, 4500, 4500, 9000,  9000 ), -- Hall of Runes 9
(65, 10, 2400, 5000, 5000, 10000, 10000), -- Hall of Runes 10

-- ===== GOBLIN FACTION =====

-- Mana Den (building_id 82)
(82, 0,  0,    0,    0,    0,     0    ), -- Mana Den 0
(82, 1,  120,  500,  500,  1000,  1000 ), -- Mana Den 1
(82, 2,  240,  1000, 1000, 2000,  2000 ), -- Mana Den 2
(82, 3,  360,  1500, 1500, 3000,  3000 ), -- Mana Den 3
(82, 4,  480,  2000, 2000, 4000,  4000 ), -- Mana Den 4
(82, 5,  600,  2500, 2500, 5000,  5000 ), -- Mana Den 5
(82, 6,  840,  3000, 3000, 6000,  6000 ), -- Mana Den 6
(82, 7,  1080, 3500, 3500, 7000,  7000 ), -- Mana Den 7
(82, 8,  1440, 4000, 4000, 8000,  8000 ), -- Mana Den 8
(82, 9,  1800, 4500, 4500, 9000,  9000 ), -- Mana Den 9
(82, 10, 2400, 5000, 5000, 10000, 10000)  -- Mana Den 10
ON CONFLICT (building_id, level) DO NOTHING;
