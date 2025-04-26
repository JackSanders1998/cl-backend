-- Add migration script here
CREATE TABLE if NOT EXISTS workouts (
  workout_id   uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  user_id       text,   -- clerk
  log           json,   -- log of the workout
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);