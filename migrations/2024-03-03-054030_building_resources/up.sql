CREATE TABLE building_resources
(
    id             UUID        NOT NULL DEFAULT generate_ulid(),
    building_id    INTEGER     NOT NULL,
    building_level INTEGER     NOT NULL,
    -- These are all in resources per hour
    population     INTEGER     NOT NULL DEFAULT 0,
    food           INTEGER     NOT NULL DEFAULT 0,
    wood           INTEGER     NOT NULL DEFAULT 0,
    stone          INTEGER     NOT NULL DEFAULT 0,
    gold           INTEGER     NOT NULL DEFAULT 0,
    -- These are storage caps, per building
    food_cap       INTEGER     NOT NULL DEFAULT 0,
    wood_cap       INTEGER     NOT NULL DEFAULT 0,
    stone_cap      INTEGER     NOT NULL DEFAULT 0,
    gold_cap       INTEGER     NOT NULL DEFAULT 0,
    -- These are accumulator caps, per building
    food_acc_cap   INTEGER     NOT NULL DEFAULT 0,
    wood_acc_cap   INTEGER     NOT NULL DEFAULT 0,
    stone_acc_cap  INTEGER     NOT NULL DEFAULT 0,
    gold_acc_cap   INTEGER     NOT NULL DEFAULT 0,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (building_id) REFERENCES buildings (id) ON DELETE CASCADE,
    FOREIGN KEY (building_id, building_level) REFERENCES building_levels (building_id, level)
);

CREATE TRIGGER set_building_resources_updated_at
    BEFORE UPDATE
    ON building_resources
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();

