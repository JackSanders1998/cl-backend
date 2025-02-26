-- Add migration script here
SET timezone = 'UTC';

ALTER TABLE ticks
    ALTER created_at SET DEFAULT now(),
    ALTER updated_at SET DEFAULT now()