CREATE TABLE users
(
    id      INTEGER                                 NOT NULL PRIMARY KEY AUTOINCREMENT,
    name    TEXT                                    NOT NULL,
    faction TEXT CHECK (faction IN ('H', 'O', 'E')) NOT NULL DEFAULT 'H',
    data    BLOB,

    FOREIGN KEY (faction) REFERENCES factions (id)
);
