CREATE TABLE buildings
(
    id        SERIAL  NOT NULL PRIMARY KEY,
    name      TEXT    NOT NULL,
    max_level INTEGER NOT NULL,
    max_count INTEGER NOT NULL,
    faction   INTEGER NOT NULL,
    starter   BOOLEAN NOT NULL DEFAULT FALSE,

    FOREIGN KEY (faction) REFERENCES factions (id)
);

INSERT INTO buildings (name, max_level, max_count, faction, starter)
VALUES ('Keep',           10, 1, 2, TRUE ), -- Humans
       ('Warehouse',      10, 1, 2, FALSE), -- Humans
       ('Farm',           10, 4, 2, TRUE ), -- Humans
       ('Lumberyard',     10, 4, 2, TRUE ), -- Humans
       ('Quarry',         10, 4, 2, FALSE), -- Humans
       ('Mine',           10, 4, 2, FALSE), -- Humans
       ('Academy',        10, 1, 2, FALSE), -- Humans
       ('University',     10, 1, 2, FALSE), -- Humans
       ('Laboratory',     10, 1, 2, FALSE), -- Humans
       ('Barracks',       10, 2, 2, FALSE), -- Humans
       ('Range',          10, 2, 2, FALSE), -- Humans
       ('Stables',        10, 2, 2, FALSE), -- Humans
       ('Workshop',       10, 2, 2, FALSE), -- Humans
       ('Mage Tower',     10, 1, 2, FALSE), -- Humans
       ('Walls',          10, 1, 2, FALSE), -- Humans
       ('Church',         10, 2, 2, FALSE), -- Humans
       ('Monument',       10, 1, 2, FALSE), -- Humans

       ('Stronghold',     10, 1, 3, TRUE ), -- Orcs
       ('Warehouse',      10, 1, 3, FALSE), -- Orcs
       ('Farm',           10, 4, 3, TRUE ), -- Orcs
       ('Lumberyard',     10, 4, 3, TRUE ), -- Orcs
       ('Quarry',         10, 4, 3, FALSE), -- Orcs
       ('Mine',           10, 4, 3, FALSE), -- Orcs
       ('Academy',        10, 1, 3, FALSE), -- Orcs
       ('University',     10, 1, 3, FALSE), -- Orcs
       ('Laboratory',     10, 1, 3, FALSE), -- Orcs
       ('Barracks',       10, 2, 3, FALSE), -- Orcs
       ('Range',          10, 2, 3, FALSE), -- Orcs
       ('Stables',        10, 2, 3, FALSE), -- Orcs
       ('Workshop',       10, 2, 3, FALSE), -- Orcs
       ('The Circle',     10, 1, 3, FALSE), -- Orcs (Mage Tower)
       ('Walls',          10, 1, 3, FALSE), -- Orcs
       ('Shamanic Altar', 10, 2, 3, FALSE), -- Orcs (Church)
       ('Monument',       10, 1, 3, FALSE), -- Orcs

       ('Tree of Life',   10, 1, 4, TRUE ), -- Elves
       ('Warehouse',      10, 1, 4, FALSE), -- Elves
       ('Farm',           10, 4, 4, TRUE ), -- Elves
       ('Lumberyard',     10, 4, 4, TRUE ), -- Elves
       ('Quarry',         10, 4, 4, FALSE), -- Elves
       ('Mine',           10, 4, 4, FALSE), -- Elves
       ('Academy',        10, 1, 4, FALSE), -- Elves
       ('University',     10, 1, 4, FALSE), -- Elves
       ('Laboratory',     10, 1, 4, FALSE), -- Elves
       ('Barracks',       10, 2, 4, FALSE), -- Elves
       ('Range',          10, 2, 4, FALSE), -- Elves
       ('Stables',        10, 2, 4, FALSE), -- Elves
       ('Workshop',       10, 2, 4, FALSE), -- Elves
       ('Arcanum',        10, 1, 4, FALSE), -- Elves (Mage Tower)
       ('Walls',          10, 1, 4, FALSE), -- Elves
       ('Shrine',         10, 2, 4, FALSE), -- Elves (Church)
       ('Monument',       10, 1, 4, FALSE), -- Elves

       ('Hall of Thanes', 10, 1, 5, TRUE ), -- Dwarves
       ('Warehouse',      10, 1, 5, FALSE), -- Humans
       ('Farm',           10, 4, 5, TRUE ), -- Dwarves
       ('Lumberyard',     10, 4, 5, TRUE ), -- Dwarves
       ('Quarry',         10, 4, 5, FALSE), -- Dwarves
       ('Mine',           10, 4, 5, FALSE), -- Dwarves
       ('Academy',        10, 1, 5, FALSE), -- Dwarves
       ('University',     10, 1, 5, FALSE), -- Dwarves
       ('Laboratory',     10, 1, 5, FALSE), -- Dwarves
       ('Barracks',       10, 2, 5, FALSE), -- Dwarves
       ('Range',          10, 2, 5, FALSE), -- Dwarves
       ('Stables',        10, 2, 5, FALSE), -- Dwarves
       ('Workshop',       10, 2, 5, FALSE), -- Dwarves
       ('Hall of Runes',  10, 1, 5, FALSE), -- Dwarves (Mage Tower)
       ('Walls',          10, 1, 5, FALSE), -- Dwarves
       ('Temple',         10, 2, 5, FALSE), -- Dwarves (Church)
       ('Monument',       10, 1, 5, FALSE), -- Dwarves

       ('The Big Shack',  10, 1, 6, TRUE ), -- Goblins
       ('Warehouse',      10, 1, 6, FALSE), -- Goblins
       ('Farm',           10, 4, 6, TRUE ), -- Goblins
       ('Lumberyard',     10, 4, 6, TRUE ), -- Goblins
       ('Quarry',         10, 4, 6, FALSE), -- Goblins
       ('Mine',           10, 4, 6, FALSE), -- Goblins
       ('Cadet School',   10, 1, 6, FALSE), -- Goblins
       ('Brainery',       10, 1, 6, FALSE), -- Goblins
       ('Laboratory',     10, 1, 6, FALSE), -- Goblins
       ('Barracks',       10, 2, 6, FALSE), -- Goblins
       ('Range',          10, 2, 6, FALSE), -- Goblins
       ('Stables',        10, 2, 6, FALSE), -- Goblins
       ('Workshop',       10, 2, 6, FALSE), -- Goblins
       ('Mana Den',       10, 1, 6, FALSE), -- Goblins (Mage Tower)
       ('Walls',          10, 1, 6, FALSE), -- Goblins
       ('Speaker''s Hut', 10, 2, 6, FALSE), -- Goblins (Church)
       ('Monument',       10, 1, 6, FALSE), -- Dwarves

       ('Guild Hall',     10, 1, 1, FALSE), -- Neutral
       ('Market',         10, 1, 1, FALSE), -- Neutral
       ('Embassy',        10, 1, 1, FALSE); -- Neutral
