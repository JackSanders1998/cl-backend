-- Add migration script here
ALTER TABLE ticks ALTER COLUMN sesh_id SET NOT NULL;

ALTER TABLE ticks ADD COLUMN notes TEXT;

ALTER TABLE routes ADD COLUMN author TEXT NOT NULL DEFAULT 'clerk_default';

ALTER TABLE locations ADD COLUMN description text;

ALTER TABLE locations ADD COLUMN author TEXT NOT NULL DEFAULT 'clerk_default';

ALTER TABLE routes DROP COLUMN description;

ALTER TABLE routes ADD COLUMN discipline TEXT[] NOT NULL DEFAULT ARRAY['sport'];