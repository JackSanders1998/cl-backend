-- Add migration script here
DO $$ BEGIN
    CREATE TYPE attempt_type AS ENUM ('onsight', 'flash', 'redpoint', 'fall');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;


 ALTER TABLE ticks
    ALTER COLUMN attempt TYPE attempt_type
    USING CAST(attempt AS attempt_type);