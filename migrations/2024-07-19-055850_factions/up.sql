CREATE TABLE factions
(
    id   INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT    NOT NULL
);

INSERT INTO factions (name)
VALUES ('Neutral'), -- 1
       ('Humans'),  -- 2
       ('Orcs'),    -- 3
       ('Elves'),   -- 4
       ('Dwarves'), -- 5
       ('Goblins'); -- 6
