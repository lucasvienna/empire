CREATE TYPE modifier_action_type AS ENUM (
    'applied',
    'expired',
    'removed',
    'updated'
    );

CREATE TABLE IF NOT EXISTS modifier_history
(
    id             UUID                 NOT NULL DEFAULT generate_ulid(),
    user_id        UUID                 NOT NULL,
    modifier_id    UUID                 NOT NULL,
    action_type    modifier_action_type NOT NULL,
    occurred_at    TIMESTAMPTZ          NOT NULL DEFAULT now(),
    magnitude      NUMERIC(10, 4)       NOT NULL,
    source_type    modifier_source_type NOT NULL,
    source_id      UUID                 NULL,
    previous_state JSONB                NULL, -- Stores previous state for updates
    reason         TEXT                 NULL, -- Optional reason for the change
    created_at     TIMESTAMPTZ          NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ          NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (modifier_id) REFERENCES modifiers (id) ON DELETE CASCADE
);

-- Indexes for common queries
CREATE INDEX modifier_history_user_idx on modifier_history (user_id, occurred_at);
CREATE INDEX modifier_history_modifier_idx on modifier_history (modifier_id);
CREATE INDEX modifier_history_source_idx on modifier_history (source_type, source_id);
CREATE INDEX modifier_history_action_idx on modifier_history (action_type);

-- Add trigger for updating timestamp
CREATE TRIGGER set_modifier_history_updated_at
    BEFORE UPDATE
    ON modifier_history
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();