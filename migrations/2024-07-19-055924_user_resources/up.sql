CREATE TABLE resources
(
    user_id   UUID    NOT NULL,
    food      INTEGER NOT NULL DEFAULT 100,
    wood      INTEGER NOT NULL DEFAULT 100,
    stone     INTEGER NOT NULL DEFAULT 100,
    gold      INTEGER NOT NULL DEFAULT 100,
    food_cap  INTEGER NOT NULL DEFAULT 0,
    wood_cap  INTEGER NOT NULL DEFAULT 0,
    stone_cap INTEGER NOT NULL DEFAULT 0,
    gold_cap  INTEGER NOT NULL DEFAULT 0,

    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION new_user_resources_fn()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS
$$
BEGIN
    INSERT INTO resources (user_id) VALUES (NEW.id);
    RETURN NEW;
END;
$$;

CREATE TRIGGER new_user_resources_trigger
    AFTER INSERT
    ON users
    FOR EACH ROW
EXECUTE FUNCTION new_user_resources_fn();
