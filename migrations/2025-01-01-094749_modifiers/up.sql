CREATE TYPE modifier_type AS ENUM ('percentage', 'flat', 'multiplier');
CREATE TYPE mod_target_type AS ENUM ('resource', 'combat', 'training', 'research');

CREATE TABLE IF NOT EXISTS modifiers
(
    id              UUID            NOT NULL DEFAULT generate_ulid(),
    name            TEXT            NOT NULL,                      -- unique identifier for the modifier
    description     TEXT            NOT NULL,                      -- description of the modifier
    modifier_type   modifier_type   NOT NULL DEFAULT 'percentage', -- type of modifier
    target_type     mod_target_type NOT NULL,                      -- what does this modifier affect?
    target_resource resource_type   NULL,                          -- what resource does this modifier affect?
    stacking_group  TEXT            NULL,                          -- group of modifiers that stack together
    created_at      TIMESTAMPTZ     NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ     NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    UNIQUE (name),
    CHECK (
        (target_type = 'resource' AND target_resource IS NOT NULL)
            OR
        (target_type != 'resource' AND target_resource IS NULL)
        )
);

CREATE TRIGGER set_modifiers_updated_at
    BEFORE UPDATE
    ON modifiers
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();

CREATE INDEX modifiers_target_idx ON modifiers (target_type, target_resource);

CREATE INDEX modifiers_group_idx ON modifiers (stacking_group);
