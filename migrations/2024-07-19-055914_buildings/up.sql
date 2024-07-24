CREATE TABLE buildings
(
    id        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name      TEXT    NOT NULL,
    max_level INTEGER NOT NULL,
    max_count INTEGER NOT NULL,
    faction   INTEGER NOT NULL DEFAULT 0,

    FOREIGN KEY (faction) REFERENCES factions (id)
);

INSERT INTO buildings (name, max_level, max_count, faction)
VALUES ('Keep', 10, 1, 2),           -- Humans
       ('Farm', 10, 4, 2),           -- Humans
       ('Lumberyard', 10, 4, 2),     -- Humans
       ('Quarry', 10, 4, 2),         -- Humans
       ('Mine', 10, 4, 2),           -- Humans
       ('Academy', 10, 1, 2),        -- Humans
       ('University', 10, 1, 2),     -- Humans
       ('Laboratory', 10, 1, 2),     -- Humans
       ('Barracks', 10, 2, 2),       -- Humans
       ('Range', 10, 2, 2),          -- Humans
       ('Stables', 10, 2, 2),        -- Humans
       ('Workshop', 10, 2, 2),       -- Humans
       ('Mage Tower', 10, 1, 2),     -- Humans
       ('Walls', 10, 1, 2),          -- Humans
       ('Church', 10, 2, 2),         -- Humans
       ('Monument', 10, 1, 2),       -- Humans

       ('Stronghold', 10, 1, 3),     -- Orcs
       ('Farm', 10, 4, 3),           -- Orcs
       ('Lumberyard', 10, 4, 3),     -- Orcs
       ('Quarry', 10, 4, 3),         -- Orcs
       ('Mine', 10, 4, 3),           -- Orcs
       ('Academy', 10, 1, 3),        -- Orcs
       ('University', 10, 1, 3),     -- Orcs
       ('Laboratory', 10, 1, 3),     -- Orcs
       ('Barracks', 10, 2, 3),       -- Orcs
       ('Range', 10, 2, 3),          -- Orcs
       ('Stables', 10, 2, 3),        -- Orcs
       ('Workshop', 10, 2, 3),       -- Orcs
       ('The Circle', 10, 1, 3),     -- Orcs (Mage Tower)
       ('Walls', 10, 1, 3),          -- Orcs
       ('Shamanic Altar', 10, 2, 3), -- Orcs (Church)
       ('Monument', 10, 1, 3),       -- Orcs

       ('Tree of Life', 10, 1, 4),   -- Elves
       ('Farm', 10, 4, 4),           -- Elves
       ('Lumberyard', 10, 4, 4),     -- Elves
       ('Quarry', 10, 4, 4),         -- Elves
       ('Mine', 10, 4, 4),           -- Elves
       ('Academy', 10, 1, 4),        -- Elves
       ('University', 10, 1, 4),     -- Elves
       ('Laboratory', 10, 1, 4),     -- Elves
       ('Barracks', 10, 2, 4),       -- Elves
       ('Range', 10, 2, 4),          -- Elves
       ('Stables', 10, 2, 4),        -- Elves
       ('Workshop', 10, 2, 4),       -- Elves
       ('Arcanum', 10, 1, 4),        -- Elves (Mage Tower)
       ('Walls', 10, 1, 4),          -- Elves
       ('Shrine', 10, 2, 4),         -- Elves (Church)
       ('Monument', 10, 1, 4),       -- Elves

       ('Hall of Thanes', 10, 1, 5), -- Dwarves
       ('Farm', 10, 4, 5),           -- Dwarves
       ('Lumberyard', 10, 4, 5),     -- Dwarves
       ('Quarry', 10, 4, 5),         -- Dwarves
       ('Mine', 10, 4, 5),           -- Dwarves
       ('Academy', 10, 1, 5),        -- Dwarves
       ('University', 10, 1, 5),     -- Dwarves
       ('Laboratory', 10, 1, 5),     -- Dwarves
       ('Barracks', 10, 2, 5),       -- Dwarves
       ('Range', 10, 2, 5),          -- Dwarves
       ('Stables', 10, 2, 5),        -- Dwarves
       ('Workshop', 10, 2, 5),       -- Dwarves
       ('Hall of Runes', 10, 1, 5),  -- Dwarves (Mage Tower)
       ('Walls', 10, 1, 5),          -- Dwarves
       ('Temple', 10, 2, 5),         -- Dwarves (Church)
       ('Monument', 10, 1, 5),       -- Dwarves

       ('The Big Shack', 10, 1, 6),  -- Goblins
       ('Farm', 10, 4, 6),           -- Goblins
       ('Lumberyard', 10, 4, 6),     -- Goblins
       ('Quarry', 10, 4, 6),         -- Goblins
       ('Mine', 10, 4, 6),           -- Goblins
       ('Cadet School', 10, 1, 6),   -- Goblins
       ('Brainery', 10, 1, 6),       -- Goblins
       ('Laboratory', 10, 1, 6),     -- Goblins
       ('Barracks', 10, 2, 6),       -- Goblins
       ('Range', 10, 2, 6),          -- Goblins
       ('Stables', 10, 2, 6),        -- Goblins
       ('Workshop', 10, 2, 6),       -- Goblins
       ('Mana Den', 10, 1, 6),       -- Goblins (Mage Tower)
       ('Walls', 10, 1, 6),          -- Goblins
       ('Speaker''s Hut', 10, 2, 6), -- Goblins (Church)
       ('Monument', 10, 1, 6),       -- Dwarves

       ('Guild Hall', 10, 1, 1),     -- Neutral
       ('Market', 10, 1, 1),         -- Neutral
       ('Embassy', 10, 1, 1); -- Neutral