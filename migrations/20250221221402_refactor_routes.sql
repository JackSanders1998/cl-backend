-- Add migration script here
ALTER TABLE routes RENAME COLUMN climb_id TO route_id;
ALTER TABLE routes RENAME COLUMN notes TO description;
ALTER TABLE routes DROP COLUMN attempt;
ALTER TABLE routes DROP COLUMN climb_type;

CREATE TABLE if NOT EXISTS ticks (
  tick_id       uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  route_id      uuid NOT NULL REFERENCES routes(route_id) ON DELETE CASCADE,
  style         text NOT NULL,  -- TR | Lead | Boulder
  attempt       text NOT NULL,  -- Onsight | Flash | Redpoint | Fall
  created_at    TIMESTAMP NOT NULL DEFAULT now(),
  updated_at    TIMESTAMP NOT NULL DEFAULT now()
);