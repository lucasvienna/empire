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
    INSERT INTO user_buildings (user_id, building_id, level)
    SELECT NEW.id, id, 1 -- pre-built buildings should be level 1
    FROM buildings
    WHERE faction = NEW.faction
      AND starter = TRUE;
    RETURN NEW;
END;
$$;

CREATE TRIGGER new_user_buildings_trigger
    AFTER INSERT
    ON users
    FOR EACH ROW
EXECUTE FUNCTION new_user_buildings_fn();
