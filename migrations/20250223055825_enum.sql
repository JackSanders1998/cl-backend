-- Add migration script here
DO $$ BEGIN
    CREATE TYPE environment_type AS ENUM ('gym', 'outdoor');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;


 ALTER TABLE locations
    Alter COLUMN environment TYPE environment_type
    USING CAST(environment AS environment_type);