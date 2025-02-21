-- Add migration script here
ALTER TABLE routes ADD COLUMN location_id uuid REFERENCES locations(location_id);