CREATE TABLE users
(
    id       UUID         NOT NULL DEFAULT generate_ulid(),
    name     TEXT UNIQUE  NOT NULL,
    pwd_hash TEXT         NOT NULL,
    email    VARCHAR(254) NULL,
    faction  faction_code NOT NULL DEFAULT 'neutral',

    PRIMARY KEY (id),
    FOREIGN KEY (faction) REFERENCES factions (id)
);
