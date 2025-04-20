CREATE TABLE users
(
    id         UUID         NOT NULL DEFAULT generate_ulid(),
    name       TEXT UNIQUE  NOT NULL,
    pwd_hash   TEXT         NOT NULL,
    email      VARCHAR(254) NULL,
    faction    faction_code NOT NULL DEFAULT 'neutral',
    created_at TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (faction) REFERENCES faction (id)
);

CREATE TRIGGER set_users_updated_at
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();
