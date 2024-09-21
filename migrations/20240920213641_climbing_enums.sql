-- Add migration script here
CREATE TYPE climb_type AS ENUM ('boulder', 'sport');
CREATE TYPE style AS ENUM ('top_rope', 'lead');
CREATE TYPE scale AS ENUM ('verm', 'font', 'yosemite', 'french');

ALTER TABLE climbs
Alter COLUMN climb_type TYPE climb_type
USING CAST(climb_type AS climb_type),
Alter COLUMN style TYPE style
USING CAST(style AS style),
Alter COLUMN scale TYPE scale
USING CAST(scale AS scale);
