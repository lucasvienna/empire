CREATE TABLE users
(
    id      INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name    TEXT    NOT NULL,
    faction INTEGER NOT NULL DEFAULT 0,
    data    BLOB,

    FOREIGN KEY (faction) REFERENCES factions (id)
);
