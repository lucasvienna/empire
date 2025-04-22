use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum_extra::headers;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use empire::controllers::{
    NewUserPayload, RegisterPayload, UpdateUserPayload, UserBody, UserListBody,
};
use empire::db::players::PlayerRepository;
use empire::db::Repository;
use empire::domain::app_state::AppPool;
use empire::domain::auth::{encode_token, Claims};
use empire::domain::factions::FactionCode;
use empire::domain::player::buildings::PlayerBuilding;
use empire::domain::player::{NewPlayer, Player, PlayerKey, UserName};
use empire::schema::player::dsl::player;
use empire::schema::player_building;
use empire::services::auth_service::hash_password;
use http_body_util::BodyExt;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn get_all() {
    let server = common::init_server();
    let router = server.router;
    let pool = Arc::new(server.db_pool);
    let user = create_test_user(&pool, None);
    let bearer = get_bearer(user.id);

    let response = router
        .oneshot(
            Request::builder()
                .uri("/users")
                .header(header::AUTHORIZATION, format!("Bearer {}", bearer.token()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: UserListBody = serde_json::from_slice(&body).unwrap();
    assert!(!body.is_empty(), "No users returned");
    assert_eq!(
        body[0].username.as_str(),
        "test_user",
        "First user isn't test_user"
    )
}

#[tokio::test]
async fn create_and_get_by_id() {
    let app = common::spawn_app();
    let client = reqwest::Client::new();
    let pool = Arc::new(app.db_pool);
    let user = create_test_user(&pool, None);
    let bearer = get_bearer(user.id);

    let req = NewUserPayload {
        username: "test1".to_string(),
        password: "1234".to_string(),
        email: None,
        faction: FactionCode::Human,
    };
    let response = client
        .post(format!("{}/users", &app.address))
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .bearer_auth(bearer.token())
        .json(&req)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::CREATED);

    let new_user: UserBody = response.json().await.unwrap();
    assert_eq!(
        new_user.username.as_str(),
        req.username.as_str(),
        "New username isn't equal to request username"
    );

    let bearer = get_bearer(new_user.id);
    let response = client
        .get(format!("{}/users/{}", &app.address, new_user.id))
        .bearer_auth(bearer.token())
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    let user: UserBody = response.json().await.unwrap();
    assert_eq!(new_user.id, user.id);
    assert_eq!(new_user.username, user.username);
    assert_eq!(new_user.faction, user.faction);

    let del = delete_test_user(&pool, user.id);
    assert_eq!(del, 1, "Failed to delete user");
}

#[tokio::test]
async fn update() {
    let app = common::spawn_app();
    let client = reqwest::Client::new();
    let mut conn = app.db_pool.get().unwrap();

    let req = RegisterPayload {
        username: "testy".to_string(),
        password: "123".to_string(),
        email: None,
    };
    let response = client
        .post(format!("{}/register", &app.address))
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .json(&req)
        .send()
        .await
        .expect("Failed to create player.");
    assert_eq!(response.status(), StatusCode::CREATED);

    let user: Player = player.first(&mut conn).unwrap();
    assert_eq!(user.faction, FactionCode::Neutral, "Faction is not neutral");

    let player_blds: Vec<PlayerBuilding> = player_building::table
        .filter(player_building::player_id.eq(&user.id))
        .get_results(&mut conn)
        .expect("Failed to get player buildings");
    assert!(player_blds.is_empty(), "User has buildings");

    let bearer = get_bearer(user.id);
    let body = UpdateUserPayload {
        username: None,
        password: None,
        email: None,
        faction: Some(FactionCode::Human),
    };
    let response = client
        .put(format!("{}/users/{}", &app.address, user.id))
        .bearer_auth(bearer.token())
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .json(&body)
        .send()
        .await
        .expect("Failed to update player.");
    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let player_blds: Vec<PlayerBuilding> = player_building::table
        .filter(player_building::player_id.eq(&user.id))
        .get_results(&mut conn)
        .expect("Failed to get player buildings");
    assert!(!player_blds.is_empty(), "User has no buildings");
}

#[tokio::test]
async fn delete() {
    let app = common::spawn_app();
    let client = reqwest::Client::new();
    let pool = Arc::new(app.db_pool);
    let user = create_test_user(&pool, None);
    let bearer = get_bearer(user.id);

    let res = client
        .delete(format!("{}/users/{}", &app.address, user.id))
        .bearer_auth(bearer.token())
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let res = client
        .get(format!("{}/users/{}", &app.address, user.id))
        .bearer_auth(bearer.token())
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(
        res.status(),
        StatusCode::UNAUTHORIZED,
        "Shouldn't be able to authorize with deleted user"
    );

    let user2 = create_test_user(&pool, None);
    let bearer2 = get_bearer(user2.id); // TODO: add a test to cover the expired player trying to reuse the token
    let response = client
        .get(format!("{}/users/{}", &app.address, user.id))
        .bearer_auth(bearer2.token())
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

/// Create a player. Uses internal DB functions.
fn create_test_user(pool: &AppPool, faction: Option<FactionCode>) -> Player {
    let user_repo = PlayerRepository::new(pool);
    user_repo
        .create(NewPlayer {
            name: UserName::parse("test_user".to_string()).unwrap(),
            pwd_hash: hash_password(b"1234").unwrap(),
            email: None,
            faction: faction.unwrap_or(FactionCode::Human),
        })
        .expect("Failed to create player")
}

/// Delete a player. Uses internal DB functions.
fn delete_test_user(pool: &AppPool, player_id: PlayerKey) -> usize {
    let user_repo = PlayerRepository::new(pool);
    user_repo.delete(&player_id).unwrap()
}

fn get_bearer(player_id: PlayerKey) -> Authorization<Bearer> {
    let now = chrono::Utc::now();
    let token = encode_token(Claims {
        sub: player_id,
        iat: now.timestamp() as usize,
        exp: (now + chrono::Duration::minutes(1)).timestamp() as usize,
    })
    .unwrap();

    headers::Authorization::bearer(&token).unwrap()
}