INSERT INTO building_resources (building_id, building_level, population, food_cap, wood_cap, stone_cap, gold_cap)
VALUES
-- Keep
(1,  0,  10,  240,    240,    160,    160   ), -- Keep 0: Food/Wood: 240; Stone/Gold: 160
(1,  1,  20,  640,    640,    427,    427   ), -- Keep 1: Food/Wood: 640; Stone/Gold: ~427
(1,  2,  30,  2880,   2880,   1920,   1920  ), -- Keep 2: Food/Wood: 2880; Stone/Gold: 1920
(1,  3,  40,  3840,   3840,   2560,   2560  ), -- Keep 3: Food/Wood: 3840; Stone/Gold: 2560
(1,  4,  50,  9600,   9600,   6400,   6400  ), -- Keep 4: Food/Wood: 9600; Stone/Gold: 6400
(1,  5,  60,  16800,  16800,  10000,  10000 ), -- Keep 5: Food/Wood: 16800; Stone/Gold: 10000
(1,  6,  75,  25920,  25920,  14400,  14400 ), -- Keep 6: Food/Wood: 25920; Stone/Gold: 14400
(1,  7,  90,  52800,  52800,  28000,  28000 ), -- Keep 7: Food/Wood: 52800; Stone/Gold: 28000
(1,  8,  110, 74880,  74880,  40800,  40800 ), -- Keep 8: Food/Wood: 74880; Stone/Gold: 40800
(1,  9,  130, 122880, 122880, 70400,  70400 ), -- Keep 9: Food/Wood: 122880; Stone/Gold: 70400
(1,  10, 150, 218880, 218880, 124800, 124800), -- Keep 10: Food/Wood: 218880; Stone/Gold: 124800
(18, 0,  10,  240,    240,    160,    160   ), -- Stronghold 0: Food/Wood: 240; Stone/Gold: 160
(18, 1,  20,  640,    640,    427,    427   ), -- Stronghold 1: Food/Wood: 640; Stone/Gold: ~427
(18, 2,  30,  2880,   2880,   1920,   1920  ), -- Stronghold 2: Food/Wood: 2880; Stone/Gold: 1920
(18, 3,  40,  3840,   3840,   2560,   2560  ), -- Stronghold 3: Food/Wood: 3840; Stone/Gold: 2560
(18, 4,  50,  9600,   9600,   6400,   6400  ), -- Stronghold 4: Food/Wood: 9600; Stone/Gold: 6400
(18, 5,  60,  16800,  16800,  10000,  10000 ), -- Stronghold 5: Food/Wood: 16800; Stone/Gold: 10000
(18, 6,  75,  25920,  25920,  14400,  14400 ), -- Stronghold 6: Food/Wood: 25920; Stone/Gold: 14400
(18, 7,  90,  52800,  52800,  28000,  28000 ), -- Stronghold 7: Food/Wood: 52800; Stone/Gold: 28000
(18, 8,  110, 74880,  74880,  40800,  40800 ), -- Stronghold 8: Food/Wood: 74880; Stone/Gold: 40800
(18, 9,  130, 122880, 122880, 70400,  70400 ), -- Stronghold 9: Food/Wood: 122880; Stone/Gold: 70400
(18, 10, 150, 218880, 218880, 124800, 124800), -- Stronghold 10: Food/Wood: 218880; Stone/Gold: 124800
-- Warehouse
(2,  0,  0,   480,    480,    320,    320   ), -- Warehouse 0: Food/Wood: 480; Stone/Gold: 320
(2,  1,  0,   1280,   1280,   853,    853   ), -- Warehouse 1: Food/Wood: 1280; Stone/Gold: ~853
(2,  2,  0,   5760,   5760,   3840,   3840  ), -- Warehouse 2: Food/Wood: 5760; Stone/Gold: 3840
(2,  3,  0,   7680,   7680,   5120,   5120  ), -- Warehouse 3: Food/Wood: 7680; Stone/Gold: 5120
(2,  4,  0,   19200,  19200,  12800,  12800 ), -- Warehouse 4: Food/Wood: 19200; Stone/Gold: 12800
(2,  5,  0,   33600,  33600,  20000,  20000 ), -- Warehouse 5: Food/Wood: 33600; Stone/Gold: 20000
(2,  6,  0,   51840,  51840,  28800,  28800 ), -- Warehouse 6: Food/Wood: 51840; Stone/Gold: 28800
(2,  7,  0,   105600, 105600, 56000,  56000 ), -- Warehouse 7: Food/Wood: 105600; Stone/Gold: 56000
(2,  8,  0,   149760, 149760, 81600,  81600 ), -- Warehouse 8: Food/Wood: 149760; Stone/Gold: 81600
(2,  9,  0,   245760, 245760, 140800, 140800), -- Warehouse 9: Food/Wood: 245760; Stone/Gold: 140800
(2,  10, 0,   437760, 437760, 249600, 249600), -- Warehouse 10: Food/Wood: 437760; Stone/Gold: 249600
(19, 0,  0,   480,    480,    320,    320   ), -- Warehouse 0: Food/Wood: 480; Stone/Gold: 320
(19, 1,  0,   1280,   1280,   853,    853   ), -- Warehouse 1: Food/Wood: 1280; Stone/Gold: ~853
(19, 2,  0,   5760,   5760,   3840,   3840  ), -- Warehouse 2: Food/Wood: 5760; Stone/Gold: 3840
(19, 3,  0,   7680,   7680,   5120,   5120  ), -- Warehouse 3: Food/Wood: 7680; Stone/Gold: 5120
(19, 4,  0,   19200,  19200,  12800,  12800 ), -- Warehouse 4: Food/Wood: 19200; Stone/Gold: 12800
(19, 5,  0,   33600,  33600,  20000,  20000 ), -- Warehouse 5: Food/Wood: 33600; Stone/Gold: 20000
(19, 6,  0,   51840,  51840,  28800,  28800 ), -- Warehouse 6: Food/Wood: 51840; Stone/Gold: 28800
(19, 7,  0,   105600, 105600, 56000,  56000 ), -- Warehouse 7: Food/Wood: 105600; Stone/Gold: 56000
(19, 8,  0,   149760, 149760, 81600,  81600 ), -- Warehouse 8: Food/Wood: 149760; Stone/Gold: 81600
(19, 9,  0,   245760, 245760, 140800, 140800), -- Warehouse 9: Food/Wood: 245760; Stone/Gold: 140800
(19, 10, 0,   437760, 437760, 249600, 249600) -- Warehouse 10: Food/Wood: 437760; Stone/Gold: 249600
;

