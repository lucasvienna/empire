CREATE TABLE factions
(
    id   SERIAL NOT NULL PRIMARY KEY,
    name TEXT   NOT NULL
);

INSERT INTO factions (name)
VALUES ('Neutral'), -- 1
       ('Humans'),  -- 2
       ('Orcs'),    -- 3
       ('Elves'),   -- 4
       ('Dwarves'), -- 5
       ('Goblins'); -- 6
