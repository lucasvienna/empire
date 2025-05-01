CREATE TYPE modifier_type AS ENUM ('percentage', 'flat', 'multiplier');
CREATE TYPE modifier_target AS ENUM ('resource', 'combat', 'training', 'research');
CREATE TYPE stacking_behaviour AS ENUM ('additive', 'multiplicative', 'highest');

CREATE TABLE IF NOT EXISTS modifiers
(
    id                 UUID               NOT NULL DEFAULT generate_ulid(),
    name               TEXT               NOT NULL,                      -- unique identifier for the modifier
    description        TEXT               NOT NULL,                      -- description of the modifier
    modifier_type      modifier_type      NOT NULL DEFAULT 'percentage', -- type of modifier
    magnitude          NUMERIC(10, 4)     NOT NULL,                      -- value of the modifier
    target_type        modifier_target    NOT NULL,                      -- what does this modifier affect?
    target_resource    resource_type      NULL,                          -- what resource does this modifier affect?
    stacking_behaviour stacking_behaviour NOT NULL DEFAULT 'additive',   -- how does this modifier stack?
    stacking_group     TEXT               NULL,                          -- group of modifiers for the high_add/high_mult stacking behaviour
    created_at         TIMESTAMPTZ        NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ        NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    UNIQUE (name),
    CONSTRAINT type_resource CHECK (
        (target_type = 'resource' AND target_resource IS NOT NULL)
            OR
        (target_type != 'resource' AND target_resource IS NULL)
        ),
    -- percentage modifiers must be between 0 and 1
    CONSTRAINT magnitude_validity CHECK ((modifier_type = 'percentage' AND magnitude > 0 AND magnitude <= 1)
        -- flat modifiers must be a positive number
        OR (modifier_type = 'flat' AND magnitude >= 0)
        -- multiplier modifiers must be a positive number greater than 1
        OR (modifier_type = 'multiplier' AND magnitude >= 1))
);

CREATE TRIGGER set_modifiers_updated_at
    BEFORE UPDATE
    ON modifiers
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();

CREATE INDEX modifiers_target_idx ON modifiers (target_type, target_resource);

CREATE INDEX modifiers_group_idx ON modifiers (stacking_group);
