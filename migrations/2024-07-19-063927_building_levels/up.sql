CREATE TABLE building_levels
(
    id           SERIAL  NOT NULL PRIMARY KEY,
    building_id  INTEGER NOT NULL,
    level        INTEGER NOT NULL,
    upgrade_time TEXT    NOT NULL, -- "%H:%M:%S"
    req_food     INTEGER,
    req_wood     INTEGER,
    req_stone    INTEGER,
    req_gold     INTEGER,

    FOREIGN KEY (building_id) REFERENCES buildings (id)
);

INSERT INTO building_levels (building_id, level, upgrade_time, req_food, req_wood, req_stone, req_gold)
VALUES
-- 1-15: Humans
(1, 1, '00:01:00', 1000, 1000, 1000, 1000),      -- Keep 1
(1, 2, '00:02:00', 2000, 2000, 2000, 2000),      -- Keep 2
(1, 3, '00:03:00', 3000, 3000, 3000, 3000),      -- Keep 3
(1, 4, '00:04:00', 4000, 4000, 4000, 4000),      -- Keep 4
(1, 5, '00:05:00', 5000, 5000, 5000, 5000),      -- Keep 5
(1, 6, '00:06:00', 6000, 6000, 6000, 6000),      -- Keep 6
(1, 7, '00:07:00', 7000, 7000, 7000, 7000),      -- Keep 7
(1, 8, '00:08:00', 8000, 8000, 8000, 8000),      -- Keep 8
(1, 9, '00:09:00', 9000, 9000, 9000, 9000),      -- Keep 9
(1, 10, '00:10:00', 10000, 10000, 10000, 10000), -- Keep 10
(2, 1, '00:01:00', 100, 100, 0, 0),              -- Farm 1
(2, 2, '00:02:00', 200, 200, 0, 0),              -- Farm 2
(2, 3, '00:03:00', 300, 300, 0, 0),              -- Farm 3
(2, 4, '00:04:00', 400, 400, 0, 0),              -- Farm 4
(2, 5, '00:05:00', 500, 500, 0, 0),              -- Farm 5
(2, 6, '00:06:00', 600, 600, 0, 0),              -- Farm 6
(2, 7, '00:07:00', 700, 700, 0, 0),              -- Farm 7
(2, 8, '00:08:00', 800, 800, 0, 0),              -- Farm 8
(2, 9, '00:09:00', 900, 900, 0, 0),              -- Farm 9
(2, 10, '00:10:00', 1000, 1000, 0, 0); -- Farm 10
