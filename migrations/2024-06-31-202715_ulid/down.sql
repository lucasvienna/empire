-- Drop the generate_ulid function if it exists
DROP FUNCTION IF EXISTS generate_ulid();

-- Drop the pgcrypto extension if it exists
DROP EXTENSION IF EXISTS pgcrypto;