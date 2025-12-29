CREATE TABLE player_unit
(

    id         UUID        NOT NULL DEFAULT uuidv7(),
    player_id  UUID        NOT NULL,
    unit_id    UUID        NOT NULL,
    quantity   BIGINT      NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (player_id) REFERENCES player (id) ON DELETE CASCADE,
    FOREIGN KEY (unit_id) REFERENCES unit (id) ON DELETE CASCADE,
    CONSTRAINT player_unit_army UNIQUE (player_id, unit_id)
);

CREATE TRIGGER set_player_unit_updated_at
    BEFORE UPDATE
    ON player_unit
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();
