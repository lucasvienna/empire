CREATE TYPE faction_code AS ENUM ('neutral', 'human', 'orc', 'elf', 'dwarf', 'goblin');

CREATE TABLE factions
(
    id   faction_code NOT NULL,
    name TEXT         NOT NULL,

    PRIMARY KEY (id)
);

INSERT INTO factions (id, name)
VALUES ('neutral', 'Neutral'),
       ('human',   'Humans' ),
       ('orc',     'Orcs'   ),
       ('elf',     'Elves'  ),
       ('dwarf',   'Dwarves'),
       ('goblin',  'Goblins');
