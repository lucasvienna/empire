CREATE TABLE building_requirement
(
    id                      UUID        NOT NULL DEFAULT uuidv7(),
    building_level_id       UUID        NOT NULL,
    required_building_id    INTEGER     NULL,
    required_building_level INTEGER     NULL,
    required_tech_id        UUID        NULL,
    required_tech_level     INTEGER     NULL,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    UNIQUE (building_level_id, required_building_id, required_building_level),
    UNIQUE (building_level_id, required_tech_id, required_tech_level),
    FOREIGN KEY (building_level_id) REFERENCES building_level (id) ON DELETE CASCADE,
    FOREIGN KEY (required_building_id) REFERENCES building (id) ON DELETE CASCADE,
    CONSTRAINT check_building_xor_tech CHECK (
        (required_building_id IS NOT NULL AND required_tech_id IS NULL
            AND required_building_level IS NOT NULL AND required_tech_level IS NULL)
            OR
        (required_building_id IS NULL AND required_tech_id IS NOT NULL
            AND required_building_level IS NULL AND required_tech_level IS NOT NULL)
        )
);

CREATE TRIGGER set_building_requirement_updated_at
    BEFORE UPDATE
    ON building_requirement
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();