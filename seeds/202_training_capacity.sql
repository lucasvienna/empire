-- Training Capacity Seed Data
-- Sets training_capacity for military buildings based on level progression:
-- Level 0: 0, Level 1-2: 1, Level 3-4: 2, Level 5-6: 3, Level 7-9: 4, Level 10+: 5

UPDATE building_level SET training_capacity = CASE
    WHEN level = 0 THEN 0
    WHEN level BETWEEN 1 AND 2 THEN 1
    WHEN level BETWEEN 3 AND 4 THEN 2
    WHEN level BETWEEN 5 AND 6 THEN 3
    WHEN level BETWEEN 7 AND 9 THEN 4
    ELSE 5
END
WHERE building_id IN (
    -- Human: Barracks, Range, Stables, Workshop
    10, 11, 12, 13,
    -- Orc
    27, 28, 29, 30,
    -- Elf
    44, 45, 46, 47,
    -- Dwarf
    61, 62, 63, 64,
    -- Goblin
    78, 79, 80, 81
);
