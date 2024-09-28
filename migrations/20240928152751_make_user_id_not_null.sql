-- Add migration script here
ALTER TABLE locations ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE preferences ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE seshes ALTER COLUMN user_id SET NOT NULL;
