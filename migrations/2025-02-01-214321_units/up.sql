CREATE TYPE unit_type AS ENUM ('infantry', 'ranged', 'cavalry', 'artillery', 'magical');

CREATE TABLE unit
(
    id                    UUID        NOT NULL DEFAULT uuidv7(),
    name                  TEXT        NOT NULL,
    unit_type             unit_type   NOT NULL,
    base_atk              INTEGER     NOT NULL,
    base_def              INTEGER     NOT NULL,
    base_training_seconds INTEGER     NOT NULL,
    description           TEXT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    CONSTRAINT unit_name_unique UNIQUE (name)
);

CREATE TRIGGER set_unit_updated_at
    BEFORE UPDATE
    ON unit
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();
