/// This module contains end-to-end tests for the `locations` API.
///
/// # Tests
///
/// - `location_201`: Tests the creation of a location and verifies the response.
/// - `location_200`: Tests retrieving a location by ID and verifies the response.
/// - `locations_200`: Tests retrieving multiple locations by name and verifies the response.
/// - `location_404`: Tests retrieving a non-existent location and expects a 404 status code.
///
/// # Helper Functions
///
/// - `post_location`: Creates a new location using the API and returns the created `Location`.
/// - `delete_location`: Deletes a location by ID using the API.
#[cfg(test)]
mod test_locations {
    use crate::models::{CreateLocation, Environment, Location};
    use uuid::Uuid;

    #[tokio::test]
    async fn location_201() {
        let location = post_location(None).await;
        delete_location(location.location_id).await;

        assert_eq!(
            location,
            Location {
                location_id: location.location_id,
                author: "e2e location author".to_string(),
                name: "e2e location name".to_string(),
                environment: Environment::Gym,
                description: Some("test location description".to_string()),
                created_at: location.created_at,
                updated_at: location.updated_at
            }
        );
    }

    #[tokio::test]
    async fn location_200() {
        let bearer_auth = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
        let client = reqwest::Client::new();
        let post_location = post_location(None).await;

        let get_response = client
            .get(&format!(
                "http://127.0.0.1:8000/locations/{}",
                post_location.location_id
            ))
            .bearer_auth(bearer_auth.clone())
            .send()
            .await
            .expect("GET should succeed with a 204 status code");

        assert_eq!(get_response.status(), 200);

        let get_location: Location = get_response
            .json()
            .await
            .expect("Response should be {status: 'healthy'}");

        delete_location(get_location.location_id).await;

        assert_eq!(
            get_location,
            Location {
                location_id: get_location.location_id,
                author: "e2e location author".to_string(),
                name: "e2e location name".to_string(),
                environment: Environment::Gym,
                description: Some("test location description".to_string()),
                created_at: get_location.created_at,
                updated_at: get_location.updated_at
            }
        );
    }

    #[tokio::test]
    async fn locations_200() {
        let bearer_auth = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
        let client = reqwest::Client::new();
        let location_a = post_location(Some("locations_200 e2e test".to_string())).await;
        let location_b = post_location(Some("locations_200 e2e test".to_string())).await;

        let get_response = client
            .get("http://127.0.0.1:8000/locations?name=locations_200 e2e test")
            .bearer_auth(bearer_auth.clone())
            .send()
            .await
            .expect("GET should succeed with a 204 status code");

        assert_eq!(get_response.status(), 200);

        let get_location: Vec<Location> = get_response
            .json()
            .await
            .expect("Response should be Vec<location>");

        delete_location(location_a.location_id).await;
        delete_location(location_b.location_id).await;

        println!("{:?}", get_location);

        // There should only be 2 e2e locations in the db
        assert_eq!(get_location.len(), 2);
    }

    #[tokio::test]
    async fn location_404() {
        let bearer_auth = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
        let client = reqwest::Client::new();

        let response = client
            .get("http://127.0.0.1:8000/locations/00000000-0000-0000-0000-000000000000")
            .bearer_auth(bearer_auth)
            .send()
            .await
            .expect("Location should fail with a 404 status code");

        assert_eq!(response.status(), 404);
    }

    // >>>>>> Helper functions <<<<<<<

    async fn post_location(name: Option<String>) -> Location {
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

    async fn delete_location(location_id: Uuid) {
        let bearer_auth: String = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
        let client = reqwest::Client::new();

        let response = client
            .delete(&format!("http://127.0.0.1:8000/locations/{}", location_id))
            .bearer_auth(bearer_auth)
            .send()
            .await
            .expect("Location should succeed with a 204 status code");

        assert_eq!(response.status(), 204);
    }
}
