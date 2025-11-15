-- Base Building Levels Seed Data
-- Base buildings: Keep/Stronghold/Tree of Life/Hall of Thanes/Big Shack (main building) and Warehouse
-- These are the foundational buildings for each faction

INSERT INTO building_level (building_id, level, upgrade_seconds, req_food, req_wood, req_stone, req_gold)
VALUES
-- ===== HUMAN FACTION (building_id 1-17) =====

-- Keep (building_id 1)
(1,  0,  0,   0,     0,     0,     0    ), -- Keep 0
(1,  1,  60,  1000,  1000,  1000,  1000 ), -- Keep 1
(1,  2,  120, 2000,  2000,  2000,  2000 ), -- Keep 2
(1,  3,  180, 3000,  3000,  3000,  3000 ), -- Keep 3
(1,  4,  240, 4000,  4000,  4000,  4000 ), -- Keep 4
(1,  5,  300, 5000,  5000,  5000,  5000 ), -- Keep 5
(1,  6,  360, 6000,  6000,  6000,  6000 ), -- Keep 6
(1,  7,  420, 7000,  7000,  7000,  7000 ), -- Keep 7
(1,  8,  480, 8000,  8000,  8000,  8000 ), -- Keep 8
(1,  9,  540, 9000,  9000,  9000,  9000 ), -- Keep 9
(1,  10, 600, 10000, 10000, 10000, 10000), -- Keep 10

-- Warehouse (building_id 2)
(2,  0,  0,   0,     0,     0,     0    ), -- Warehouse 0
(2,  1,  60,  200,   100,   100,   50   ), -- Warehouse 1
(2,  2,  120, 400,   200,   200,   100  ), -- Warehouse 2
(2,  3,  180, 600,   300,   300,   150  ), -- Warehouse 3
(2,  4,  240, 800,   400,   400,   200  ), -- Warehouse 4
(2,  5,  300, 1000,  500,   500,   250  ), -- Warehouse 5
(2,  6,  360, 1200,  600,   600,   300  ), -- Warehouse 6
(2,  7,  420, 1400,  700,   700,   350  ), -- Warehouse 7
(2,  8,  480, 1600,  800,   800,   400  ), -- Warehouse 8
(2,  9,  540, 1800,  900,   900,   450  ), -- Warehouse 9
(2,  10, 600, 2000,  1000,  1000,  500  ), -- Warehouse 10

-- ===== ORC FACTION (building_id 18-34) =====

-- Stronghold (building_id 18)
(18, 0,  0,   0,     0,     0,     0    ), -- Stronghold 0
(18, 1,  60,  1000,  1000,  1000,  1000 ), -- Stronghold 1
(18, 2,  120, 2000,  2000,  2000,  2000 ), -- Stronghold 2
(18, 3,  180, 3000,  3000,  3000,  3000 ), -- Stronghold 3
(18, 4,  240, 4000,  4000,  4000,  4000 ), -- Stronghold 4
(18, 5,  300, 5000,  5000,  5000,  5000 ), -- Stronghold 5
(18, 6,  360, 6000,  6000,  6000,  6000 ), -- Stronghold 6
(18, 7,  420, 7000,  7000,  7000,  7000 ), -- Stronghold 7
(18, 8,  480, 8000,  8000,  8000,  8000 ), -- Stronghold 8
(18, 9,  540, 9000,  9000,  9000,  9000 ), -- Stronghold 9
(18, 10, 600, 10000, 10000, 10000, 10000), -- Stronghold 10

-- Warehouse (building_id 19)
(19, 0,  0,   0,     0,     0,     0    ), -- Warehouse 0
(19, 1,  60,  200,   100,   100,   50   ), -- Warehouse 1
(19, 2,  120, 400,   200,   200,   100  ), -- Warehouse 2
(19, 3,  180, 600,   300,   300,   150  ), -- Warehouse 3
(19, 4,  240, 800,   400,   400,   200  ), -- Warehouse 4
(19, 5,  300, 1000,  500,   500,   250  ), -- Warehouse 5
(19, 6,  360, 1200,  600,   600,   300  ), -- Warehouse 6
(19, 7,  420, 1400,  700,   700,   350  ), -- Warehouse 7
(19, 8,  480, 1600,  800,   800,   400  ), -- Warehouse 8
(19, 9,  540, 1800,  900,   900,   450  ), -- Warehouse 9
(19, 10, 600, 2000,  1000,  1000,  500  ), -- Warehouse 10

-- ===== ELF FACTION (building_id 35-51) =====

-- Tree of Life (building_id 35)
(35, 0,  0,   0,     0,     0,     0    ), -- Tree of Life 0
(35, 1,  60,  1000,  1000,  1000,  1000 ), -- Tree of Life 1
(35, 2,  120, 2000,  2000,  2000,  2000 ), -- Tree of Life 2
(35, 3,  180, 3000,  3000,  3000,  3000 ), -- Tree of Life 3
(35, 4,  240, 4000,  4000,  4000,  4000 ), -- Tree of Life 4
(35, 5,  300, 5000,  5000,  5000,  5000 ), -- Tree of Life 5
(35, 6,  360, 6000,  6000,  6000,  6000 ), -- Tree of Life 6
(35, 7,  420, 7000,  7000,  7000,  7000 ), -- Tree of Life 7
(35, 8,  480, 8000,  8000,  8000,  8000 ), -- Tree of Life 8
(35, 9,  540, 9000,  9000,  9000,  9000 ), -- Tree of Life 9
(35, 10, 600, 10000, 10000, 10000, 10000), -- Tree of Life 10

