CREATE TABLE user_accumulator
(
    user_id UUID    NOT NULL,
    food    INTEGER NOT NULL DEFAULT 0,
    wood    INTEGER NOT NULL DEFAULT 0,
    stone   INTEGER NOT NULL DEFAULT 0,
    gold    INTEGER NOT NULL DEFAULT 0,

    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION new_user_accumulators_fn()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS
$$
BEGIN
    INSERT INTO user_accumulator (user_id) VALUES (NEW.id);
    RETURN NEW;
END;
$$;

CREATE TRIGGER new_user_accumulators_trigger
    AFTER INSERT
    ON users
    FOR EACH ROW
EXECUTE FUNCTION new_user_accumulators_fn();