-- Farm
INSERT INTO building_resources (building_id, building_level, food, food_acc_cap)
VALUES (3,  0,  120,  360  ), -- Farm 0: rate=120, capacity=360
       (3,  1,  240,  960  ), -- Farm 1: rate=240, capacity=960
       (3,  2,  360,  2160 ), -- Farm 2: rate=360, capacity=2160
       (3,  3,  480,  2880 ), -- Farm 3: rate=480, capacity=2880
       (3,  4,  600,  4800 ), -- Farm 4: rate=600, capacity=4800
       (3,  5,  840,  8400 ), -- Farm 5: rate=840, capacity=8400
       (3,  6,  1080, 12960), -- Farm 6: rate=1080, capacity=12960
       (3,  7,  1320, 19800), -- Farm 7: rate=1320, capacity=19800
       (3,  8,  1560, 28080), -- Farm 8: rate=1560, capacity=28080
       (3,  9,  1920, 46080), -- Farm 9: rate=1920, capacity=46080
       (3,  10, 2280, 82080), -- Farm 10: rate=2280, capacity=82080
       (20, 0,  120,  360  ), -- Farm 0: rate=120, capacity=360
       (20, 1,  240,  960  ), -- Farm 1: rate=240, capacity=960
       (20, 2,  360,  2160 ), -- Farm 2: rate=360, capacity=2160
       (20, 3,  480,  2880 ), -- Farm 3: rate=480, capacity=2880
       (20, 4,  600,  4800 ), -- Farm 4: rate=600, capacity=4800
       (20, 5,  840,  8400 ), -- Farm 5: rate=840, capacity=8400
       (20, 6,  1080, 12960), -- Farm 6: rate=1080, capacity=12960
       (20, 7,  1320, 19800), -- Farm 7: rate=1320, capacity=19800
       (20, 8,  1560, 28080), -- Farm 8: rate=1560, capacity=28080
       (20, 9,  1920, 46080), -- Farm 9: rate=1920, capacity=46080
       (20, 10, 2280, 82080) -- Farm 10: rate=2280, capacity=82080
;

-- Lumberyard
INSERT INTO building_resources (building_id, building_level, wood, wood_acc_cap)
VALUES (4,  0,  120,  360  ), -- Lumberyard 0: wood rate=120, wood capacity=360
       (4,  1,  240,  960  ), -- Lumberyard 1: wood rate=240, wood capacity=960
       (4,  2,  360,  2160 ), -- Lumberyard 2: wood rate=360, wood capacity=2160
       (4,  3,  480,  2880 ), -- Lumberyard 3: wood rate=480, wood capacity=2880
       (4,  4,  600,  4800 ), -- Lumberyard 4: wood rate=600, wood capacity=4800
       (4,  5,  840,  8400 ), -- Lumberyard 5: wood rate=840, wood capacity=8400
       (4,  6,  1080, 12960), -- Lumberyard 6: wood rate=1080, wood capacity=12960
       (4,  7,  1320, 19800), -- Lumberyard 7: wood rate=1320, wood capacity=19800
       (4,  8,  1560, 28080), -- Lumberyard 8: wood rate=1560, wood capacity=28080
       (4,  9,  1920, 46080), -- Lumberyard 9: wood rate=1920, wood capacity=46080
       (4,  10, 2280, 82080), -- Lumberyard 10: wood rate=2280, wood capacity=82080
       (21, 0,  120,  360  ), -- Lumberyard 0: wood rate=120, wood capacity=360
       (21, 1,  240,  960  ), -- Lumberyard 1: wood rate=240, wood capacity=960
       (21, 2,  360,  2160 ), -- Lumberyard 2: wood rate=360, wood capacity=2160
       (21, 3,  480,  2880 ), -- Lumberyard 3: wood rate=480, wood capacity=2880
       (21, 4,  600,  4800 ), -- Lumberyard 4: wood rate=600, wood capacity=4800
       (21, 5,  840,  8400 ), -- Lumberyard 5: wood rate=840, wood capacity=8400
       (21, 6,  1080, 12960), -- Lumberyard 6: wood rate=1080, wood capacity=12960
       (21, 7,  1320, 19800), -- Lumberyard 7: wood rate=1320, wood capacity=19800
       (21, 8,  1560, 28080), -- Lumberyard 8: wood rate=1560, wood capacity=28080
       (21, 9,  1920, 46080), -- Lumberyard 9: wood rate=1920, wood capacity=46080
       (21, 10, 2280, 82080) -- Lumberyard 10: wood rate=2280, wood capacity=82080
;

