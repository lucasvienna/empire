CREATE TABLE buildings
(
    id         SERIAL       NOT NULL,
    name       TEXT         NOT NULL,
    max_level  INTEGER      NOT NULL,
    max_count  INTEGER      NOT NULL,
    faction    faction_code NOT NULL,
    starter    BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (faction) REFERENCES faction (id)
);

CREATE TRIGGER set_buildings_updated_at
    BEFORE UPDATE
    ON buildings
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();

INSERT INTO buildings (name, max_level, max_count, faction, starter)
VALUES ('Keep',           10, 1, 'human',   TRUE ), -- Humans
       ('Warehouse',      10, 1, 'human',   FALSE), -- Humans
       ('Farm',           10, 4, 'human',   TRUE ), -- Humans
       ('Lumberyard',     10, 4, 'human',   TRUE ), -- Humans
       ('Quarry',         10, 4, 'human',   FALSE), -- Humans
       ('Mine',           10, 4, 'human',   FALSE), -- Humans
       ('Academy',        10, 1, 'human',   FALSE), -- Humans
       ('University',     10, 1, 'human',   FALSE), -- Humans
       ('Laboratory',     10, 1, 'human',   FALSE), -- Humans
       ('Barracks',       10, 2, 'human',   FALSE), -- Humans
       ('Range',          10, 2, 'human',   FALSE), -- Humans
       ('Stables',        10, 2, 'human',   FALSE), -- Humans
       ('Workshop',       10, 2, 'human',   FALSE), -- Humans
       ('Mage Tower',     10, 1, 'human',   FALSE), -- Humans
       ('Walls',          10, 1, 'human',   FALSE), -- Humans
       ('Church',         10, 2, 'human',   FALSE), -- Humans
       ('Monument',       10, 1, 'human',   FALSE), -- Humans

       ('Stronghold',     10, 1, 'orc',     TRUE ), -- Orcs
       ('Warehouse',      10, 1, 'orc',     FALSE), -- Orcs
       ('Farm',           10, 4, 'orc',     TRUE ), -- Orcs
       ('Lumberyard',     10, 4, 'orc',     TRUE ), -- Orcs
       ('Quarry',         10, 4, 'orc',     FALSE), -- Orcs
       ('Mine',           10, 4, 'orc',     FALSE), -- Orcs
       ('Academy',        10, 1, 'orc',     FALSE), -- Orcs
       ('University',     10, 1, 'orc',     FALSE), -- Orcs
       ('Laboratory',     10, 1, 'orc',     FALSE), -- Orcs
       ('Barracks',       10, 2, 'orc',     FALSE), -- Orcs
       ('Range',          10, 2, 'orc',     FALSE), -- Orcs
       ('Stables',        10, 2, 'orc',     FALSE), -- Orcs
       ('Workshop',       10, 2, 'orc',     FALSE), -- Orcs
       ('The Circle',     10, 1, 'orc',     FALSE), -- Orcs (Mage Tower)
       ('Walls',          10, 1, 'orc',     FALSE), -- Orcs
       ('Shamanic Altar', 10, 2, 'orc',     FALSE), -- Orcs (Church)
       ('Monument',       10, 1, 'orc',     FALSE), -- Orcs

       ('Tree of Life',   10, 1, 'elf',     TRUE ), -- Elves
       ('Warehouse',      10, 1, 'elf',     FALSE), -- Elves
       ('Farm',           10, 4, 'elf',     TRUE ), -- Elves
       ('Lumberyard',     10, 4, 'elf',     TRUE ), -- Elves
       ('Quarry',         10, 4, 'elf',     FALSE), -- Elves
       ('Mine',           10, 4, 'elf',     FALSE), -- Elves
       ('Academy',        10, 1, 'elf',     FALSE), -- Elves
       ('University',     10, 1, 'elf',     FALSE), -- Elves
       ('Laboratory',     10, 1, 'elf',     FALSE), -- Elves
       ('Barracks',       10, 2, 'elf',     FALSE), -- Elves
       ('Range',          10, 2, 'elf',     FALSE), -- Elves
       ('Stables',        10, 2, 'elf',     FALSE), -- Elves
       ('Workshop',       10, 2, 'elf',     FALSE), -- Elves
       ('Arcanum',        10, 1, 'elf',     FALSE), -- Elves (Mage Tower)
       ('Walls',          10, 1, 'elf',     FALSE), -- Elves
       ('Shrine',         10, 2, 'elf',     FALSE), -- Elves (Church)
       ('Monument',       10, 1, 'elf',     FALSE), -- Elves

       ('Hall of Thanes', 10, 1, 'dwarf',   TRUE ), -- Dwarves
       ('Warehouse',      10, 1, 'dwarf',   FALSE), -- Humans
       ('Farm',           10, 4, 'dwarf',   TRUE ), -- Dwarves
       ('Lumberyard',     10, 4, 'dwarf',   TRUE ), -- Dwarves
       ('Quarry',         10, 4, 'dwarf',   FALSE), -- Dwarves
       ('Mine',           10, 4, 'dwarf',   FALSE), -- Dwarves
       ('Academy',        10, 1, 'dwarf',   FALSE), -- Dwarves
       ('University',     10, 1, 'dwarf',   FALSE), -- Dwarves
       ('Laboratory',     10, 1, 'dwarf',   FALSE), -- Dwarves
       ('Barracks',       10, 2, 'dwarf',   FALSE), -- Dwarves
       ('Range',          10, 2, 'dwarf',   FALSE), -- Dwarves
       ('Stables',        10, 2, 'dwarf',   FALSE), -- Dwarves
       ('Workshop',       10, 2, 'dwarf',   FALSE), -- Dwarves
       ('Hall of Runes',  10, 1, 'dwarf',   FALSE), -- Dwarves (Mage Tower)
       ('Walls',          10, 1, 'dwarf',   FALSE), -- Dwarves
       ('Temple',         10, 2, 'dwarf',   FALSE), -- Dwarves (Church)
       ('Monument',       10, 1, 'dwarf',   FALSE), -- Dwarves

       ('The Big Shack',  10, 1, 'goblin',  TRUE ), -- Goblins
       ('Warehouse',      10, 1, 'goblin',  FALSE), -- Goblins
       ('Farm',           10, 4, 'goblin',  TRUE ), -- Goblins
       ('Lumberyard',     10, 4, 'goblin',  TRUE ), -- Goblins
       ('Quarry',         10, 4, 'goblin',  FALSE), -- Goblins
       ('Mine',           10, 4, 'goblin',  FALSE), -- Goblins
       ('Cadet School',   10, 1, 'goblin',  FALSE), -- Goblins
       ('Brainery',       10, 1, 'goblin',  FALSE), -- Goblins
       ('Laboratory',     10, 1, 'goblin',  FALSE), -- Goblins
       ('Barracks',       10, 2, 'goblin',  FALSE), -- Goblins
       ('Range',          10, 2, 'goblin',  FALSE), -- Goblins
       ('Stables',        10, 2, 'goblin',  FALSE), -- Goblins
       ('Workshop',       10, 2, 'goblin',  FALSE), -- Goblins
       ('Mana Den',       10, 1, 'goblin',  FALSE), -- Goblins (Mage Tower)
       ('Walls',          10, 1, 'goblin',  FALSE), -- Goblins
       ('Speaker''s Hut', 10, 2, 'goblin',  FALSE), -- Goblins (Church)
       ('Monument',       10, 1, 'goblin',  FALSE), -- Dwarves

       ('Guild Hall',     10, 1, 'neutral', FALSE), -- Neutral
       ('Market',         10, 1, 'neutral', FALSE), -- Neutral
       ('Embassy',        10, 1, 'neutral', FALSE); -- Neutral
