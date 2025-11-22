CREATE TABLE player_accumulator
(
    id         UUID        NOT NULL DEFAULT uuidv7(),
    player_id  UUID UNIQUE NOT NULL,
    food       BIGINT      NOT NULL DEFAULT 0,
    wood       BIGINT      NOT NULL DEFAULT 0,
    stone      BIGINT      NOT NULL DEFAULT 0,
    gold       BIGINT      NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (player_id) REFERENCES player (id) ON DELETE CASCADE
);

CREATE TRIGGER set_player_accumulator_updated_at
    BEFORE UPDATE
    ON player_accumulator
    FOR EACH ROW
EXECUTE PROCEDURE set_current_timestamp_updated_at();

CREATE OR REPLACE FUNCTION new_player_accumulator_fn()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS
$$
BEGIN
    INSERT INTO player_accumulator (player_id) VALUES (NEW.id);
    RETURN NEW;
END;
$$;

CREATE TRIGGER new_player_accumulator_trigger
    AFTER INSERT
    ON player
    FOR EACH ROW
EXECUTE FUNCTION new_player_accumulator_fn();
