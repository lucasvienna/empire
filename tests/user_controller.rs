use axum::body::Body;
use axum::http;
use axum::http::{Request, StatusCode};
use empire::controllers::{CreateUserRequest, UserResponse};
use empire::db::users::UserRepository;
use empire::db::{DbConn, Repository};
use empire::domain::user;
use empire::domain::user::{NewUser, User};
use http_body_util::BodyExt;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn get_all_works() {
    let router = common::init_server().router;
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
    let app = common::spawn_app();
    let client = reqwest::Client::new();

    let req = CreateUserRequest {
        username: "test1".to_string(),
        faction: 2,
    };
    let response = client
        .post(format!("{}/users", &app.address))
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
        .get(format!("{}/users/{}", &app.address, new_user.id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    let user: UserResponse = response.json().await.unwrap();
    assert_eq!(new_user.id, user.id);
    assert_eq!(new_user.username, user.username);
    assert_eq!(new_user.faction, user.faction);

    let del = delete_test_user(user.id, app.db_pool.get().unwrap());
    assert_eq!(del, 1, "Failed to delete user");
}

#[tokio::test]
async fn delete_works() {
    let app = common::spawn_app();
    let client = reqwest::Client::new();
    let user = create_test_user(app.db_pool.get().unwrap());

    let del_res = client
        .delete(format!("{}/users/{}", &app.address, user.id))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(del_res.status(), StatusCode::NO_CONTENT);

    let response = client
        .get(format!("{}/users/{}", &app.address, user.id))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

/// Create a user. Uses internal DB functions.
fn create_test_user(mut conn: DbConn) -> User {
    let user_repo = UserRepository {};
    user_repo
        .create(
            &mut conn,
            &NewUser {
                name: "test_user".to_string(),
                faction: 2,
                data: None,
            },
        )
        .unwrap()
}

/// Delete a user. Uses internal DB functions.
fn delete_test_user(user_id: user::PK, mut conn: DbConn) -> usize {
    let user_repo = UserRepository {};
    user_repo.delete(&mut conn, &user_id).unwrap()
}
