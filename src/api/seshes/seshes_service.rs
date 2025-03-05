use crate::api::locations_repository::get_location_by_location_id;
use crate::api::{seshes_repository, ticks_service, AppState};
use crate::models::{CreateSesh, Location, SeshWithLocation, SeshWithLocationAndTicks};
use std::io::ErrorKind;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub async fn create_sesh(
    state: Arc<AppState>,
    user_id: String,
    payload: CreateSesh,
) -> Result<SeshWithLocation, ErrorKind> {
    let location = match get_location_by_location_id(state.clone(), payload.location_id).await {
        Ok(location) => location,
        Err(_) => return Err(ErrorKind::NotFound),
    };

    match seshes_repository::create_sesh(state.clone(), payload, user_id.clone()).await {
        Ok(sesh) => Ok({
            // reconcile active seshes
            let _ = seshes_repository::reconcile_active_seshes(state, user_id).await;

            // return SeshWithLocationAndTicks
            SeshWithLocation {
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
            }
        }),
        Err(error) => {
            info!("create_sesh failed with error: {:?}", error);
            Err(ErrorKind::NotFound)
        }
    }
}

pub async fn get_sesh_by_sesh_id(
    state: Arc<AppState>,
    sesh_id: Uuid,
) -> Result<SeshWithLocationAndTicks, ErrorKind> {
    let sesh =
        match seshes_repository::get_sesh_and_location_by_sesh_id(state.clone(), sesh_id).await {
            Ok(sesh) => sesh,
            Err(_) => return Err(ErrorKind::NotFound),
        };
    let ticks = match ticks_service::get_ticks_by_sesh_id(state.clone(), sesh_id).await {
        Ok(ticks) => ticks,
        Err(_) => return Err(ErrorKind::NotFound),
    };

    Ok(SeshWithLocationAndTicks {
        sesh_id: sesh.sesh_id,
        user_id: sesh.user_id,
        notes: sesh.notes,
        start: sesh.start,
        end: sesh.end,
        ticks,
        location: Location {
            location_id: sesh.location_id,
            author: sesh.author,
            name: sesh.name,
            environment: sesh.environment,
            description: sesh.description,
            created_at: sesh.location_created_at,
            updated_at: sesh.location_updated_at,
        },
        created_at: sesh.created_at,
        updated_at: sesh.updated_at,
    })
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
