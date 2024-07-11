CREATE TABLE buildings
(
    id        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name      TEXT    NOT NULL,
    max_level INTEGER NOT NULL,
    faction   TEXT    NULL
);
