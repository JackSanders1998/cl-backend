use crate::api::{routes_service, ticks_repository, AppState};
use crate::models::{CreateTick, TickWithRoute};
use std::io::ErrorKind;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub async fn create_tick(
    state: Arc<AppState>,
    payload: CreateTick,
) -> Result<TickWithRoute, ErrorKind> {
    let route = match routes_service::get_route_by_route_id(state.clone(), payload.route_id).await {
        Ok(route) => route,
        Err(_) => return Err(ErrorKind::NotFound),
    };

    match ticks_repository::create_tick(state, payload).await {
        Ok(tick) => Ok({
            // return SeshWithLocationAndTicks
            TickWithRoute {
                tick_id: tick.tick_id,
                sesh_id: tick.sesh_id,
                route_id: tick.route_id,
                discipline: tick.discipline,
                attempt: tick.attempt,
                notes: tick.notes,
                lap_group: tick.lap_group,
                created_at: tick.created_at,
                updated_at: tick.updated_at,
                route,
            }
        }),
        Err(error) => {
            info!("create_sesh failed with error: {:?}", error);
            Err(ErrorKind::NotFound)
        }
    }
}

pub async fn get_ticks_by_sesh_id(
    state: Arc<AppState>,
    sesh_id: Uuid,
) -> Result<Vec<TickWithRoute>, ErrorKind> {
    let ticks = match ticks_repository::search_ticks(state.clone(), sesh_id).await {
        Ok(ticks) => ticks,
        Err(_) => return Err(ErrorKind::NotFound),
    };

    let mut ticks_with_routes = Vec::new();
    for tick in ticks {
        let route = match routes_service::get_route_by_route_id(state.clone(), tick.route_id).await
        {
            Ok(route) => route,
            Err(_) => return Err(ErrorKind::NotFound),
        };

        ticks_with_routes.push(TickWithRoute {
            tick_id: tick.tick_id,
            sesh_id: tick.sesh_id,
            route_id: tick.route_id,
            discipline: tick.discipline,
            attempt: tick.attempt,
            notes: tick.notes,
            lap_group: tick.lap_group,
            created_at: tick.created_at,
            updated_at: tick.updated_at,
            route,
        });
    }

    Ok(ticks_with_routes)
}
