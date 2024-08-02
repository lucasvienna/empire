CREATE TABLE resources
(
    user_id INTEGER NOT NULL,
    food    INTEGER NOT NULL DEFAULT 100,
    wood    INTEGER NOT NULL DEFAULT 100,
    stone   INTEGER NOT NULL DEFAULT 100,
    gold    INTEGER NOT NULL DEFAULT 100,

    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TRIGGER new_user_resources_trigger
    AFTER INSERT
    ON users
BEGIN
    INSERT INTO resources (user_id) VALUES (NEW.id);
END;

CREATE TRIGGER delete_user_resources_trigger
    AFTER DELETE
    ON users
BEGIN
    DELETE FROM resources WHERE user_id = OLD.id;
END;
