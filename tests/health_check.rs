use axum::body::Body;
use axum::http::{Request, StatusCode};
use empire::controllers::HealthCheckResponse;
use http_body_util::BodyExt;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn health_check_works() {
    let router = common::init_server().router;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: HealthCheckResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(body.status, "OK");
}

#[tokio::test]
async fn health_check_with_server() {
    let app = common::spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let body: HealthCheckResponse = response.json().await.unwrap();
    assert_eq!(body.status, "OK");
}
