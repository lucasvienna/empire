CREATE TABLE factions
(
    id   TEXT CHECK (id IN ('H', 'O', 'E')) NOT NULL PRIMARY KEY,
    name TEXT                               NOT NULL
);