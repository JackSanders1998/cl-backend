-- Add migration script here
CREATE OR replace function set_updated_at() RETURNS trigger
  LANGUAGE plpgsql
  AS $$
BEGIN
  new.updated_at := current_timestamp;
  return new;
END;
$$;

CREATE TABLE if NOT EXISTS locations (
  location_id   uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  user_id       text,   -- clerk
  name          text NOT NULL,
  environment   text NOT NULL,  -- GYM, OUTDOOR, etc...
  created_at    TIMESTAMP NOT NULL DEFAULT now(),
  updated_at    TIMESTAMP NOT NULL DEFAULT now()
);
CREATE trigger updated_at before UPDATE ON locations FOR each row EXECUTE PROCEDURE set_updated_at();

CREATE TABLE if NOT EXISTS seshes (
  sesh_id       uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  user_id       text,   -- clerk
  location_id   uuid NOT NULL REFERENCES locations(location_id) ON DELETE CASCADE,
  notes         text,
  start         TIMESTAMP NOT NULL DEFAULT now(),
  "end"         TIMESTAMP,
  created_at    TIMESTAMP NOT NULL DEFAULT now(),
  updated_at    TIMESTAMP NOT NULL DEFAULT now()
);
CREATE trigger updated_at before UPDATE ON seshes FOR each row EXECUTE PROCEDURE set_updated_at();

CREATE TABLE if NOT EXISTS climbs (
  climb_id      uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  sesh_id       uuid NOT NULL REFERENCES seshes(sesh_id) ON DELETE CASCADE,
  type          text NOT NULL,  -- BOULDER | SPORT
  style         text NOT NULL,  -- TR | Lead
  scale         text NOT NULL,  -- Font, YDS, etc...
  grade         text NOT NULL,  -- v5, 5.11b, etc...
  created_at    TIMESTAMP NOT NULL DEFAULT now(),
  updated_at    TIMESTAMP NOT NULL DEFAULT now()
);
CREATE trigger updated_at before UPDATE ON climbs FOR each row EXECUTE PROCEDURE set_updated_at();

CREATE TABLE if NOT EXISTS preferences (
  preference_id uuid primary key not null default gen_random_uuid(),
  user_id       text NOT NULL, -- clerk
  boulder_scale text NOT NULL DEFAULT 'USA',    -- USA, FONT, etc...
  sport_scale   text NOT NULL DEFAULT 'YDS',    -- YDS, FRENCH, etc...
  color_scheme  text NOT NULL DEFAULT 'AUTO',   -- Light, Dark Auto...
  theme         text NOT NULL DEFAULT 'BRAND',  -- Brand, etc..
  created_at    TIMESTAMP NOT NULL DEFAULT now(),
  updated_at    TIMESTAMP NOT NULL DEFAULT now()
);
CREATE trigger updated_at before UPDATE ON preferences FOR each row EXECUTE PROCEDURE set_updated_at();
