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