CREATE TABLE user_buildings
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id      INTEGER                           NOT NULL,
    building_id  INTEGER                           NOT NULL,
    level        INTEGER                           NOT NULL DEFAULT 0,
    upgrade_time TEXT                              NULL     DEFAULT NULL, -- RFC 3339

    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (building_id) REFERENCES buildings (id)
);

CREATE TRIGGER new_user_buildings_trigger
    AFTER INSERT
    ON users
BEGIN
    INSERT INTO user_buildings (user_id, building_id, level)
    SELECT NEW.id, id, 1 -- pre-built buildings should be level 1
    FROM buildings
    WHERE faction = NEW.faction
      AND starter = 1;
END;

CREATE TRIGGER delete_user_buildings_trigger
    AFTER DELETE
    ON users
BEGIN
    DELETE FROM user_buildings WHERE user_id = OLD.id;
END;
