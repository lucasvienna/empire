CREATE TABLE buildings
(
    id        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name      TEXT    NOT NULL,
    max_level INTEGER NOT NULL,
    max_count INTEGER NOT NULL,
    faction   INTEGER NOT NULL,
    starter   BOOLEAN NOT NULL CHECK (starter IN (0, 1)) DEFAULT 0,

    FOREIGN KEY (faction) REFERENCES factions (id)
);

INSERT INTO buildings (name, max_level, max_count, faction, starter)
VALUES ('Keep', 10, 1, 2, 1),           -- Humans
       ('Farm', 10, 4, 2, 1),           -- Humans
       ('Lumberyard', 10, 4, 2, 1),     -- Humans
       ('Quarry', 10, 4, 2, 0),         -- Humans
       ('Mine', 10, 4, 2, 0),           -- Humans
       ('Academy', 10, 1, 2, 0),        -- Humans
       ('University', 10, 1, 2, 0),     -- Humans
       ('Laboratory', 10, 1, 2, 0),     -- Humans
       ('Barracks', 10, 2, 2, 0),       -- Humans
       ('Range', 10, 2, 2, 0),          -- Humans
       ('Stables', 10, 2, 2, 0),        -- Humans
       ('Workshop', 10, 2, 2, 0),       -- Humans
       ('Mage Tower', 10, 1, 2, 0),     -- Humans
       ('Walls', 10, 1, 2, 0),          -- Humans
       ('Church', 10, 2, 2, 0),         -- Humans
       ('Monument', 10, 1, 2, 0),       -- Humans

       ('Stronghold', 10, 1, 3, 1),     -- Orcs
       ('Farm', 10, 4, 3, 1),           -- Orcs
       ('Lumberyard', 10, 4, 3, 1),     -- Orcs
       ('Quarry', 10, 4, 3, 0),         -- Orcs
       ('Mine', 10, 4, 3, 0),           -- Orcs
       ('Academy', 10, 1, 3, 0),        -- Orcs
       ('University', 10, 1, 3, 0),     -- Orcs
       ('Laboratory', 10, 1, 3, 0),     -- Orcs
       ('Barracks', 10, 2, 3, 0),       -- Orcs
       ('Range', 10, 2, 3, 0),          -- Orcs
       ('Stables', 10, 2, 3, 0),        -- Orcs
       ('Workshop', 10, 2, 3, 0),       -- Orcs
       ('The Circle', 10, 1, 3, 0),     -- Orcs (Mage Tower)
       ('Walls', 10, 1, 3, 0),          -- Orcs
       ('Shamanic Altar', 10, 2, 3, 0), -- Orcs (Church)
       ('Monument', 10, 1, 3, 0),       -- Orcs

       ('Tree of Life', 10, 1, 4, 1),   -- Elves
       ('Farm', 10, 4, 4, 1),           -- Elves
       ('Lumberyard', 10, 4, 4, 1),     -- Elves
       ('Quarry', 10, 4, 4, 0),         -- Elves
       ('Mine', 10, 4, 4, 0),           -- Elves
       ('Academy', 10, 1, 4, 0),        -- Elves
       ('University', 10, 1, 4, 0),     -- Elves
       ('Laboratory', 10, 1, 4, 0),     -- Elves
       ('Barracks', 10, 2, 4, 0),       -- Elves
       ('Range', 10, 2, 4, 0),          -- Elves
       ('Stables', 10, 2, 4, 0),        -- Elves
       ('Workshop', 10, 2, 4, 0),       -- Elves
       ('Arcanum', 10, 1, 4, 0),        -- Elves (Mage Tower)
       ('Walls', 10, 1, 4, 0),          -- Elves
       ('Shrine', 10, 2, 4, 0),         -- Elves (Church)
       ('Monument', 10, 1, 4, 0),       -- Elves

       ('Hall of Thanes', 10, 1, 5, 1), -- Dwarves
       ('Farm', 10, 4, 5, 1),           -- Dwarves
       ('Lumberyard', 10, 4, 5, 1),     -- Dwarves
       ('Quarry', 10, 4, 5, 0),         -- Dwarves
       ('Mine', 10, 4, 5, 0),           -- Dwarves
       ('Academy', 10, 1, 5, 0),        -- Dwarves
       ('University', 10, 1, 5, 0),     -- Dwarves
       ('Laboratory', 10, 1, 5, 0),     -- Dwarves
       ('Barracks', 10, 2, 5, 0),       -- Dwarves
       ('Range', 10, 2, 5, 0),          -- Dwarves
       ('Stables', 10, 2, 5, 0),        -- Dwarves
       ('Workshop', 10, 2, 5, 0),       -- Dwarves
       ('Hall of Runes', 10, 1, 5, 0),  -- Dwarves (Mage Tower)
       ('Walls', 10, 1, 5, 0),          -- Dwarves
       ('Temple', 10, 2, 5, 0),         -- Dwarves (Church)
       ('Monument', 10, 1, 5, 0),       -- Dwarves

       ('The Big Shack', 10, 1, 6, 1),  -- Goblins
       ('Farm', 10, 4, 6, 1),           -- Goblins
       ('Lumberyard', 10, 4, 6, 1),     -- Goblins
       ('Quarry', 10, 4, 6, 0),         -- Goblins
       ('Mine', 10, 4, 6, 0),           -- Goblins
       ('Cadet School', 10, 1, 6, 0),   -- Goblins
       ('Brainery', 10, 1, 6, 0),       -- Goblins
       ('Laboratory', 10, 1, 6, 0),     -- Goblins
       ('Barracks', 10, 2, 6, 0),       -- Goblins
       ('Range', 10, 2, 6, 0),          -- Goblins
       ('Stables', 10, 2, 6, 0),        -- Goblins
       ('Workshop', 10, 2, 6, 0),       -- Goblins
       ('Mana Den', 10, 1, 6, 0),       -- Goblins (Mage Tower)
       ('Walls', 10, 1, 6, 0),          -- Goblins
       ('Speaker''s Hut', 10, 2, 6, 0), -- Goblins (Church)
       ('Monument', 10, 1, 6, 0),       -- Dwarves

       ('Guild Hall', 10, 1, 1, 0),     -- Neutral
       ('Market', 10, 1, 1, 0),         -- Neutral
       ('Embassy', 10, 1, 1, 0); -- Neutral
