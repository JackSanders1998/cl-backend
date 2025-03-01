use uuid::Uuid;

use crate::models::{
    Attempt, CreateLocation, CreateRoute, CreateSesh, CreateTick, Discipline, Environment,
    Location, Route, Scale, Tick,
};

// >>>>>>> Location Helpers <<<<<<<
pub async fn post_location(name: Option<String>) -> Location {
    let bearer_auth: String = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
    let client = reqwest::Client::new();

    let response = client
        .post("http://127.0.0.1:8000/locations")
        .bearer_auth(bearer_auth)
        .json(&CreateLocation {
            author: "e2e location author".to_string(),
            name: name.unwrap_or_else(|| "e2e location name".to_string()),
            environment: Environment::Gym,
            description: Some("test location description".to_string()),
        })
        .send()
        .await
        .expect("POST should succeed with a 201 status code");

    assert_eq!(response.status(), 201);

    response
        .json()
        .await
        .expect("Response should be a Location")
}

pub async fn delete_location(location_id: Uuid) {
    let bearer_auth: String = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
    let client = reqwest::Client::new();

    let response = client
        .delete(&format!("http://127.0.0.1:8000/locations/{}", location_id))
        .bearer_auth(bearer_auth)
        .send()
        .await
        .expect("DELETE should succeed with a 204 status code");

    assert_eq!(response.status(), 204);
}

// >>>>>>> Route Helpers <<<<<<<
pub async fn post_route(location_id: Uuid) -> Route {
    let bearer_auth: String = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
    let client = reqwest::Client::new();

    let response = client
        .post("http://127.0.0.1:8000/routes")
        .bearer_auth(bearer_auth)
        .json(&CreateRoute {
            author: "e2e route author".to_string(),
            description: Some("test route description".to_string()),
            location_id,
            grade: "5.10a".to_string(),
            scale: Scale::Yosemite,
            disciplines: vec![Discipline::Sport, Discipline::TopRope],
        })
        .send()
        .await
        .expect("POST should succeed with a 201 status code");

    assert_eq!(response.status(), 201);

    response.json().await.expect("Response should be a Route")
}

pub async fn delete_route(route_id: Uuid) {
    let bearer_auth: String = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
    let client = reqwest::Client::new();

    let response = client
        .delete(&format!("http://127.0.0.1:8000/routes/{}", route_id))
        .bearer_auth(bearer_auth)
        .send()
        .await
        .expect("DELETE should succeed with a 204 status code");

    assert_eq!(response.status(), 204);
}

// >>>>>>> Tick Helpers <<<<<<<
pub async fn post_tick(sesh_id: Uuid, route_id: Uuid) -> Tick {
    let bearer_auth: String = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
    let client = reqwest::Client::new();

    let response = client
        .post("http://127.0.0.1:8000/ticks")
        .bearer_auth(bearer_auth)
        .json(&CreateTick {
            route_id,
            sesh_id,
            discipline: Discipline::Sport,
            attempt: Attempt::Flash,
            notes: Some("test tick notes".to_string()),
            lap_group: None,
        })
        .send()
        .await
        .expect("POST should succeed with a 201 status code");

    assert_eq!(response.status(), 201);

    response.json().await.expect("Response should be a Tick")
}

pub async fn delete_tick(tick_id: Uuid) {
    let bearer_auth: String = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
    let client = reqwest::Client::new();

    let response = client
        .delete(&format!("http://127.0.0.1:8000/ticks/{}", tick_id))
        .bearer_auth(bearer_auth)
        .send()
        .await
        .expect("DELETE should succeed with a 204 status code");

    assert_eq!(response.status(), 204);
}

// >>>>>>> Sesh Helpers <<<<<<<
pub async fn post_sesh(location_id: Uuid) -> Tick {
    let bearer_auth: String = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
    let client = reqwest::Client::new();

    let response = client
        .post("http://127.0.0.1:8000/seshes")
        .bearer_auth(bearer_auth)
        .json(&CreateSesh {
            location_id,
            notes: None,
        })
        .send()
        .await
        .expect("POST should succeed with a 201 status code");

    assert_eq!(response.status(), 201);

    response.json().await.expect("Response should be a Tick")
}

pub async fn delete_sesh(sesh_id: Uuid) {
    let bearer_auth: String = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
    let client = reqwest::Client::new();

    let response = client
        .delete(&format!("http://127.0.0.1:8000/seshes/{}", sesh_id))
        .bearer_auth(bearer_auth)
        .send()
        .await
        .expect("DELETE should succeed with a 204 status code");

    assert_eq!(response.status(), 204);
}
