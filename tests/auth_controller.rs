use axum::body::Body;
use axum::http;
use axum::http::{Request, StatusCode};
use empire::controllers::{LoginPayload, RegisterPayload};
use empire::db::users::UserRepository;
use empire::db::{DbConn, Repository};
use empire::domain::auth::AuthBody;
use empire::domain::faction::FactionCode;
use empire::domain::user::{NewUser, User, UserEmail, UserName};
use empire::services::auth_service::hash_password;
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
    let pool = server.db_pool;

    let user = create_test_user(pool.get().unwrap());

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
    let pool = server.db_pool;

    let user = create_test_user(pool.get().unwrap());

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

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(Vec::from(body)).unwrap();
    assert!(
        body.contains("access_token"),
        "Error message doesn't contain 'access_token': {}",
        body
    );
}

#[tokio::test]
async fn cannot_register_with_existing_username() {
    let server = common::init_server();
    let router = server.router;
    let pool = server.db_pool;

    let user = create_test_user(pool.get().unwrap());
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

    let new_user = get_user_by_name(app.db_pool.get().unwrap(), "test1").unwrap();
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
    let body: AuthBody = response.json().await.unwrap();
    assert!(!body.access_token.is_empty());
}

#[tokio::test]
async fn logout_succeeds() {
    let app = common::spawn_app();
    let client = reqwest::Client::new();

    let user = create_test_user(app.db_pool.get().unwrap());
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
    let body: AuthBody = response.json().await.unwrap();
    assert!(!body.access_token.is_empty());

    let response = client
        .get(format!("{}/logout", &app.address))
        .header(
            http::header::AUTHORIZATION,
            format!("{} {}", body.token_type, body.access_token),
        )
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

/// Create a user. Uses internal DB functions.
fn create_test_user(mut conn: DbConn) -> User {
    let user_repo = UserRepository {};
    user_repo
        .create(
            &mut conn,
            &NewUser {
                name: UserName::parse("test_user".to_string()).unwrap(),
                pwd_hash: hash_password(b"1234").unwrap(),
                email: Some(UserEmail::parse("test@example.com".to_string()).unwrap()),
                faction: FactionCode::Human,
            },
        )
        .unwrap()
}

fn get_user_by_name(mut conn: DbConn, name: &str) -> empire::Result<User> {
    let repo = UserRepository {};
    repo.get_by_name(&mut conn, name)
}
