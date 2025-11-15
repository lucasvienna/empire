CREATE TABLE building_resource
(
    id             UUID        NOT NULL DEFAULT generate_ulid(),
    building_id    INTEGER     NOT NULL,
    building_level INTEGER     NOT NULL,
    -- These are all in resources per hour
    population     BIGINT      NOT NULL DEFAULT 0,
    food           BIGINT      NOT NULL DEFAULT 0,
    wood           BIGINT      NOT NULL DEFAULT 0,
    stone          BIGINT      NOT NULL DEFAULT 0,
    gold           BIGINT      NOT NULL DEFAULT 0,
    -- These are storage caps, per building
    food_cap       BIGINT      NOT NULL DEFAULT 0,
    wood_cap       BIGINT      NOT NULL DEFAULT 0,
    stone_cap      BIGINT      NOT NULL DEFAULT 0,
    gold_cap       BIGINT      NOT NULL DEFAULT 0,
    -- These are accumulator caps, per building
    food_acc_cap   BIGINT      NOT NULL DEFAULT 0,
    wood_acc_cap   BIGINT      NOT NULL DEFAULT 0,
    stone_acc_cap  BIGINT      NOT NULL DEFAULT 0,
    gold_acc_cap   BIGINT      NOT NULL DEFAULT 0,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    UNIQUE (building_id, building_level),
    FOREIGN KEY (building_id) REFERENCES building (id) ON DELETE CASCADE,
    FOREIGN KEY (building_id, building_level) REFERENCES building_level (building_id, level) ON DELETE CASCADE
);

CREATE TRIGGER set_building_resource_updated_at
    BEFORE UPDATE
    ON building_resource
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();
