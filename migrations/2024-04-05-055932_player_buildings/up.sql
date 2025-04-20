CREATE TABLE player_building
(
    id           UUID        NOT NULL DEFAULT generate_ulid(),
    player_id    UUID        NOT NULL,
    building_id  INTEGER     NOT NULL,
    level        INTEGER     NOT NULL DEFAULT 0,
    upgrade_time TEXT        NULL     DEFAULT NULL, -- RFC 3339
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (player_id) REFERENCES player (id) ON DELETE CASCADE,
    FOREIGN KEY (building_id) REFERENCES building (id)
);

CREATE INDEX player_building_tupple_idx ON player_building (player_id, building_id);

CREATE TRIGGER set_player_building_updated_at
    BEFORE UPDATE
    ON player_building
    FOR EACH ROW
EXECUTE PROCEDURE set_current_timestamp_updated_at();

CREATE OR REPLACE FUNCTION new_player_building_fn()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS
$$
BEGIN
    INSERT INTO player_building (player_id, building_id, level)
    SELECT NEW.id, id, 0 -- pre-built buildings should be level 0
    FROM building
    WHERE faction = NEW.faction
      AND starter = TRUE;

    RETURN NEW;
END;
$$;

CREATE TRIGGER new_player_building_trigger
    AFTER INSERT OR UPDATE
    ON player
    FOR EACH ROW
EXECUTE FUNCTION new_player_building_fn();

CREATE OR REPLACE FUNCTION change_player_building_fn()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS
$$
BEGIN
    -- whenever a player switches from neutral to another faction,
    -- insert all pre-built buildings for that faction
    IF OLD.faction = 'neutral' THEN
        INSERT INTO player_building (player_id, building_id, level)
        SELECT NEW.id, id, 0 -- pre-built buildings should be level 0
        FROM building
        WHERE faction = NEW.faction
          AND starter = TRUE;
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER faction_select_player_building_trigger
    AFTER UPDATE OF faction
    ON player
    FOR EACH ROW
    WHEN (OLD.faction IS DISTINCT FROM NEW.faction)
EXECUTE FUNCTION change_player_building_fn();

-- TODO: add a trigger to switch buildings when the switch changes the faction

CREATE OR REPLACE FUNCTION update_player_resource_caps_fn()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS
$$
DECLARE
    old_caps RECORD;
    new_caps RECORD;
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- Get the new caps from building_resource based on the building and level
        SELECT food_cap, wood_cap, stone_cap, gold_cap
        INTO new_caps
        FROM building_resource
        WHERE building_id = NEW.building_id
          AND building_level = NEW.level;

        UPDATE player_resource
        SET food_cap  = food_cap + new_caps.food_cap,
            wood_cap  = wood_cap + new_caps.wood_cap,
            stone_cap = stone_cap + new_caps.stone_cap,
            gold_cap  = gold_cap + new_caps.gold_cap
        WHERE player_id = NEW.player_id;

    ELSIF TG_OP = 'UPDATE' THEN
        -- Get the previous cap values from building_resource
        SELECT food_cap, wood_cap, stone_cap, gold_cap
        INTO old_caps
        FROM building_resource
        WHERE building_id = OLD.building_id
          AND building_level = OLD.level;

        -- Get the updated cap values from building_resource
        SELECT food_cap, wood_cap, stone_cap, gold_cap
        INTO new_caps
        FROM building_resource
        WHERE building_id = NEW.building_id
          AND building_level = NEW.level;

        UPDATE player_resource
        SET food_cap  = food_cap - old_caps.food_cap + new_caps.food_cap,
            wood_cap  = wood_cap - old_caps.wood_cap + new_caps.wood_cap,
            stone_cap = stone_cap - old_caps.stone_cap + new_caps.stone_cap,
            gold_cap  = gold_cap - old_caps.gold_cap + new_caps.gold_cap
        WHERE player_id = NEW.player_id;
    END IF;

    RETURN NEW;
END;
$$;

CREATE TRIGGER update_player_resource_caps_trigger
    AFTER INSERT OR UPDATE
    ON player_building
    FOR EACH ROW
EXECUTE FUNCTION update_player_resource_caps_fn();