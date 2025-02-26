-- Add migration script here
DO $$ BEGIN
    CREATE TYPE discipline_type AS ENUM ('boulder', 'sport', 'top_rope');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;


 ALTER TABLE ticks
    ALTER COLUMN discipline TYPE discipline_type
    USING CAST(discipline AS discipline_type);

 ALTER TABLE routes
    ALTER COLUMN disciplines DROP DEFAULT,
    ALTER COLUMN disciplines TYPE discipline_type[]
    USING disciplines::discipline_type[];