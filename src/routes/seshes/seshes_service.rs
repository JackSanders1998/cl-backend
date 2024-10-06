pub use crate::models::{Attempt, ClimbType, Scale, Style};
use crate::models::{
    ClimbData, CreateLocation, SeshWithLocationAndClimbs, SqlxSeshWithLocationAndClimbs,
};
use std::io::ErrorKind;

pub fn map_db_rows_to_sesh_object(
    seshes: Vec<SqlxSeshWithLocationAndClimbs>,
) -> Result<SeshWithLocationAndClimbs, ErrorKind> {
    let first_sesh = seshes.clone().into_iter().nth(0);

    match first_sesh {
        Some(sesh) => {
            let location = CreateLocation {
                name: sesh.name.clone(),
                environment: sesh.environment.clone(),
            };

            let mut sesh_with_location_and_climbs: SeshWithLocationAndClimbs =
                SeshWithLocationAndClimbs {
                    sesh_id: sesh.sesh_id,
                    user_id: sesh.user_id.clone(),
                    location_id: sesh.location_id,
                    notes: sesh.notes.clone(),
                    start: sesh.start,
                    end: sesh.end,
                    created_at: sesh.created_at,
                    updated_at: sesh.updated_at,
                    location,
                    climbs: Vec::new(),
                };

            for row in seshes {
                if row.climb_type.is_some()
                    && row.scale.is_some()
                    && row.grade.is_some()
                    && row.attempt.is_some()
                {
                    let climb_row = ClimbData {
                        climb_type: row.climb_type.unwrap(),
                        style: row.style,
                        scale: row.scale.unwrap(),
                        grade: row.grade.unwrap(),
                        attempt: row.attempt.unwrap(),
                        pointer: row.pointer,
                        notes: row.notes,
                    };
                    sesh_with_location_and_climbs.climbs.push(climb_row);
                }
            }

            Ok(sesh_with_location_and_climbs)
        }
        _ => Err(ErrorKind::NotFound),
    }
}
