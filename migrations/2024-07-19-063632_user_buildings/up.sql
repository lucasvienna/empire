CREATE TABLE user_buildings
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id      INTEGER                           NOT NULL,
    building_id  INTEGER                           NOT NULL,
    level        INTEGER                           NOT NULL DEFAULT 0,
    upgrade_time TEXT                              NULL     DEFAULT NULL, -- ISO8601 string

    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (building_id) REFERENCES buildings (id)
);