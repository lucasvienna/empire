-- Seed default faction modifiers
INSERT INTO modifiers (name, description, modifier_type, magnitude, target_type, target_resource, stacking_group)
VALUES
    -- Human faction modifiers
    ('human_wood_production',        'Human wood production bonus',         'percentage', 1.15, 'resource', 'wood',       'faction_wood'      ),
    ('human_cavalry_training',       'Human cavalry training speed bonus',  'percentage', 1.15, 'training', NULL,         'faction_training'  ),
    ('human_cavalry_combat',         'Human cavalry combat effectiveness',  'percentage', 1.15, 'combat',   NULL,         'faction_combat'    ),

    -- Orc faction modifiers
    ('orc_stone_production',         'Orc stone production bonus',          'percentage', 1.15, 'resource', 'stone',      'faction_stone'     ),
    ('orc_infantry_training',        'Orc infantry training speed bonus',   'percentage', 1.15, 'training', NULL,         'faction_training'  ),
    ('orc_infantry_combat',          'Orc infantry combat effectiveness',   'percentage', 1.15, 'combat',   NULL,         'faction_combat'    ),

    -- Elf faction modifiers
    ('elf_food_production',          'Elf food production bonus',           'percentage', 1.15, 'resource', 'food',       'faction_food'      ),
    ('elf_ranged_training',          'Elf ranged training speed bonus',     'percentage', 1.15, 'training', NULL,         'faction_training'  ),
    ('elf_ranged_combat',            'Elf ranged combat effectiveness',     'percentage', 1.15, 'combat',   NULL,         'faction_combat'    ),

    -- Dwarf faction modifiers
    ('dwarf_gold_production',        'Dwarf gold production bonus',         'percentage', 1.15, 'resource', 'gold',       'faction_gold'      ),
    ('dwarf_siege_training',         'Dwarf siege training speed bonus',    'percentage', 1.15, 'training', NULL,         'faction_training'  ),
    ('dwarf_siege_combat',           'Dwarf siege combat effectiveness',    'percentage', 1.15, 'combat',   NULL,         'faction_combat'    ),

    -- Goblin faction modifiers
    ('goblin_population_production', 'Goblin population production bonus',  'percentage', 1.20, 'resource', 'population', 'faction_population'),
    ('goblin_general_training',      'Goblin general training speed bonus', 'percentage', 1.20, 'training', NULL,         'faction_training'  );

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

    IF TG_OP = 'INSERT' AND new_faction_code = 'neutral' THEN
        RETURN NEW;
    END IF;

    -- Remove old faction modifiers if this is an UPDATE
    IF old_faction_code IS NOT NULL THEN
        DELETE
        FROM active_modifiers
        WHERE player_id = OLD.id
          AND modifier_id IN (SELECT id FROM modifiers WHERE name LIKE old_faction_code || '_%')
          AND source_type = 'faction';


        -- Record changes in modifier history
        INSERT INTO modifier_history (player_id, modifier_id, action_type, magnitude, source_type, occurred_at, reason)
        SELECT OLD.id,
               m.id,
               'removed',
               m.magnitude,
               'faction',
               change_time,
               'Faction change'
        FROM modifiers m
        WHERE m.name LIKE old_faction_code || '_%';
    END IF;

    -- Apply new faction modifiers
    INSERT INTO active_modifiers (player_id, modifier_id, source_type)
    SELECT NEW.id,
           m.id,
           'faction'
    FROM modifiers m
    WHERE m.name LIKE new_faction_code || '_%';

    -- Get the number of rows that were just inserted
    GET DIAGNOSTICS modified_rows = ROW_COUNT;

    -- If no rows were inserted, something went wrong (no modifiers found for this faction)
    IF modified_rows = 0 THEN
        RAISE EXCEPTION 'No modifiers found for faction %', new_faction_code;
    END IF;

    -- updating the time here ensures that:
    -- a) the timestamp is always the same and
    -- b) applications are always logged after removals
    change_time := clock_timestamp();
    -- Record changes in modifier history
    INSERT INTO modifier_history (player_id, modifier_id, action_type, magnitude, source_type, occurred_at, reason)
    SELECT NEW.id,
           m.id,
           'applied',
           m.magnitude,
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

-- Create triggers for player faction changes
CREATE TRIGGER player_faction_modifiers_insert
    AFTER INSERT
    ON player
    FOR EACH ROW
EXECUTE FUNCTION manage_faction_modifiers();

CREATE TRIGGER player_faction_modifiers_update
    AFTER UPDATE OF faction
    ON player
    FOR EACH ROW
    WHEN (OLD.faction IS DISTINCT FROM NEW.faction)
EXECUTE FUNCTION manage_faction_modifiers();
