CREATE FUNCTION set_current_timestamp_updated_at()
    RETURNS TRIGGER AS
$$
DECLARE
    _new record;
BEGIN
    _new := NEW;
    _new."updated_at" = now();
    RETURN _new;
END;
$$ LANGUAGE plpgsql;