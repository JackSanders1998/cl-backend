-- Add migration script here
ALTER TABLE climbs
  RENAME TO routes;

-- ALTER TABLE routes
--   RENAME climb_id notes TO route_id,
--   RENAME COLUMN notes TO description,
--   DROP COLUMN attempt,
--   DROP COLUMN climb_type;

-- CREATE TABLE if NOT EXISTS ticks (
--   tick_id       uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
--   route_id      uuid NOT NULL REFERENCES routes(route_id) ON DELETE CASCADE,
--   style         text NOT NULL,  -- TR | Lead | Boulder
--   attempt       text NOT NULL,  -- Onsight | Flash | Redpoint | Fall
--   created_at    TIMESTAMP NOT NULL DEFAULT now(),
--   updated_at    TIMESTAMP NOT NULL DEFAULT now()
-- );