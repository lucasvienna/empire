CREATE TABLE building_resources
(
    id             UUID    NOT NULL DEFAULT generate_ulid(),
    building_id    INTEGER NOT NULL,
    building_level INTEGER NOT NULL,
    -- These are all in resources per minute
    population     INTEGER NOT NULL,
    food           INTEGER NOT NULL,
    wood           INTEGER NOT NULL,
    stone          INTEGER NOT NULL,
    gold           INTEGER NOT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY (building_id) REFERENCES buildings (id) ON DELETE CASCADE,
    FOREIGN KEY (building_id, building_level) REFERENCES building_levels (building_id, level)
);

INSERT INTO building_resources (building_id, building_level, population, food, wood, stone, gold)
VALUES
-- Humans --
-- Keep
(1, 0, 10, 0, 0, 0, 0),   -- Keep 0
(1, 1, 20, 0, 0, 0, 0),   -- Keep 1
(1, 2, 30, 0, 0, 0, 0),   -- Keep 2
(1, 3, 40, 0, 0, 0, 0),   -- Keep 3
(1, 4, 50, 0, 0, 0, 0),   -- Keep 4
(1, 5, 60, 0, 0, 0, 0),   -- Keep 5
(1, 6, 75, 0, 0, 0, 0),   -- Keep 6
(1, 7, 90, 0, 0, 0, 0),   -- Keep 7
(1, 8, 110, 0, 0, 0, 0),  -- Keep 8
(1, 9, 130, 0, 0, 0, 0),  -- Keep 9
(1, 10, 150, 0, 0, 0, 0), -- Keep 10
-- Farm
(2, 0, 0, 10, 0, 0, 0),   -- Farm 0
(2, 1, 0, 20, 0, 0, 0),   -- Farm 1
(2, 2, 0, 30, 0, 0, 0),   -- Farm 2
(2, 3, 0, 50, 0, 0, 0),   -- Farm 3
(2, 4, 0, 70, 0, 0, 0),   -- Farm 4
(2, 5, 0, 90, 0, 0, 0),   -- Farm 5
(2, 6, 0, 120, 0, 0, 0),  -- Farm 6
(2, 7, 0, 150, 0, 0, 0),  -- Farm 7
(2, 8, 0, 175, 0, 0, 0),  -- Farm 8
(2, 9, 0, 200, 0, 0, 0),  -- Farm 9
(2, 10, 0, 250, 0, 0, 0), -- Farm 10
-- Lumberyard
(3, 0, 0, 0, 10, 0, 0),   -- Lumberyard 0
(3, 1, 0, 0, 20, 0, 0),   -- Lumberyard 1
(3, 2, 0, 0, 30, 0, 0),   -- Lumberyard 2
(3, 3, 0, 0, 50, 0, 0),   -- Lumberyard 3
(3, 4, 0, 0, 70, 0, 0),   -- Lumberyard 4
(3, 5, 0, 0, 90, 0, 0),   -- Lumberyard 5
(3, 6, 0, 0, 120, 0, 0),  -- Lumberyard 6
(3, 7, 0, 0, 150, 0, 0),  -- Lumberyard 7
(3, 8, 0, 0, 175, 0, 0),  -- Lumberyard 8
(3, 9, 0, 0, 200, 0, 0),  -- Lumberyard 9
(3, 10, 0, 0, 250, 0, 0), -- Lumberyard 10
-- Quarry
(4, 0, 0, 0, 0, 8, 0),    -- Quarry 0
(4, 1, 0, 0, 0, 16, 0),   -- Quarry 1
(4, 2, 0, 0, 0, 24, 0),   -- Quarry 2
(4, 3, 0, 0, 0, 40, 0),   -- Quarry 3
(4, 4, 0, 0, 0, 56, 0),   -- Quarry 4
(4, 5, 0, 0, 0, 72, 0),   -- Quarry 5
(4, 6, 0, 0, 0, 96, 0),   -- Quarry 6
(4, 7, 0, 0, 0, 120, 0),  -- Quarry 7
(4, 8, 0, 0, 0, 140, 0),  -- Quarry 8
(4, 9, 0, 0, 0, 160, 0),  -- Quarry 9
(4, 10, 0, 0, 0, 200, 0), -- Quarry 10
-- Mine
(5, 0, 0, 0, 0, 0, 8),    -- Mine 0
(5, 1, 0, 0, 0, 0, 16),   -- Mine 1
(5, 2, 0, 0, 0, 0, 24),   -- Mine 2
(5, 3, 0, 0, 0, 0, 40),   -- Mine 3
(5, 4, 0, 0, 0, 0, 56),   -- Mine 4
(5, 5, 0, 0, 0, 0, 72),   -- Mine 5
(5, 6, 0, 0, 0, 0, 96),   -- Mine 6
(5, 7, 0, 0, 0, 0, 120),  -- Mine 7
(5, 8, 0, 0, 0, 0, 140),  -- Mine 8
(5, 9, 0, 0, 0, 0, 160),  -- Mine 9
(5, 10, 0, 0, 0, 0, 200) -- Mine 10