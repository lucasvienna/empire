CREATE TABLE user_buildings
(
    user         INTEGER NOT NULL,
    building     INTEGER NOT NULL,
    level        INTEGER NOT NULL DEFAULT 0,
    upgrade_time TEXT    NULL, -- ISO8601 string

    PRIMARY KEY (user, building),
    FOREIGN KEY (user) REFERENCES users (id),
    FOREIGN KEY (building) REFERENCES buildings (id)
);