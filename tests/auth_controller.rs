use std::sync::Arc;

use axum::body::Body;
use axum::http;
use axum::http::{Request, StatusCode};
use empire::auth::utils::hash_password;
use empire::controllers::{LoginPayload, RegisterPayload};
use empire::db::players::PlayerRepository;
use empire::db::Repository;
use empire::domain::app_state::AppPool;
use empire::domain::factions::FactionCode;
use empire::domain::player::{NewPlayer, Player, UserEmail, UserName};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn login_fails_without_body() {
    let router = common::init_server().router;
    let response = router
        .oneshot(
            Request::builder()
                .uri("/login")
                .method(http::Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn login_fails_without_credentials() {
    let router = common::init_server().router;
    let response = router
        .oneshot(
            Request::builder()
                .uri("/login")
                .method(http::Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    json!({ "username": "", "password": "" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(Vec::from(body)).unwrap();
    assert!(
        body.contains("Missing credentials"),
        "Error message doesn't contain 'Missing credentials': {}",
        body
    );
}

#[tokio::test]
async fn login_fails_with_wrong_credentials() {
    let server = common::init_server();
    let router = server.router;
    let pool = Arc::new(server.db_pool);

    let user = create_test_user(&pool);

    let response = router
        .oneshot(
            Request::builder()
                .uri("/login")
                .method(http::Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    json!({ "username": user.name, "password": "WRONG :)" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(Vec::from(body)).unwrap();
    assert!(
        body.contains("Wrong credentials"),
        "Error message doesn't contain 'Wrong credentials': {}",
        body
    );
}

#[tokio::test]
async fn login_succeeds_with_correct_credentials() {
    let server = common::init_server();
    let router = server.router;
    let pool = Arc::new(server.db_pool);

    let user = create_test_user(&pool);

    let response = router
        .oneshot(
            Request::builder()
                .uri("/login")
                .method(http::Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    json!({ "username": user.name, "password": "1234" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let cookie = response.headers().get(http::header::SET_COOKIE);
    assert!(cookie.is_some());
    let value = cookie.unwrap().to_str().unwrap();
    assert!(
        value.contains("rsession="),
        "'rsession=' not found in cookie: {}",
        value
    );
}

#[tokio::test]
async fn cannot_register_with_existing_username() {
    let server = common::init_server();
    let router = server.router;
    let pool = Arc::new(server.db_pool);

    let user = create_test_user(&pool);
    let register = RegisterPayload {
        username: user.name.clone(),
        password: "1234".to_string(),
        email: None,
    };

    let response = router
        .oneshot(
            Request::builder()
                .uri("/register")
                .method(http::Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&register).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(Vec::from(body)).unwrap();
    assert!(
        body.contains("Username already taken"),
        "Error message doesn't contain 'Username already taken': {}",
        body
    );
}

#[tokio::test]
async fn user_can_register_and_login() {
    let app = common::spawn_app();
    let pool = Arc::new(app.db_pool);
    let client = reqwest::Client::new();

    let req = RegisterPayload {
        username: "test1".to_string(),
        password: "1234".to_string(),
        email: None,
    };
    let response = client
        .post(format!("{}/register", &app.address))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .json(&req)
        .send()
        .await
        .expect("Failed to execute register request.");

    assert_eq!(response.status(), StatusCode::CREATED);

    let new_user = get_user_by_name(&pool, "test1").unwrap();
    assert_eq!(
        new_user.name.as_str(),
        req.username.as_str(),
        "New username isn't equal to request username"
    );

    let req = LoginPayload {
        username: req.username.clone(),
        password: req.password.clone(),
    };
    let response = client
        .post(format!("{}/login", &app.address))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .json(&req)
        .send()
        .await
        .expect("Failed to execute login request.");

    assert_eq!(response.status(), StatusCode::OK);
    let cookie = response.headers().get(http::header::SET_COOKIE);
    assert!(cookie.is_some());
}

#[tokio::test]
async fn logout_succeeds() {
    let app = common::spawn_app();
    let pool = Arc::new(app.db_pool);
    let client = reqwest::Client::new();

    let user = create_test_user(&pool);
    let req = LoginPayload {
        username: user.name,
        password: "1234".to_string(),
    };
    let response = client
        .post(format!("{}/login", &app.address))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .json(&req)
        .send()
        .await
        .expect("Failed to execute login request.");

    assert_eq!(response.status(), StatusCode::OK);
    let cookie = response.headers().get(http::header::SET_COOKIE);
    assert!(cookie.is_some());

    let response = client
        .get(format!("{}/logout", &app.address))
        .header(http::header::COOKIE, cookie.unwrap().to_str().unwrap())
        .send()
        .await
        .expect("Failed to execute logout request.");

    assert_eq!(response.status(), StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct Res {
        status: String,
    }
    let body: Res = response.json().await.unwrap();

    assert_eq!(body.status, "ok");
}

/// Create a player. Uses internal DB functions.
fn create_test_user(pool: &AppPool) -> Player {
    let user_repo = PlayerRepository::new(pool);
    user_repo
        .create(NewPlayer {
            name: UserName::parse("test_user".to_string()).unwrap(),
            pwd_hash: hash_password(b"1234").unwrap(),
            email: Some(UserEmail::parse("test@example.com".to_string()).unwrap()),
            faction: FactionCode::Human,
        })
        .expect("Failed to create user")
}

fn get_user_by_name(pool: &AppPool, name: &str) -> empire::Result<Player> {
    let repo = PlayerRepository::new(pool);
    repo.get_by_name(name)
}
