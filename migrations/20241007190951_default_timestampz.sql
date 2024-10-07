-- Add migration script here
SET timezone = 'UTC';

ALTER TABLE locations
    ALTER created_at SET DEFAULT now(),
    ALTER updated_at SET DEFAULT now();

ALTER TABLE climbs
    ALTER created_at SET DEFAULT now(),
    ALTER updated_at SET DEFAULT now();

ALTER TABLE preferences
    ALTER created_at SET DEFAULT now(),
    ALTER updated_at SET DEFAULT now();

ALTER TABLE seshes
    ALTER created_at SET DEFAULT now(),
    ALTER updated_at SET DEFAULT now(),
    ALTER "start" SET DEFAULT now();
