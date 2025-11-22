-- Building Requirements Seed Data
-- AIDEV-NOTE: Each building level X requires the faction's main building at level Y where Y >= X
-- Main buildings: Keep (1), Stronghold (18), Tree of Life (35), Hall of Thanes (52), The Big Shack (69)
-- Tech requirements are ignored as per requirements
-- Idempotency: Uses ON CONFLICT with existing UNIQUE constraint on (building_level_id, required_building_id, required_building_level)

-- ===== HUMAN FACTION (building_id 1-17) =====
-- Keep (building_id 1) has no requirements

-- Warehouse (building_id 2) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 2
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Farm (building_id 3) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 3
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Lumberyard (building_id 4) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 4
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Quarry (building_id 5) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 5
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Mine (building_id 6) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 6
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Academy (building_id 7) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 7
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- University (building_id 8) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 8
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Laboratory (building_id 9) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 9
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Barracks (building_id 10) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 10
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Range (building_id 11) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 11
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Stables (building_id 12) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 12
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Workshop (building_id 13) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 13
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Mage Tower (building_id 14) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 14
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Walls (building_id 15) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 15
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Church (building_id 16) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 16
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Monument (building_id 17) requires Keep
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 1, bl.level
FROM building_level bl
WHERE bl.building_id = 17
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- ===== ORC FACTION (building_id 18-34) =====
-- Stronghold (building_id 18) has no requirements

-- Warehouse (building_id 19) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 19
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Farm (building_id 20) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 20
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Lumberyard (building_id 21) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 21
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Quarry (building_id 22) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 22
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Mine (building_id 23) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 23
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Academy (building_id 24) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 24
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- University (building_id 25) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 25
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Laboratory (building_id 26) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 26
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Barracks (building_id 27) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 27
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Range (building_id 28) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 28
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Stables (building_id 29) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 29
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Workshop (building_id 30) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 30
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- The Circle (building_id 31) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 31
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Walls (building_id 32) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 32
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Shamanic Altar (building_id 33) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 33
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Monument (building_id 34) requires Stronghold
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 18, bl.level
FROM building_level bl
WHERE bl.building_id = 34
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- ===== ELF FACTION (building_id 35-51) =====
-- Tree of Life (building_id 35) has no requirements

-- Warehouse (building_id 36) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 36
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Farm (building_id 37) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 37
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Lumberyard (building_id 38) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 38
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Quarry (building_id 39) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 39
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Mine (building_id 40) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 40
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Academy (building_id 41) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 41
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- University (building_id 42) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 42
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Laboratory (building_id 43) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 43
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Barracks (building_id 44) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 44
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Range (building_id 45) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 45
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Stables (building_id 46) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 46
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Workshop (building_id 47) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 47
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Arcanum (building_id 48) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 48
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Walls (building_id 49) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 49
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Shrine (building_id 50) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 50
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Monument (building_id 51) requires Tree of Life
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 35, bl.level
FROM building_level bl
WHERE bl.building_id = 51
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- ===== DWARF FACTION (building_id 52-68) =====
-- Hall of Thanes (building_id 52) has no requirements

-- Warehouse (building_id 53) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 53
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Farm (building_id 54) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 54
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Lumberyard (building_id 55) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 55
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Quarry (building_id 56) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 56
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Mine (building_id 57) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 57
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Academy (building_id 58) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 58
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- University (building_id 59) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 59
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Laboratory (building_id 60) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 60
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Barracks (building_id 61) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 61
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Range (building_id 62) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 62
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Stables (building_id 63) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 63
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Workshop (building_id 64) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 64
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Hall of Runes (building_id 65) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 65
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Walls (building_id 66) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 66
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Temple (building_id 67) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 67
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Monument (building_id 68) requires Hall of Thanes
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 52, bl.level
FROM building_level bl
WHERE bl.building_id = 68
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- ===== GOBLIN FACTION (building_id 69-85) =====
-- The Big Shack (building_id 69) has no requirements

-- Warehouse (building_id 70) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 70
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Farm (building_id 71) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 71
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Lumberyard (building_id 72) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 72
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Quarry (building_id 73) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 73
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Mine (building_id 74) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 74
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Cadet School (building_id 75) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 75
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Brainery (building_id 76) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 76
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Laboratory (building_id 77) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 77
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Barracks (building_id 78) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 78
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Range (building_id 79) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 79
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Stables (building_id 80) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 80
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Workshop (building_id 81) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 81
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Mana Den (building_id 82) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 82
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Walls (building_id 83) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 83
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Speaker's Hut (building_id 84) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 84
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- Monument (building_id 85) requires The Big Shack
INSERT INTO building_requirement (building_level_id, required_building_id, required_building_level)
SELECT bl.id, 69, bl.level
FROM building_level bl
WHERE bl.building_id = 85
  AND bl.level > 0
ON CONFLICT (building_level_id, required_building_id, required_building_level) DO NOTHING;

-- ===== NEUTRAL FACTION (building_id 86-88) =====
-- Neutral buildings have no requirements as they are special cross-faction buildings
-- Guild Hall (building_id 86) - no requirements
-- Market (building_id 87) - no requirements
-- Embassy (building_id 88) - no requirements
