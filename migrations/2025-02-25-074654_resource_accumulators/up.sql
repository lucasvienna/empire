CREATE TABLE resources_accumulator
(
    user_id   UUID    NOT NULL,
    food      INTEGER NOT NULL DEFAULT 0,
    wood      INTEGER NOT NULL DEFAULT 0,
    stone     INTEGER NOT NULL DEFAULT 0,
    gold      INTEGER NOT NULL DEFAULT 0,
    food_cap  INTEGER NOT NULL DEFAULT 2000,
    wood_cap  INTEGER NOT NULL DEFAULT 2000,
    stone_cap INTEGER NOT NULL DEFAULT 2000,
    gold_cap  INTEGER NOT NULL DEFAULT 2000,

    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION new_resource_accumulators_fn()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS
$$
BEGIN
    INSERT INTO resources_accumulator (user_id) VALUES (NEW.id);
    RETURN NEW;
END;
$$;

CREATE TRIGGER new_resource_accumulators_trigger
    AFTER INSERT
    ON users
    FOR EACH ROW
EXECUTE FUNCTION new_resource_accumulators_fn();
