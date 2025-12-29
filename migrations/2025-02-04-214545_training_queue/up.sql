CREATE TYPE training_status AS ENUM ('pending', 'in_progress', 'completed', 'cancelled');

CREATE TABLE training_queue
(
    id           UUID            NOT NULL DEFAULT uuidv7(),
    player_id    UUID            NOT NULL,
    building_id  UUID            NOT NULL,
    unit_id      UUID            NOT NULL,
    quantity     BIGINT          NOT NULL DEFAULT 0,
    started_at   TIMESTAMPTZ     NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ     NULL,
    status       training_status NOT NULL DEFAULT 'pending'::training_status,
    job_id       UUID            NULL,
    created_at   TIMESTAMPTZ     NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ     NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (player_id) REFERENCES player (id) ON DELETE CASCADE,
    FOREIGN KEY (building_id) REFERENCES player_building (id) ON DELETE CASCADE,
    FOREIGN KEY (unit_id) REFERENCES unit (id) ON DELETE CASCADE,
    FOREIGN KEY (job_id) REFERENCES job (id) ON DELETE SET NULL
);

CREATE INDEX idx_training_queue_building ON training_queue (building_id);

CREATE TRIGGER set_training_queue_updated_at
    BEFORE UPDATE
    ON training_queue
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();

-- AIDEV-NOTE: Maps which building types can train which unit types
-- e.g., Barracks -> Infantry, Stables -> Cavalry
CREATE TABLE building_unit_type
(
    id          UUID        NOT NULL DEFAULT uuidv7(),
    building_id INTEGER     NOT NULL,
    unit_type   unit_type   NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (building_id) REFERENCES building (id) ON DELETE CASCADE,
    UNIQUE (building_id, unit_type)
);

CREATE TRIGGER set_building_unit_type_updated_at
    BEFORE UPDATE
    ON building_unit_type
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();
