pub use crate::models::{Attempt, ClimbType, Scale, Style};
use crate::models::{
    ClimbData, CreateLocation, SeshWithLocationAndClimbs, SqlxSeshWithLocationAndClimbs,
};

pub fn map_db_rows_to_sesh_object(
    seshes: Vec<SqlxSeshWithLocationAndClimbs>,
) -> SeshWithLocationAndClimbs {
    let location = CreateLocation {
        name: seshes[0].name.clone(),
        environment: seshes[0].environment.clone(),
    };

    let mut sesh_with_location_and_climbs = SeshWithLocationAndClimbs {
        sesh_id: seshes[0].sesh_id,
        user_id: seshes[0].user_id.clone(),
        location_id: seshes[0].location_id,
        notes: seshes[0].notes.clone(),
        start: seshes[0].start,
        end: seshes[0].end,
        created_at: seshes[0].created_at,
        updated_at: seshes[0].updated_at,
        location,
        climbs: Vec::new(),
    };

    for row in seshes {
        let climb_row = ClimbData {
            climb_type: row.climb_type,
            style: row.style,
            scale: row.scale,
            grade: row.grade,
            attempt: row.attempt,
            pointer: row.pointer,
            notes: row.notes,
        };
        sesh_with_location_and_climbs.climbs.push(climb_row);
    }

    sesh_with_location_and_climbs
}
