-- Add migration script here
SET timezone = 'UTC';

ALTER TABLE locations
    ALTER created_at TYPE timestamptz,
    ALTER updated_at TYPE timestamptz;

ALTER TABLE climbs
    ALTER created_at TYPE timestamptz,
    ALTER updated_at TYPE timestamptz;

ALTER TABLE preferences
    ALTER created_at TYPE timestamptz,
    ALTER updated_at TYPE timestamptz;

ALTER TABLE seshes
    ALTER created_at TYPE timestamptz,
    ALTER updated_at TYPE timestamptz,
    ALTER "start" TYPE timestamptz,
    ALTER "end" TYPE timestamptz;