-- Quarry
INSERT INTO building_resources (building_id, building_level, wood, wood_acc_cap)
VALUES (5,  0,  80,   240  ), -- Quarry 0: stone rate=80,  stone capacity=240
       (5,  1,  160,  640  ), -- Quarry 1: stone rate=160, stone capacity=640
       (5,  2,  240,  1440 ), -- Quarry 2: stone rate=240, stone capacity=1440
       (5,  3,  320,  1920 ), -- Quarry 3: stone rate=320, stone capacity=1920
       (5,  4,  400,  3200 ), -- Quarry 4: stone rate=400, stone capacity=3200
       (5,  5,  500,  5000 ), -- Quarry 5: stone rate=500, stone capacity=5000
       (5,  6,  600,  7200 ), -- Quarry 6: stone rate=600, stone capacity=7200
       (5,  7,  700,  10500), -- Quarry 7: stone rate=700, stone capacity=10500
       (5,  8,  850,  15300), -- Quarry 8: stone rate=850, stone capacity=15300
       (5,  9,  1100, 26400), -- Quarry 9: stone rate=1100, stone capacity=26400
       (5,  10, 1300, 46800), -- Quarry 10: stone rate=1300, stone capacity=46800
       (22, 0,  80,   240  ), -- Quarry 0: stone rate=80,  stone capacity=240
       (22, 1,  160,  640  ), -- Quarry 1: stone rate=160, stone capacity=640
       (22, 2,  240,  1440 ), -- Quarry 2: stone rate=240, stone capacity=1440
       (22, 3,  320,  1920 ), -- Quarry 3: stone rate=320, stone capacity=1920
       (22, 4,  400,  3200 ), -- Quarry 4: stone rate=400, stone capacity=3200
       (22, 5,  500,  5000 ), -- Quarry 5: stone rate=500, stone capacity=5000
       (22, 6,  600,  7200 ), -- Quarry 6: stone rate=600, stone capacity=7200
       (22, 7,  700,  10500), -- Quarry 7: stone rate=700, stone capacity=10500
       (22, 8,  850,  15300), -- Quarry 8: stone rate=850, stone capacity=15300
       (22, 9,  1100, 26400), -- Quarry 9: stone rate=1100, stone capacity=26400
       (22, 10, 1300, 46800) -- Quarry 10: stone rate=1300, stone capacity=46800
;

-- Mine
INSERT INTO building_resources (building_id, building_level, gold, gold_acc_cap)
VALUES (6,  0,  80,   240  ), -- Mine 0: gold rate=80,  gold capacity=240
       (6,  1,  160,  640  ), -- Mine 1: gold rate=160, gold capacity=640
       (6,  2,  240,  1440 ), -- Mine 2: gold rate=240, gold capacity=1440
       (6,  3,  320,  1920 ), -- Mine 3: gold rate=320, gold capacity=1920
       (6,  4,  400,  3200 ), -- Mine 4: gold rate=400, gold capacity=3200
       (6,  5,  500,  5000 ), -- Mine 5: gold rate=500, gold capacity=5000
       (6,  6,  600,  7200 ), -- Mine 6: gold rate=600, gold capacity=7200
       (6,  7,  700,  10500), -- Mine 7: gold rate=700, gold capacity=10500
       (6,  8,  850,  15300), -- Mine 8: gold rate=850, gold capacity=15300
       (6,  9,  1100, 26400), -- Mine 9: gold rate=1100, gold capacity=26400
       (6,  10, 1300, 46800), -- Mine 10: gold rate=1300, gold capacity=46800
       (23, 0,  80,   240  ), -- Mine 0: gold rate=80,  gold capacity=240
       (23, 1,  160,  640  ), -- Mine 1: gold rate=160, gold capacity=640
       (23, 2,  240,  1440 ), -- Mine 2: gold rate=240, gold capacity=1440
       (23, 3,  320,  1920 ), -- Mine 3: gold rate=320, gold capacity=1920
       (23, 4,  400,  3200 ), -- Mine 4: gold rate=400, gold capacity=3200
       (23, 5,  500,  5000 ), -- Mine 5: gold rate=500, gold capacity=5000
       (23, 6,  600,  7200 ), -- Mine 6: gold rate=600, gold capacity=7200
       (23, 7,  700,  10500), -- Mine 7: gold rate=700, gold capacity=10500
       (23, 8,  850,  15300), -- Mine 8: gold rate=850, gold capacity=15300
       (23, 9,  1100, 26400), -- Mine 9: gold rate=1100, gold capacity=26400
       (23, 10, 1300, 46800) -- Mine 10: gold rate=1300, gold capacity=46800
;
