-- Drop dependent table first, then the table it conceptually references.
-- training_queue references building data via building_id, so drop it before building_unit_type.
DROP TABLE training_queue;
DROP TABLE building_unit_type;
DROP TYPE IF EXISTS training_status;
