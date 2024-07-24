CREATE TABLE IF NOT EXISTS building_levels
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    building_id  INTEGER                           NOT NULL,
    level        INTEGER                           NOT NULL,
    upgrade_time TEXT                              NOT NULL, -- ISO8601
    req_food     INTEGER,
    req_wood     INTEGER,
    req_stone    INTEGER,
    req_gold     INTEGER,

    FOREIGN KEY (building_id) REFERENCES buildings (id)
);

-- 1-15: Humans
INSERT INTO building_levels (building_id, level, upgrade_time, req_food, req_wood, req_stone, req_gold)
VALUES (1, 1, '00:01:00', 1000, 1000, 1000, 1000), -- Keep 1
       (1, 2, '00:02:00', 2000, 2000, 2000, 2000), -- Keep 2
       (1, 3, '00:03:00', 3000, 3000, 3000, 3000), -- Keep 3
       (1, 4, '00:04:00', 4000, 4000, 4000, 4000), -- Keep 4
       (1, 5, '00:05:00', 5000, 5000, 5000, 5000), -- Keep 5
       (1, 6, '00:06:00', 6000, 6000, 6000, 6000), -- Keep 6
       (1, 7, '00:07:00', 7000, 7000, 7000, 7000), -- Keep 7
       (1, 8, '00:08:00', 8000, 8000, 8000, 8000), -- Keep 8
       (1, 9, '00:09:00', 9000, 9000, 9000, 9000), -- Keep 9
       (1, 10, '00:10:00', 10000, 10000, 10000, 10000); -- Keep 10