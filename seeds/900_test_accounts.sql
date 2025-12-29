-- Test Account Seed Data
-- Creates test accounts at different progression levels for local testing
-- Accounts: rookie (human), prince (elf), king (orc), emperor (dwarf)
-- Password for all accounts: "password" (argon2id hash)

-- ===== CREATE TEST PLAYERS =====
-- Note: Triggers automatically create player_resource and starter buildings

INSERT INTO player (name, pwd_hash, email, faction)
VALUES ('rookie',  '$argon2id$v=19$m=19456,t=2,p=1$GZlVQdTfzUOQraKTOJipGg$Ivmq9wyal+q849dYcD3X6aTLCjA/g8zZMroTUCVnWzM', 'rookie@neonrook.com',  'human'),
       ('prince',  '$argon2id$v=19$m=19456,t=2,p=1$GZlVQdTfzUOQraKTOJipGg$Ivmq9wyal+q849dYcD3X6aTLCjA/g8zZMroTUCVnWzM', 'prince@neonrook.com',  'elf'  ),
       ('king',    '$argon2id$v=19$m=19456,t=2,p=1$GZlVQdTfzUOQraKTOJipGg$Ivmq9wyal+q849dYcD3X6aTLCjA/g8zZMroTUCVnWzM', 'king@neonrook.com',    'orc'  ),
       ('emperor', '$argon2id$v=19$m=19456,t=2,p=1$GZlVQdTfzUOQraKTOJipGg$Ivmq9wyal+q849dYcD3X6aTLCjA/g8zZMroTUCVnWzM', 'emperor@neonrook.com', 'dwarf')
ON CONFLICT (name) DO NOTHING;


-- ===== ROOKIE (Human, Level 1) - True Beginner =====
-- Buildings: Keep(1), Warehouse(2), Farm(3), Lumberyard(4), Quarry(5), Mine(6) all at level 1
-- Resources: 5000 each

-- Upgrade existing starter buildings to level 1
UPDATE player_building
SET level = 1
WHERE player_id = (SELECT id FROM player WHERE name = 'rookie')
  AND building_id IN (1, 3, 4);

-- Add missing buildings (Warehouse, Quarry, Mine)
INSERT INTO player_building (player_id, building_id, level)
SELECT p.id, b.building_id, b.level
FROM player p
         CROSS JOIN (VALUES (2, 1), (5, 1), (6, 1)) AS b(building_id, level)
WHERE p.name = 'rookie'
ON CONFLICT DO NOTHING;

-- Update resources
UPDATE player_resource
SET food  = 5000,
    wood  = 5000,
    stone = 5000,
    gold  = 5000
WHERE player_id = (SELECT id FROM player WHERE name = 'rookie');


-- ===== PRINCE (Elf, Level 3) - Early Game =====
-- Buildings: Tree of Life(35) L3, Warehouse(36) L3, Farm(37) L3, Lumberyard(38) L3,
--            Quarry(39) L2, Mine(40) L2, Barracks(44) L2, Range(45) L2
-- Resources: 15000 each

-- Upgrade existing starter buildings
UPDATE player_building
SET level = 3
WHERE player_id = (SELECT id FROM player WHERE name = 'prince')
  AND building_id IN (35, 37, 38);

-- Add missing buildings
INSERT INTO player_building (player_id, building_id, level)
SELECT p.id, b.building_id, b.level
FROM player p
         CROSS JOIN (VALUES (36, 3), -- Warehouse L3
                            (39, 2), -- Quarry L2
                            (40, 2), -- Mine L2
                            (44, 2), -- Barracks L2
                            (45, 2)  -- Range L2
) AS b(building_id, level)
WHERE p.name = 'prince'
ON CONFLICT DO NOTHING;

-- Update resources
UPDATE player_resource
SET food  = 15000,
    wood  = 15000,
    stone = 15000,
    gold  = 15000
WHERE player_id = (SELECT id FROM player WHERE name = 'prince');


-- ===== KING (Orc, Level 7) - Mid Game =====
-- Buildings: Stronghold(18) L7, Warehouse(19) L7, Farm(20) L7, Lumberyard(21) L7,
--            Quarry(22) L5, Mine(23) L5, Barracks(27) L5, Range(28) L4, Stables(29) L3,
--            Academy(24) L3, University(25) L2
-- Resources: 75000 each

-- Upgrade existing starter buildings
UPDATE player_building
SET level = 7
WHERE player_id = (SELECT id FROM player WHERE name = 'king')
  AND building_id IN (18, 20, 21);

-- Add missing buildings
INSERT INTO player_building (player_id, building_id, level)
SELECT p.id, b.building_id, b.level
FROM player p
         CROSS JOIN (VALUES (19, 7), -- Warehouse L7
                            (22, 5), -- Quarry L5
                            (23, 5), -- Mine L5
                            (27, 5), -- Barracks L5
                            (28, 4), -- Range L4
                            (29, 3), -- Stables L3
                            (24, 3), -- Academy L3
                            (25, 2) -- University L2
) AS b(building_id, level)
WHERE p.name = 'king'
ON CONFLICT DO NOTHING;

-- Update resources
UPDATE player_resource
SET food  = 75000,
    wood  = 75000,
    stone = 75000,
    gold  = 75000
WHERE player_id = (SELECT id FROM player WHERE name = 'king');


-- ===== EMPEROR (Dwarf, Level 10) - End Game =====
-- Buildings: Hall of Thanes(52) L10, Warehouse(53) L10, Farm(54) L10, Lumberyard(55) L10,
--            Quarry(56) L10, Mine(57) L10, Barracks(61) L10, Range(62) L10, Stables(63) L10,
--            Workshop(64) L8, Academy(58) L10, University(59) L10, Laboratory(60) L8,
--            Hall of Runes(65) L5, Walls(66) L5, Temple(67) L3, Monument(68) L1
-- Resources: 250000 each

-- Upgrade existing starter buildings
UPDATE player_building
SET level = 10
WHERE player_id = (SELECT id FROM player WHERE name = 'emperor')
  AND building_id IN (52, 54, 55);

-- Add missing buildings
INSERT INTO player_building (player_id, building_id, level)
SELECT p.id, b.building_id, b.level
FROM player p
         CROSS JOIN (VALUES (53, 10), -- Warehouse L10
                            (56, 10), -- Quarry L10
                            (57, 10), -- Mine L10
                            (61, 10), -- Barracks L10
                            (62, 10), -- Range L10
                            (63, 10), -- Stables L10
                            (64, 8),  -- Workshop L8
                            (58, 10), -- Academy L10
                            (59, 10), -- University L10
                            (60, 8),  -- Laboratory L8
                            (65, 5),  -- Hall of Runes L5
                            (66, 5),  -- Walls L5
                            (67, 3),  -- Temple L3
                            (68, 1) -- Monument L1
) AS b(building_id, level)
WHERE p.name = 'emperor'
ON CONFLICT DO NOTHING;

-- Update resources
UPDATE player_resource
SET food  = 250000,
    wood  = 250000,
    stone = 250000,
    gold  = 250000
WHERE player_id = (SELECT id FROM player WHERE name = 'emperor');


-- ===== RESOURCE PRODUCTION JOBS =====
-- Schedule resource production jobs for all test players
-- These jobs trigger the resource accumulation system

INSERT INTO job (job_type, status, payload, run_at, priority)
SELECT 'resource',
       'pending',
       jsonb_build_object('ProduceResources', jsonb_build_object('players_id', p.id::text)),
       now(),
       50
FROM player p
WHERE p.name IN ('rookie', 'prince', 'king', 'emperor')
ON CONFLICT DO NOTHING;
