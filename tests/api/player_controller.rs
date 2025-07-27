use axum::http::{header, StatusCode};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use empire::domain::player::buildings::PlayerBuilding;
use empire::schema::player_building;
use serde_json::json;

use crate::common::TestApp;

#[tokio::test]
async fn join_faction_success() {
    let server = TestApp::new();
    let client = reqwest::Client::new();
    let user = server.create_test_user(None); // Start with neutral faction
    let bearer = server.create_bearer_token(user.id);

    let response = client
        .post(format!("{}/player/faction", &server.address))
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .bearer_auth(bearer.token())
        .json(&json!({"faction": "human"}))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::ACCEPTED);

    // Verify the user's faction was updated, and buildings were created
    let mut conn = server.db_pool.get().unwrap();
    let player_blds: Vec<PlayerBuilding> = player_building::table
        .filter(player_building::player_id.eq(&user.id))
        .get_results(&mut conn)
        .expect("Failed to get player buildings");
    assert!(
        !player_blds.is_empty(),
        "User should have buildings after joining faction"
    );
}
