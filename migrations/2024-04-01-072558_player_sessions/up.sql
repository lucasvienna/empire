CREATE TABLE player_session
(
    id         TEXT        NOT NULL,
    player_id  UUID        NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY (player_id) REFERENCES player (id) ON DELETE CASCADE
);