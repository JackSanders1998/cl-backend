-- Add migration script here
ALTER TABLE workouts ADD COLUMN sesh_id uuid REFERENCES seshes(sesh_id);