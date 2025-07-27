use axum::body::Body;
use axum::http::{Request, StatusCode};
use empire::controllers::health::{HealthCheckBody, LivenessCheckBody, ReadyCheckBody};
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::common::{TestApp, TestHarness};

#[tokio::test]
async fn health_check_works() {
    let router = TestHarness::new().router;

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
    let body: HealthCheckBody = serde_json::from_slice(&body).unwrap();
    assert_eq!(body.status, "OK");
}

#[tokio::test]
async fn readiness_check_works() {
    let router = TestHarness::new().router;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/health/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: ReadyCheckBody = serde_json::from_slice(&body).unwrap();
    assert!(body.ready);
    assert!(body.services.database);
    assert!(body.services.queue);
}

#[tokio::test]
async fn liveness_check_works() {
    let router = TestHarness::new().router;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/health/live")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: LivenessCheckBody = serde_json::from_slice(&body).unwrap();
    assert!(body.alive);
    assert!(!body.uptime.is_empty());
}

#[tokio::test]
async fn health_check_with_server() {
    let server = TestApp::new();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health", &server.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let body: HealthCheckBody = response.json().await.unwrap();
    assert_eq!(body.status, "OK");
}
