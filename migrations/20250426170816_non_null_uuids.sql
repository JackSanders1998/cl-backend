-- Add migration script here
ALTER TABLE workouts
    ALTER sesh_id SET NOT NULL,
    ALTER user_id SET NOT NULL,
    ALTER log SET NOT NULL;