CREATE TABLE user_accumulator
(
    user_id    UUID        NOT NULL,
    food       INTEGER     NOT NULL DEFAULT 0,
    wood       INTEGER     NOT NULL DEFAULT 0,
    stone      INTEGER     NOT NULL DEFAULT 0,
    gold       INTEGER     NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TRIGGER set_user_accumulator_updated_at
    BEFORE UPDATE
    ON user_accumulator
    FOR EACH ROW
EXECUTE PROCEDURE set_current_timestamp_updated_at();

CREATE OR REPLACE FUNCTION new_user_accumulator_fn()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS
$$
BEGIN
    INSERT INTO user_accumulator (user_id) VALUES (NEW.id);
    RETURN NEW;
END;
$$;

CREATE TRIGGER new_user_accumulator_trigger
    AFTER INSERT
    ON users
    FOR EACH ROW
EXECUTE FUNCTION new_user_accumulator_fn();
