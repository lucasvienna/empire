CREATE TYPE modifier_source_type AS ENUM ('faction', 'item', 'skill', 'research', 'event');

CREATE TABLE IF NOT EXISTS active_modifiers
(
    id          UUID                 NOT NULL DEFAULT generate_ulid(),
    player_id     UUID                 NOT NULL,
    modifier_id UUID                 NOT NULL,
    started_at  TIMESTAMPTZ          NOT NULL DEFAULT now(),
    expires_at  TIMESTAMPTZ          NULL,
    source_type modifier_source_type NOT NULL,
    source_id   UUID                 NULL,
    created_at  TIMESTAMPTZ          NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ          NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (player_id) REFERENCES player (id) ON DELETE CASCADE,
    FOREIGN KEY (modifier_id) REFERENCES modifiers (id) ON DELETE CASCADE,

    -- Ensure expires_at is after started_at if set
    CONSTRAINT valid_timespan CHECK (expires_at IS NULL OR expires_at > started_at)
);

-- Create composite index for efficient queries
CREATE INDEX active_modifiers_player_expires_idx ON active_modifiers (player_id, expires_at);
CREATE INDEX active_modifiers_modifier_idx ON active_modifiers (modifier_id);
CREATE INDEX active_modifiers_source_idx ON active_modifiers (source_type, source_id);

-- Add trigger for updating timestamp
CREATE TRIGGER set_active_modifiers_updated_at
    BEFORE UPDATE
    ON active_modifiers
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();