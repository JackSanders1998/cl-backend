-- Add migration script here
ALTER TABLE routes ADD COLUMN description text;

ALTER TABLE routes DROP COLUMN pointer;

ALTER TABLE ticks ADD COLUMN isLappedWith uuid;