-- Add migration script here
ALTER TABLE ticks ADD COLUMN sesh_id uuid REFERENCES seshes(sesh_id);