-- Add migration script here
SET timezone = 'UTC';

ALTER TABLE ticks
    ALTER created_at TYPE timestamptz,
    ALTER updated_at TYPE timestamptz;