CREATE TABLE unit_cost
(

    id         UUID          NOT NULL DEFAULT uuidv7(),
    unit_id    UUID          NOT NULL,
    resource   resource_type NOT NULL,
    amount     INTEGER       NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ   NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ   NOT NULL DEFAULT now(),

    PRIMARY KEY (id),
    FOREIGN KEY (unit_id) REFERENCES unit (id) ON DELETE CASCADE,
    CONSTRAINT unit_resource UNIQUE (unit_id, resource)
);

CREATE TRIGGER set_unit_cost_updated_at
    BEFORE UPDATE
    ON unit_cost
    FOR EACH ROW
EXECUTE FUNCTION set_current_timestamp_updated_at();
