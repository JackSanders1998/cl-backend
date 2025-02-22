#[derive(serde::Deserialize, Debug, PartialEq)]
struct HealthCheck {
    status: String,
}

#[tokio::test]
async fn healthcheck_200() {
    let bearer_auth = std::env::var("BEARER_AUTH").expect("BEARER_AUTH must be set");

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/healthcheck")
        .bearer_auth(bearer_auth)
        .send()
        .await
        .expect("Heathcheck should succeed with a 200 status code");

    println!("{:?}", response);
    assert_eq!(response.status(), 200);

    let body: HealthCheck = response
        .json()
        .await
        .expect("Response should be {status: 'healthy'}");
    assert_eq!(
        body,
        HealthCheck {
            status: "healthy".to_string()
        }
    );
}
