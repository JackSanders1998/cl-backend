From Source:
- Run project locally in watch mode (exclude bindings changes): `cargo watch -qcx 'shuttle run' --ignore '**/cl-bindings/**'`
- Deploy project to shuttle: `cargo shuttle deploy`
- Prepare sqlx: `cargo sqlx prepare`
- Add sqlx migration: `sqlx migrate add <migration-name>`

From cl-bindings (`cl-bindings`)
- Generate openapi bindings: `npm run generate`
- Publish openapi bindings patch: `npm version patch && npm publish --access=public`

```
            WITH sesh_data AS (
                SELECT * FROM seshes WHERE seshes.sesh_id = $1 ORDER BY start DESC
            ), tick_and_location_data AS (
              SELECT
                sesh_data.*,

                locations.location_id,
                locations.author,
                locations.name,
                locations.environment,
                locations.description,
                locations.created_at AS location_created_at,
                locations.updated_at AS location_updated_at,

                ticks.tick_id,
                ticks.route_id,
                ticks.discipline,
                ticks.attempt,
                ticks.notes AS tick_notes,
                ticks.lap_group,
                ticks.created_at AS tick_created_at,
                ticks.updated_at AS tick_updated_at
            FROM sesh_data
              JOIN locations ON locations.location_id = sesh_data.location_id
              LEFT JOIN ticks ON ticks.sesh_id = sesh_data.sesh_id
            ), route_data AS (
              SELECT 
              tick_and_location_data.*,

              routes.route_id,
              routes.location_id,
              routes.grade,
              routes.scale,
              routes.disciplines,
              routes.author AS route_author,
              routes.description AS route_description,
              routes.created_at AS route_created_at,
              routes.updated_at AS route_updated_at
            FROM tick_and_location_data
                JOIN routes ON routes.route_id = tick_and_location_data.route_id
            )
            SELECT * FROM route_data
            ORDER BY route_data.created_at DESC;
```