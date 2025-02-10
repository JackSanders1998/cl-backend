pub use crate::models::{Attempt, ClimbType, Scale, Style};
use crate::models::{ClimbData, LocationData, Sesh, SqlxSeshWithLocationAndClimbs};
use crate::routes::seshes_repository::Id;
use crate::routes::{seshes_repository, AppState};
use std::collections::HashMap;
use std::io::ErrorKind;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub fn get_ids_from_struct(sesh_ids: Vec<Id>) -> Vec<Uuid> {
    sesh_ids.iter().map(|&i| i.sesh_id).collect()
}

pub fn map_db_rows_to_sesh_object(
    db_rows: Vec<SqlxSeshWithLocationAndClimbs>,
) -> Result<Vec<Sesh>, ErrorKind> {
    info!("map_db_rows_to_sesh_object called with: {:?}", db_rows);

    let mut mapped_seshes: Vec<Sesh> = Vec::new();

    let mut seshes: HashMap<Uuid, Vec<SqlxSeshWithLocationAndClimbs>> = HashMap::new();

    db_rows.into_iter().for_each(|row| {
        let sesh = seshes.entry(row.sesh_id).or_insert(vec![]);
        sesh.push(row);
    });

    for (_id, sesh) in seshes {
        match sesh.clone().into_iter().nth(0) {
            Some(first_sesh) => {
                let location = LocationData {
                    location_id: first_sesh.location_id,
                    name: first_sesh.name,
                    environment: first_sesh.environment,
                    created_at: first_sesh.location_created_at,
                    updated_at: first_sesh.location_updated_at,
                };

                let mut sesh_with_location_and_climbs: Sesh = Sesh {
                    sesh_id: first_sesh.sesh_id,
                    user_id: first_sesh.user_id.clone(),
                    notes: first_sesh.notes.clone(),
                    start: first_sesh.start,
                    end: first_sesh.end,
                    created_at: first_sesh.created_at,
                    updated_at: first_sesh.updated_at,
                    location,
                    climbs: Vec::new(),
                };

                for sesh_row in sesh {
                    if sesh_row.climb_type.is_some()
                        && sesh_row.climb_id.is_some()
                        && sesh_row.scale.is_some()
                        && sesh_row.grade.is_some()
                        && sesh_row.attempt.is_some()
                    {
                        let climb_row = ClimbData {
                            climb_id: sesh_row.climb_id.unwrap(),
                            climb_type: sesh_row.climb_type.unwrap(),
                            style: sesh_row.style,
                            scale: sesh_row.scale.unwrap(),
                            grade: sesh_row.grade.unwrap(),
                            attempt: sesh_row.attempt.unwrap(),
                            pointer: sesh_row.pointer,
                            notes: sesh_row.notes,
                            created_at: sesh_row.climb_created_at.unwrap(),
                            updated_at: sesh_row.climb_updated_at.unwrap(),
                        };
                        sesh_with_location_and_climbs.climbs.push(climb_row);
                    }
                }
                mapped_seshes.push(sesh_with_location_and_climbs);
            }
            _ => info!("No sesh found!"),
        }
    }

    mapped_seshes.sort_by(|a, b| b.start.cmp(&a.start));

    Ok(mapped_seshes)
}

pub async fn get_seshes_with_location_and_climbs(
    state: Arc<AppState>,
    sesh_ids: Vec<Uuid>,
) -> Result<Vec<Sesh>, ErrorKind> {
    info!(
        "get_seshes_with_location_and_climbs called with {:?}",
        sesh_ids
    );

    match seshes_repository::get_seshes_with_location_and_climbs(state, sesh_ids).await {
        Ok(db_rows) => map_db_rows_to_sesh_object(db_rows),
        Err(error) => {
            info!(
                "get_seshes_with_location_and_climbs failed with error: {:?}",
                error
            );
            Err(ErrorKind::NotFound)
        }
    }
}