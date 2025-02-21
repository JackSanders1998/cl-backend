-- Add migration script here
ALTER TABLE ticks RENAME COLUMN isLappedWith TO is_lapped_with;