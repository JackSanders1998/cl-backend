-- Add migration script here
DO $$ BEGIN
    CREATE TYPE attempt AS ENUM ('onsight', 'flash', 'redpoint', 'fall');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

ALTER TABLE climbs
    ADD COLUMN notes text,                      -- jot down notes on the climb
    ADD COLUMN pointer uuid,                    -- reference another climb on this table
    ADD COLUMN attempt attempt NOT NULL;        -- how did the climb go?