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
-- Starter buildings only, 1 of each resource producer
-- Resources: 5000 each

-- Upgrade starter Farm and Lumberyard to level 1 (Keep starts at level 1)
UPDATE player_building
SET level = 1
WHERE player_id = (SELECT id FROM player WHERE name = 'rookie')
  AND building_id IN (3, 4);

-- Add basic buildings (1 resource producer each)
INSERT INTO player_building (player_id, building_id, level)
SELECT p.id, b.building_id, b.level
FROM player p
         CROSS JOIN (VALUES
                            -- Basic infrastructure
                            (2, 1),  -- Warehouse L1
                            -- Resource producers x1 (starter has Farm + Lumberyard)
                            (5, 1),  -- Quarry L1
                            (6, 1),  -- Mine L1
                            -- Basic military
                            (10, 1), -- Barracks L1
                            (11, 1)  -- Range L1
) AS b(building_id, level)
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
-- Some progression buildings, 2 of each resource producer
-- Resources: 15000 each

-- Upgrade all starter buildings to level 3
UPDATE player_building
SET level = 3
WHERE player_id = (SELECT id FROM player WHERE name = 'prince')
  AND building_id IN (35, 37, 38);

-- Add buildings (2 resource producers each)
INSERT INTO player_building (player_id, building_id, level)
SELECT p.id, b.building_id, b.level
FROM player p
         CROSS JOIN (VALUES
                            -- Infrastructure
                            (36, 3), -- Warehouse L3
                            (41, 2), -- Academy L2
                            (49, 2), -- Walls L2
                            -- Resource producers x2 (1 more, starter has 1)
                            (37, 3), -- Farm L3 x1 more
                            (38, 3), -- Lumberyard L3 x1 more
                            (39, 2), (39, 2), -- Quarry L2 x2
                            (40, 2), (40, 2), -- Mine L2 x2
                            -- Military x1 each
                            (44, 2), -- Barracks L2
                            (45, 2), -- Range L2
                            (46, 2), -- Stables L2
                            (47, 2), -- Workshop L2
                            -- Religion x1
                            (50, 2)  -- Shrine L2 x1
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
-- Mid-progression buildings, 3 of each resource producer
-- Resources: 75000 each

-- Upgrade all starter buildings to level 7
UPDATE player_building
SET level = 7
WHERE player_id = (SELECT id FROM player WHERE name = 'king')
  AND building_id IN (18, 20, 21);

-- Add buildings (3 resource producers each, partial military)
INSERT INTO player_building (player_id, building_id, level)
SELECT p.id, b.building_id, b.level
FROM player p
         CROSS JOIN (VALUES
                            -- Infrastructure
                            (19, 7), -- Warehouse L7
                            (24, 3), -- Academy L3
                            (25, 2), -- University L2
                            (31, 3), -- The Circle L3
                            (32, 3), -- Walls L3
                            -- Resource producers x3 (2 more, starter has 1)
                            (20, 7), (20, 7), -- Farm L7 x2 more
                            (21, 7), (21, 7), -- Lumberyard L7 x2 more
                            (22, 5), (22, 5), (22, 5), -- Quarry L5 x3
                            (23, 5), (23, 5), (23, 5), -- Mine L5 x3
                            -- Military (2 barracks/range, 1 stables/workshop)
                            (27, 5), (27, 5), -- Barracks L5 x2
                            (28, 4), (28, 4), -- Range L4 x2
                            (29, 3),          -- Stables L3 x1
                            (30, 3),          -- Workshop L3 x1
                            -- Shamanic Altar x1
                            (33, 2)           -- Shamanic Altar L2 x1
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
-- All buildings at max count and max level (L10)
-- Resources: 250000 each

-- Upgrade all starter buildings to level 10
UPDATE player_building
SET level = 10
WHERE player_id = (SELECT id FROM player WHERE name = 'emperor')
  AND building_id IN (52, 54, 55);

-- Add all buildings to max count at max level
INSERT INTO player_building (player_id, building_id, level)
SELECT p.id, b.building_id, b.level
FROM player p
         CROSS JOIN (VALUES
                            -- Single buildings (all L10)
                            (53, 10), -- Warehouse L10
                            (58, 10), -- Academy L10
                            (59, 10), -- University L10
                            (60, 10), -- Laboratory L10
                            (65, 10), -- Hall of Runes L10
                            (66, 10), -- Walls L10
                            (68, 10), -- Monument L10
                            -- Resource buildings x4 (3 more, starter has 1)
                            (54, 10), (54, 10), (54, 10), -- Farm L10 x3
                            (55, 10), (55, 10), (55, 10), -- Lumberyard L10 x3
                            (56, 10), (56, 10), (56, 10), (56, 10), -- Quarry L10 x4
                            (57, 10), (57, 10), (57, 10), (57, 10), -- Mine L10 x4
                            -- Military buildings x2 (all L10)
                            (61, 10), (61, 10), -- Barracks L10 x2
                            (62, 10), (62, 10), -- Range L10 x2
                            (63, 10), (63, 10), -- Stables L10 x2
                            (64, 10), (64, 10), -- Workshop L10 x2
                            -- Temple x2
                            (67, 10), (67, 10)  -- Temple L10 x2
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
