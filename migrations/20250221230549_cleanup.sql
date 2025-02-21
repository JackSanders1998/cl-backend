-- Add migration script here
ALTER TABLE routes rename COLUMN discipline TO disciplines;

ALTER TABLE ticks rename COLUMN style TO discipline;

ALTER TABLE ticks rename COLUMN is_lapped_with TO lap_group;

ALTER TABLE locations DROP COLUMN user_id;