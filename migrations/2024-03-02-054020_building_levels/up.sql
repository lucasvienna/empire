CREATE TABLE building_level
(
    id                UUID        NOT NULL DEFAULT uuidv7(),
    building_id       INTEGER     NOT NULL,
    level             INTEGER     NOT NULL,
    upgrade_seconds   BIGINT      NOT NULL,
    req_food          BIGINT,
    req_wood          BIGINT,
    req_stone         BIGINT,
    req_gold          BIGINT,
    training_capacity INTEGER,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    UNIQUE (building_id, level),
    FOREIGN KEY (building_id) REFERENCES building (id) ON DELETE CASCADE
);

CREATE TRIGGER set_building_levels_updated_at
    BEFORE UPDATE
    ON building_level
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();
