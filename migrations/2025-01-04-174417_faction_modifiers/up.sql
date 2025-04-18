-- Seed default faction modifiers
INSERT INTO modifiers (name, description, modifier_type, target_type, target_resource, stacking_group)
VALUES
    -- Human faction modifiers
    ('human_wood_production',  'Human wood production bonus',        'percentage', 'resource', 'wood',  'faction_wood'    ),
    ('human_cavalry_training', 'Human cavalry training speed bonus', 'percentage', 'training', NULL,    'faction_training'),
    ('human_cavalry_combat',   'Human cavalry combat effectiveness', 'percentage', 'combat',   NULL,    'faction_combat'  ),

    -- Orc faction modifiers
    ('orc_stone_production',   'Orc stone production bonus',         'percentage', 'resource', 'stone', 'faction_stone'   ),
    ('orc_infantry_training',  'Orc infantry training speed bonus',  'percentage', 'training', NULL,    'faction_training'),
    ('orc_infantry_combat',    'Orc infantry combat effectiveness',  'percentage', 'combat',   NULL,    'faction_combat'  ),

    -- Elf faction modifiers
    ('elf_food_production',    'Elf food production bonus',          'percentage', 'resource', 'food',  'faction_food'    ),
    ('elf_ranged_training',    'Elf ranged training speed bonus',    'percentage', 'training', NULL,    'faction_training'),
    ('elf_ranged_combat',      'Elf ranged combat effectiveness',    'percentage', 'combat',   NULL,    'faction_combat'  );

-- Update the manage_faction_modifiers function with correct magnitudes
CREATE OR REPLACE FUNCTION manage_faction_modifiers()
    RETURNS TRIGGER AS
$$
DECLARE
    old_faction_code text;
    new_faction_code text;
    change_time      timestamptz;
    modified_rows    integer;
BEGIN
    change_time := now();

    -- Handle different trigger events (INSERT or UPDATE)
    IF TG_OP = 'INSERT' THEN
        new_faction_code := NEW.faction;
        old_faction_code := NULL;
    ELSE
        new_faction_code := NEW.faction;
        old_faction_code := OLD.faction;
    END IF;

    IF new_faction_code = 'neutral' AND TG_OP = 'INSERT' THEN
        RETURN NEW;
    END IF;

    -- Remove old faction modifiers if this is an UPDATE
    IF old_faction_code IS NOT NULL THEN
        DELETE
        FROM user_active_modifiers
        WHERE user_id = OLD.id
          AND modifier_id IN (SELECT id
                              FROM modifiers
                              WHERE name LIKE old_faction_code || '_%')
          AND source_type = 'faction';

        -- Record changes in modifier history
        INSERT INTO modifier_history (user_id, modifier_id, action_type, magnitude, source_type, occurred_at, reason)
        SELECT OLD.id,
               m.id,
               'removed',
               CASE
                   WHEN m.name = 'human_wood_production' THEN 1.15
                   WHEN m.name = 'human_cavalry_training' THEN 1.15
                   WHEN m.name = 'human_cavalry_combat' THEN 1.15
                   WHEN m.name = 'orc_stone_production' THEN 1.15
                   WHEN m.name = 'orc_infantry_training' THEN 1.15
                   WHEN m.name = 'orc_infantry_combat' THEN 1.15
                   WHEN m.name = 'elf_food_production' THEN 1.15
                   WHEN m.name = 'elf_ranged_training' THEN 1.15
                   WHEN m.name = 'elf_ranged_combat' THEN 1.15
                   ELSE 1.0
                   END,
               'faction',
               change_time,
               'Faction change'
        FROM modifiers m
        WHERE m.name LIKE old_faction_code || '_%';
    END IF;

    -- Apply new faction modifiers
    INSERT INTO user_active_modifiers (user_id, modifier_id, magnitude, source_type, source_id)
    SELECT NEW.id,
           m.id,
           CASE
               -- Human modifiers (+15% wood, +15% cavalry training, +25% cavalry combat)
               WHEN m.name = 'human_wood_production' THEN 1.15
               WHEN m.name = 'human_cavalry_training' THEN 1.15
               WHEN m.name = 'human_cavalry_combat' THEN 1.15

               -- Orc modifiers (+15% stone, +15% infantry training, +15% infantry combat)
               WHEN m.name = 'orc_stone_production' THEN 1.15
               WHEN m.name = 'orc_infantry_training' THEN 1.15
               WHEN m.name = 'orc_infantry_combat' THEN 1.15

               -- Elf modifiers (+15% food, +15% ranged training, +15% ranged combat)
               WHEN m.name = 'elf_food_production' THEN 1.15
               WHEN m.name = 'elf_ranged_training' THEN 1.15
               WHEN m.name = 'elf_ranged_combat' THEN 1.15

               ELSE 1.0
               END,
           'faction',
           NULL
    FROM modifiers m
    WHERE m.name LIKE new_faction_code || '_%';

    -- Get the number of rows that were just inserted
    GET DIAGNOSTICS modified_rows = ROW_COUNT;

    -- If no rows were inserted, something went wrong (no modifiers found for this faction)
    IF modified_rows = 0 THEN
        RAISE EXCEPTION 'No modifiers found for faction %', new_faction_code;
    END IF;

    -- Record changes in modifier history
    INSERT INTO modifier_history (user_id, modifier_id, action_type, magnitude, source_type, occurred_at, reason)
    SELECT NEW.id,
           m.id,
           'applied',
           CASE
               WHEN m.name = 'human_wood_production' THEN 1.15
               WHEN m.name = 'human_cavalry_training' THEN 1.15
               WHEN m.name = 'human_cavalry_combat' THEN 1.15
               WHEN m.name = 'orc_stone_production' THEN 1.15
               WHEN m.name = 'orc_infantry_training' THEN 1.15
               WHEN m.name = 'orc_infantry_combat' THEN 1.15
               WHEN m.name = 'elf_food_production' THEN 1.15
               WHEN m.name = 'elf_ranged_training' THEN 1.15
               WHEN m.name = 'elf_ranged_combat' THEN 1.15
               ELSE 1.0
               END,
           'faction',
           change_time,
           CASE
               WHEN TG_OP = 'INSERT' THEN 'Initial faction modifiers'
               ELSE 'Faction change'
               END
    FROM modifiers m
    WHERE m.name LIKE new_faction_code || '_%';

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers for user faction changes
CREATE TRIGGER user_faction_modifiers_insert
    AFTER INSERT
    ON users
    FOR EACH ROW
EXECUTE FUNCTION manage_faction_modifiers();

CREATE TRIGGER user_faction_modifiers_update
    AFTER UPDATE OF faction
    ON users
    FOR EACH ROW
    WHEN (OLD.faction IS DISTINCT FROM NEW.faction)
EXECUTE FUNCTION manage_faction_modifiers();
