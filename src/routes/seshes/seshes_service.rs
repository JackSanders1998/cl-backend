use crate::models::{
    CreateSesh, Location, SeshFromRow, SeshWithLocation, SeshWithLocationAndTicks, TickQuery,
};
use crate::routes::locations_repository::get_location_by_location_id;
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
    db_rows: Vec<SeshFromRow>,
) -> Result<Vec<SeshWithLocationAndTicks>, ErrorKind> {
    info!("map_db_rows_to_sesh_object called with: {:?}", db_rows);

    let mut mapped_seshes: Vec<SeshWithLocationAndTicks> = Vec::new();

    let mut seshes: HashMap<Uuid, Vec<SeshFromRow>> = HashMap::new();

    db_rows.into_iter().for_each(|row| {
        let sesh = seshes.entry(row.sesh_id).or_insert(vec![]);
        sesh.push(row);
    });

    for (_id, sesh) in seshes {
        match sesh.clone().into_iter().nth(0) {
            Some(first_sesh) => {
                let location = Location {
                    location_id: first_sesh.location_id,
                    author: first_sesh.author,
                    name: first_sesh.name,
                    environment: first_sesh.environment,
                    description: first_sesh.description,
                    created_at: first_sesh.location_created_at,
                    updated_at: first_sesh.location_updated_at,
                };

                let mut hydrated_sesh: SeshWithLocationAndTicks = SeshWithLocationAndTicks {
                    sesh_id: first_sesh.sesh_id,
                    user_id: first_sesh.user_id.clone(),
                    notes: first_sesh.notes.clone(),
                    start: first_sesh.start,
                    end: first_sesh.end,
                    created_at: first_sesh.created_at,
                    updated_at: first_sesh.updated_at,
                    location,
                    ticks: Vec::new(),
                };

                for sesh_row in sesh {
                    let tick_row = TickQuery {
                        tick_id: sesh_row.tick_id,
                        route_id: sesh_row.route_id,
                        discipline: sesh_row.discipline,
                        attempt: sesh_row.attempt,
                        tick_notes: sesh_row.tick_notes,
                        lap_group: sesh_row.lap_group,
                        tick_created_at: sesh_row.tick_created_at,
                        tick_updated_at: sesh_row.tick_updated_at,
                    };
                    hydrated_sesh.ticks.push(tick_row);
                }
                mapped_seshes.push(hydrated_sesh);
            }
            _ => info!("No sesh found!"),
        }
    }

    mapped_seshes.sort_by(|a, b| b.start.cmp(&a.start));

    Ok(mapped_seshes)
}

pub async fn get_hydrated_seshes(
    state: Arc<AppState>,
    sesh_ids: Vec<Uuid>,
) -> Result<Vec<SeshWithLocationAndTicks>, ErrorKind> {
    info!("get_hydrated_seshes called with {:?}", sesh_ids);

    match seshes_repository::get_hydrated_seshes(state, sesh_ids).await {
        Ok(db_rows) => map_db_rows_to_sesh_object(db_rows),
        Err(error) => {
            info!("get_hydrated_seshes failed with error: {:?}", error);
            Err(ErrorKind::NotFound)
        }
    }
}

pub async fn create_sesh(
    state: Arc<AppState>,
    user_id: String,
    payload: CreateSesh,
) -> Result<SeshWithLocationAndTicks, ErrorKind> {
    let location = match get_location_by_location_id(state.clone(), payload.location_id).await {
        Ok(location) => location,
        Err(_) => return Err(ErrorKind::NotFound),
    };

    match seshes_repository::create_sesh(state.clone(), payload, user_id.clone()).await {
        Ok(sesh) => Ok({
            // reconcile active seshes
            let _ = seshes_repository::reconcile_active_seshes(state, user_id).await;

            // return SeshWithLocationAndTicks
            SeshWithLocationAndTicks {
                sesh_id: sesh.sesh_id,
                user_id: sesh.user_id,
                notes: sesh.notes,
                start: sesh.start,
                end: sesh.end,
                created_at: sesh.created_at,
                updated_at: sesh.updated_at,
                location: Location {
                    location_id: sesh.location_id,
                    author: location.author,
                    name: location.name,
                    environment: location.environment,
                    description: location.description,
                    created_at: location.created_at,
                    updated_at: location.updated_at,
                },
                ticks: Vec::new(),
            }
        }),
        Err(error) => {
            info!("create_sesh failed with error: {:?}", error);
            Err(ErrorKind::NotFound)
        }
    }
}

pub async fn search_seshes(
    state: Arc<AppState>,
    user_id: String,
) -> Result<Vec<SeshWithLocation>, ErrorKind> {
    match seshes_repository::search_seshes(state, user_id).await {
        Ok(seshes) => {
            let mut mapped_seshes: Vec<SeshWithLocation> = Vec::new();

            for sesh in seshes {
                mapped_seshes.push(SeshWithLocation {
                    sesh_id: sesh.sesh_id,
                    user_id: sesh.user_id,
                    notes: sesh.notes,
                    start: sesh.start,
                    end: sesh.end,
                    created_at: sesh.created_at,
                    updated_at: sesh.updated_at,
                    location: Location {
                        location_id: sesh.location_id,
                        author: sesh.author,
                        name: sesh.name,
                        environment: sesh.environment,
                        description: sesh.description,
                        created_at: sesh.location_created_at,
                        updated_at: sesh.location_updated_at,
                    },
                });
            }

            Ok(mapped_seshes)
        }
        Err(error) => {
            info!("search_seshes failed with error: {:?}", error);
            Err(ErrorKind::NotFound)
        }
    }
}
