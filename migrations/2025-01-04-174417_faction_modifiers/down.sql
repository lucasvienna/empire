-- Drop triggers
DROP TRIGGER IF EXISTS user_faction_modifiers_insert ON users;
DROP TRIGGER IF EXISTS user_faction_modifiers_update ON users;

-- Drop function
DROP FUNCTION IF EXISTS manage_faction_modifiers();

-- Remove faction modifiers
DELETE FROM modifiers WHERE name LIKE 'human_%'
    OR name LIKE 'orc_%'
    OR name LIKE 'elf_%'
    OR name LIKE 'dwarf_%'
    OR name LIKE 'goblin_%';