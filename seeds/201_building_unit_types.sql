-- =========================================
-- Building Unit Types Seed
-- =========================================
-- Maps which buildings can train which unit types.
-- These mappings apply across all factions.
--
-- Building -> Unit Type Mappings:
--   - Barracks -> Infantry
--   - Range    -> Ranged
--   - Stables  -> Cavalry
--   - Workshop -> Artillery
--
-- AIDEV-NOTE: These mappings are the same for all factions. Faction-specific
-- training bonuses are applied via the modifier system at runtime.

-- ===== BUILDING UNIT TYPE MAPPINGS =====

-- Barracks trains Infantry
INSERT INTO building_unit_type (building_id, unit_type)
SELECT b.id, 'infantry'::unit_type
FROM building b
WHERE b.name = 'Barracks'
ON CONFLICT (building_id, unit_type) DO NOTHING;

-- Range trains Ranged
INSERT INTO building_unit_type (building_id, unit_type)
SELECT b.id, 'ranged'::unit_type
FROM building b
WHERE b.name = 'Range'
ON CONFLICT (building_id, unit_type) DO NOTHING;

-- Stables trains Cavalry
INSERT INTO building_unit_type (building_id, unit_type)
SELECT b.id, 'cavalry'::unit_type
FROM building b
WHERE b.name = 'Stables'
ON CONFLICT (building_id, unit_type) DO NOTHING;

-- Workshop trains Artillery
INSERT INTO building_unit_type (building_id, unit_type)
SELECT b.id, 'artillery'::unit_type
FROM building b
WHERE b.name = 'Workshop'
ON CONFLICT (building_id, unit_type) DO NOTHING;
