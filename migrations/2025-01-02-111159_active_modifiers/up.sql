CREATE TYPE modifier_source_type AS ENUM ('faction', 'item', 'skill', 'research', 'event');

CREATE TABLE IF NOT EXISTS user_active_modifiers
(
    id          UUID                 NOT NULL DEFAULT generate_ulid(),
    user_id     UUID                 NOT NULL,
    modifier_id UUID                 NOT NULL,
    magnitude   DECIMAL(10, 4)       NOT NULL,
    started_at  TIMESTAMPTZ          NOT NULL DEFAULT now(),
    expires_at  TIMESTAMPTZ          NULL,
    source_type modifier_source_type NOT NULL,
    source_id   UUID                 NULL,
    created_at  TIMESTAMPTZ          NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ          NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (modifier_id) REFERENCES modifiers (id) ON DELETE CASCADE,

    -- Ensure expires_at is after started_at if set
    CONSTRAINT valid_timespan CHECK (expires_at IS NULL OR expires_at > started_at)
);

-- Create composite index for efficient queries
CREATE INDEX user_active_modifiers_user_expires_idx ON user_active_modifiers (user_id, expires_at);
CREATE INDEX user_active_modifiers_modifier_idx ON user_active_modifiers (modifier_id);
CREATE INDEX user_active_modifiers_source_idx ON user_active_modifiers (source_type, source_id);

-- Add trigger for updating timestamp
CREATE TRIGGER set_user_active_modifiers_updated_at
    BEFORE UPDATE
    ON user_active_modifiers
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();