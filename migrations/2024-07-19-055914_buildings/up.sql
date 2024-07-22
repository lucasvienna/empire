CREATE TABLE buildings
(
    id        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name      TEXT    NOT NULL,
    max_level INTEGER NOT NULL,
    max_count INTEGER NOT NULL,
    faction   INTEGER NOT NULL DEFAULT 0,

    FOREIGN KEY (faction) REFERENCES factions (id)
);
