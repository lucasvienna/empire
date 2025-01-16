use axum::body::Body;
use axum::http;
use axum::http::{Request, StatusCode};
use empire::controllers::user_controller::{CreateUserRequest, UserResponse};
use empire::db::conn::get_test_pool;
use empire::db::users::UserRepository;
use empire::db::Repository;
use empire::models::user::{NewUser, User};
use http_body_util::BodyExt;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn get_all_works() {
    let router = common::get_app().expect("Failed to spawn our app.");
    let response = router
        .oneshot(
            Request::builder()
                .uri("/users")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"[]");
}
#[tokio::test]
async fn create_and_get_by_id_works() {
    let address = common::spawn_app();
    let client = reqwest::Client::new();

    let req = CreateUserRequest {
        username: "test1".to_string(),
        faction: 2,
    };

    let response = client
        .post(format!("{}/users", &address))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .json(&req)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::CREATED);
    let new_user: UserResponse = response.json().await.unwrap();
    assert_eq!(
        new_user.username.as_str(),
        req.username.as_str(),
        "New username isn't equal to request username"
    );

    let response = client
        .get(format!("{}/users/{}", &address, new_user.id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    let user: UserResponse = response.json().await.unwrap();
    assert_eq!(new_user.id, user.id);
    assert_eq!(new_user.username, user.username);
    assert_eq!(new_user.faction, user.faction);
}

#[tokio::test]
async fn delete_works() {
    let address = common::spawn_app();
    let client = reqwest::Client::new();
    let user = create_test_user();

    let response = client
        .delete(format!("{}/users/{}", &address, user.id))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client
        .get(format!("{}/users/{}", &address, user.id))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

fn create_test_user() -> User {
    let mut conn = get_test_pool().get().unwrap();
    let user_repo = UserRepository {};
    user_repo
        .create(
            &mut conn,
            &NewUser {
                name: "test1",
                faction: 2,
                data: None,
            },
        )
        .unwrap()
}
