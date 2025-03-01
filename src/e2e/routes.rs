#[cfg(test)]
mod test_routes {
    use crate::{
        e2e::{delete_location, delete_route, post_location, post_route},
        models::{CreateLocation, Discipline, Environment, Location, Route, Scale},
    };
    use uuid::Uuid;

    #[tokio::test]
    async fn route_201() {
        let location = post_location(None).await;
        let route = post_route(location.location_id).await;
        delete_location(location.location_id).await;
        delete_route(route.route_id).await;

        assert_eq!(
            route,
            Route {
                route_id: route.route_id,
                location_id: location.location_id,
                grade: "5.10a".to_string(),
                scale: Scale::Yosemite,
                disciplines: vec![Discipline::Sport, Discipline::TopRope],
                author: "e2e route author".to_string(),
                description: Some("test route description".to_string()),
                created_at: route.created_at,
                updated_at: route.updated_at
            }
        );
    }

    #[tokio::test]
    async fn route_200() {
        let bearer_auth = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
        let client = reqwest::Client::new();
        let location = post_location(None).await;
        let route = post_route(location.location_id).await;

        let get_response = client
            .get(&format!("http://127.0.0.1:8000/routes/{}", route.route_id))
            .bearer_auth(bearer_auth.clone())
            .send()
            .await
            .expect("GET should succeed with a 204 status code");

        assert_eq!(get_response.status(), 200);

        let get_route: Route = get_response
            .json()
            .await
            .expect("Response should be a Route");

        delete_route(route.route_id).await;
        delete_location(location.location_id).await;

        assert_eq!(
            get_route,
            Route {
                location_id: location.location_id,
                route_id: route.route_id,
                grade: "5.10a".to_string(),
                scale: Scale::Yosemite,
                disciplines: vec![Discipline::Sport, Discipline::TopRope],
                author: "e2e route author".to_string(),
                description: Some("test route description".to_string()),
                created_at: route.created_at,
                updated_at: route.updated_at
            }
        );
    }

    #[tokio::test]
    async fn routes_200() {
        let bearer_auth = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");
        let client = reqwest::Client::new();
        let location = post_location(None).await;
        let route_a = post_route(location.location_id).await;
        let route_b = post_route(location.location_id).await;

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

        delete_location(route_a.location_id).await;
        delete_location(route_b.location_id).await;

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
}
