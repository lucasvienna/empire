-- =========================================
-- Unit Definitions Seed
-- =========================================
-- Seeds the 4 core unit types for v0.1.0
-- Stats from docs/combat_system.md
--
-- Unit Types:
--   - Infantry: Frontline fighters, balanced ATK/DEF
--   - Ranged: Archers, high ATK, low DEF
--   - Cavalry: Mounted, high mobility, moderate ATK/DEF
--   - Artillery: Siege, very high ATK, very low DEF
--
-- AIDEV-NOTE: These are baseline stats - faction bonuses are applied at runtime via modifiers
-- AIDEV-NOTE: Magical unit type is deferred to v0.2.0+

-- ===== UNIT DEFINITIONS =====

INSERT INTO unit (name, unit_type, base_atk, base_def, base_training_seconds, description)
VALUES ('Infantry',  'infantry',  10, 15, 60,  'Frontline fighters armed with sword and shield. Balanced offense and strong defense.'    ),
       ('Ranged',    'ranged',    15, 5,  90,  'Archers and crossbowmen dealing damage from afar. High attack but fragile.'              ),
       ('Cavalry',   'cavalry',   12, 10, 120, 'Mounted warriors with superior mobility. Fast flankers that excel against siege weapons.'),
       ('Artillery', 'artillery', 20, 3,  180, 'Siege engines and war machines. Devastating firepower but extremely vulnerable.'         )
ON CONFLICT (name) DO NOTHING;

-- ===== UNIT COSTS =====
-- Each unit has resource costs that scale with their power
-- Costs from docs/combat_system.md:
--   Infantry:  Food 20, Wood 10          (Total: 30)
--   Ranged:    Food 15, Wood 20          (Total: 35)
--   Cavalry:   Food 30, Gold 15          (Total: 45)
--   Artillery: Food 25, Wood 15, Stone 20 (Total: 60)

INSERT INTO unit_cost (unit_id, resource, amount)
SELECT u.id, r.resource::resource_type, r.amount
FROM unit u
         CROSS JOIN (VALUES ('Infantry', 'food', 20),
                            ('Infantry', 'wood', 10),
                            ('Ranged', 'food', 15),
                            ('Ranged', 'wood', 20),
                            ('Cavalry', 'food', 30),
                            ('Cavalry', 'gold', 15),
                            ('Artillery', 'food', 25),
                            ('Artillery', 'wood', 15),
                            ('Artillery', 'stone', 20)) AS r(unit_name, resource, amount)
WHERE u.name = r.unit_name
ON CONFLICT (unit_id, resource) DO NOTHING;
