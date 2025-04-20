CREATE TYPE resource_type AS ENUM ('population', 'food', 'wood', 'stone', 'gold');

CREATE TABLE player_resource
(
    player_id    UUID        NOT NULL,
    food       INTEGER     NOT NULL DEFAULT 100,
    wood       INTEGER     NOT NULL DEFAULT 100,
    stone      INTEGER     NOT NULL DEFAULT 100,
    gold       INTEGER     NOT NULL DEFAULT 100,
    food_cap   INTEGER     NOT NULL DEFAULT 0,
    wood_cap   INTEGER     NOT NULL DEFAULT 0,
    stone_cap  INTEGER     NOT NULL DEFAULT 0,
    gold_cap   INTEGER     NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (player_id),
    FOREIGN KEY (player_id) REFERENCES player (id) ON DELETE CASCADE
);

CREATE TRIGGER set_player_resource_updated_at
    BEFORE UPDATE
    ON player_resource
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();

CREATE OR REPLACE FUNCTION new_player_resource_fn()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS
$$
BEGIN
    INSERT INTO player_resource (player_id) VALUES (NEW.id);
    RETURN NEW;
END;
$$;

CREATE TRIGGER new_player_resource_trigger
    AFTER INSERT
    ON player
    FOR EACH ROW
EXECUTE FUNCTION new_player_resource_fn();
