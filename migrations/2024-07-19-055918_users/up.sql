CREATE TABLE users
(
    id      INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name    TEXT    NOT NULL,
    faction INTEGER NOT NULL DEFAULT 2, -- Humans
    data    BLOB,

    FOREIGN KEY (faction) REFERENCES factions (id)
);
