CREATE TYPE training_status AS ENUM ('pending', 'in_progress', 'completed', 'cancelled');

CREATE TABLE training_queue
(

    id           UUID            NOT NULL DEFAULT uuidv7(),
    player_id    UUID            NOT NULL,
    unit_id      UUID            NOT NULL,
    quantity     INTEGER         NOT NULL DEFAULT 0,
    started_at   TIMESTAMPTZ     NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ     NULL,
    status       training_status NOT NULL DEFAULT 'pending'::training_status,
    job_id       UUID            NULL,
    created_at   TIMESTAMPTZ     NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ     NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (player_id) REFERENCES player (id) ON DELETE CASCADE,
    FOREIGN KEY (unit_id) REFERENCES unit (id) ON DELETE CASCADE,
    FOREIGN KEY (job_id) REFERENCES job (id) ON DELETE SET NULL,
    CONSTRAINT player_unit_queue UNIQUE (player_id, unit_id)
);

CREATE TRIGGER set_player_unit_updated_at
    BEFORE UPDATE
    ON training_queue
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();