-- Warehouse (building_id 36)
(36, 0,  0,   0,     0,     0,     0    ), -- Warehouse 0
(36, 1,  60,  200,   100,   100,   50   ), -- Warehouse 1
(36, 2,  120, 400,   200,   200,   100  ), -- Warehouse 2
(36, 3,  180, 600,   300,   300,   150  ), -- Warehouse 3
(36, 4,  240, 800,   400,   400,   200  ), -- Warehouse 4
(36, 5,  300, 1000,  500,   500,   250  ), -- Warehouse 5
(36, 6,  360, 1200,  600,   600,   300  ), -- Warehouse 6
(36, 7,  420, 1400,  700,   700,   350  ), -- Warehouse 7
(36, 8,  480, 1600,  800,   800,   400  ), -- Warehouse 8
(36, 9,  540, 1800,  900,   900,   450  ), -- Warehouse 9
(36, 10, 600, 2000,  1000,  1000,  500  ), -- Warehouse 10

-- ===== DWARF FACTION (building_id 52-68) =====

-- Hall of Thanes (building_id 52)
(52, 0,  0,   0,     0,     0,     0    ), -- Hall of Thanes 0
(52, 1,  60,  1000,  1000,  1000,  1000 ), -- Hall of Thanes 1
(52, 2,  120, 2000,  2000,  2000,  2000 ), -- Hall of Thanes 2
(52, 3,  180, 3000,  3000,  3000,  3000 ), -- Hall of Thanes 3
(52, 4,  240, 4000,  4000,  4000,  4000 ), -- Hall of Thanes 4
(52, 5,  300, 5000,  5000,  5000,  5000 ), -- Hall of Thanes 5
(52, 6,  360, 6000,  6000,  6000,  6000 ), -- Hall of Thanes 6
(52, 7,  420, 7000,  7000,  7000,  7000 ), -- Hall of Thanes 7
(52, 8,  480, 8000,  8000,  8000,  8000 ), -- Hall of Thanes 8
(52, 9,  540, 9000,  9000,  9000,  9000 ), -- Hall of Thanes 9
(52, 10, 600, 10000, 10000, 10000, 10000), -- Hall of Thanes 10

-- Warehouse (building_id 53)
(53, 0,  0,   0,     0,     0,     0    ), -- Warehouse 0
(53, 1,  60,  200,   100,   100,   50   ), -- Warehouse 1
(53, 2,  120, 400,   200,   200,   100  ), -- Warehouse 2
(53, 3,  180, 600,   300,   300,   150  ), -- Warehouse 3
(53, 4,  240, 800,   400,   400,   200  ), -- Warehouse 4
(53, 5,  300, 1000,  500,   500,   250  ), -- Warehouse 5
(53, 6,  360, 1200,  600,   600,   300  ), -- Warehouse 6
(53, 7,  420, 1400,  700,   700,   350  ), -- Warehouse 7
(53, 8,  480, 1600,  800,   800,   400  ), -- Warehouse 8
(53, 9,  540, 1800,  900,   900,   450  ), -- Warehouse 9
(53, 10, 600, 2000,  1000,  1000,  500  ), -- Warehouse 10

-- ===== GOBLIN FACTION (building_id 69-85) =====

-- The Big Shack (building_id 69)
(69, 0,  0,   0,     0,     0,     0    ), -- The Big Shack 0
(69, 1,  60,  1000,  1000,  1000,  1000 ), -- The Big Shack 1
(69, 2,  120, 2000,  2000,  2000,  2000 ), -- The Big Shack 2
(69, 3,  180, 3000,  3000,  3000,  3000 ), -- The Big Shack 3
(69, 4,  240, 4000,  4000,  4000,  4000 ), -- The Big Shack 4
(69, 5,  300, 5000,  5000,  5000,  5000 ), -- The Big Shack 5
(69, 6,  360, 6000,  6000,  6000,  6000 ), -- The Big Shack 6
(69, 7,  420, 7000,  7000,  7000,  7000 ), -- The Big Shack 7
(69, 8,  480, 8000,  8000,  8000,  8000 ), -- The Big Shack 8
(69, 9,  540, 9000,  9000,  9000,  9000 ), -- The Big Shack 9
(69, 10, 600, 10000, 10000, 10000, 10000), -- The Big Shack 10

-- Warehouse (building_id 70)
(70, 0,  0,   0,     0,     0,     0    ), -- Warehouse 0
(70, 1,  60,  200,   100,   100,   50   ), -- Warehouse 1
(70, 2,  120, 400,   200,   200,   100  ), -- Warehouse 2
(70, 3,  180, 600,   300,   300,   150  ), -- Warehouse 3
(70, 4,  240, 800,   400,   400,   200  ), -- Warehouse 4
(70, 5,  300, 1000,  500,   500,   250  ), -- Warehouse 5
(70, 6,  360, 1200,  600,   600,   300  ), -- Warehouse 6
(70, 7,  420, 1400,  700,   700,   350  ), -- Warehouse 7
(70, 8,  480, 1600,  800,   800,   400  ), -- Warehouse 8
(70, 9,  540, 1800,  900,   900,   450  ), -- Warehouse 9
(70, 10, 600, 2000,  1000,  1000,  500  )  -- Warehouse 10
ON CONFLICT (building_id, level) DO NOTHING;
