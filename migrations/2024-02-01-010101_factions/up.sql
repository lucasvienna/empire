CREATE TYPE faction_code AS ENUM ('neutral', 'human', 'orc', 'elf', 'dwarf', 'goblin');

CREATE TABLE faction
(
    id         faction_code NOT NULL,
    name       TEXT         NOT NULL,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT now(),

    PRIMARY KEY (id)
);

CREATE TRIGGER set_faction_updated_at
    BEFORE UPDATE
    ON faction
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();

INSERT INTO faction (id, name)
VALUES ('neutral', 'Neutral'),
       ('human',   'Humans' ),
       ('orc',     'Orcs'   ),
       ('elf',     'Elves'  ),
       ('dwarf',   'Dwarves'),
       ('goblin',  'Goblins');
