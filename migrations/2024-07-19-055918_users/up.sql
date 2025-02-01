CREATE TABLE users
(
    id              UUID      NOT NULL DEFAULT generate_ulid(),
    name            TEXT      NOT NULL,
    faction         INTEGER   NOT NULL DEFAULT 2, -- Humans
    data            jsonb,

    PRIMARY KEY (id),
    FOREIGN KEY (faction) REFERENCES factions (id)
);
