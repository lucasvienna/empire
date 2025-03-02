CREATE TABLE user_buildings
(
    id           UUID    NOT NULL DEFAULT generate_ulid(),
    user_id      UUID    NOT NULL,
    building_id  INTEGER NOT NULL,
    level        INTEGER NOT NULL DEFAULT 0,
    upgrade_time TEXT    NULL     DEFAULT NULL, -- RFC 3339

    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (building_id) REFERENCES buildings (id)
);

CREATE OR REPLACE FUNCTION new_user_buildings_fn()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS
$$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO user_buildings (user_id, building_id, level)
        SELECT NEW.id, id, 0 -- pre-built buildings should be level 0
        FROM buildings
        WHERE NEW.faction <> 'neutral'
          AND faction = NEW.faction
          AND starter = TRUE;
        RETURN NEW;
    ELSIF TG_OP = 'UPDATE' THEN
        IF OLD.faction <> NEW.faction AND NEW.faction <> 'neutral' THEN
            INSERT INTO user_buildings (user_id, building_id, level)
            SELECT NEW.id, id, 0 -- pre-built buildings should be level 0
            FROM buildings
            WHERE NEW.faction <> 'neutral'
              AND faction = NEW.faction
              AND starter = TRUE;
            RETURN NEW;
        END IF;
    END IF;
END;
$$;

CREATE TRIGGER new_user_buildings_trigger
    AFTER INSERT OR UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION new_user_buildings_fn();

CREATE OR REPLACE FUNCTION update_user_resources_caps_fn()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS
$$
DECLARE
    old_caps RECORD;
    new_caps RECORD;
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- Get the new caps from building_resources based on the building and level
        SELECT food_cap, wood_cap, stone_cap, gold_cap
        INTO new_caps
        FROM building_resources
        WHERE building_id = NEW.building_id
          AND building_level = NEW.level;

        UPDATE user_resources
        SET food_cap  = food_cap + new_caps.food_cap,
            wood_cap  = wood_cap + new_caps.wood_cap,
            stone_cap = stone_cap + new_caps.stone_cap,
            gold_cap  = gold_cap + new_caps.gold_cap
        WHERE user_id = NEW.user_id;

    ELSIF TG_OP = 'UPDATE' THEN
        -- Get the previous cap values from building_resources
        SELECT food_cap, wood_cap, stone_cap, gold_cap
        INTO old_caps
        FROM building_resources
        WHERE building_id = OLD.building_id
          AND building_level = OLD.level;

        -- Get the updated cap values from building_resources
        SELECT food_cap, wood_cap, stone_cap, gold_cap
        INTO new_caps
        FROM building_resources
        WHERE building_id = NEW.building_id
          AND building_level = NEW.level;

        UPDATE user_resources
        SET food_cap  = food_cap - old_caps.food_cap + new_caps.food_cap,
            wood_cap  = wood_cap - old_caps.wood_cap + new_caps.wood_cap,
            stone_cap = stone_cap - old_caps.stone_cap + new_caps.stone_cap,
            gold_cap  = gold_cap - old_caps.gold_cap + new_caps.gold_cap
        WHERE user_id = NEW.user_id;
    END IF;

    RETURN NEW;
END;
$$;

CREATE TRIGGER update_user_resources_caps_trigger
    AFTER INSERT OR UPDATE
    ON user_buildings
    FOR EACH ROW
EXECUTE FUNCTION update_user_resources_caps_fn();