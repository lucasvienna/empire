use axum::http::StatusCode;
use empire::domain::factions::FactionCode;

use crate::common::TestApp;

#[tokio::test]
async fn get_factions_returns_200() {
	let app = TestApp::new();
	let client = reqwest::Client::new();
	let user = app.create_test_user(Some(FactionCode::Human));
	let token = app.create_bearer_token(&user.id);

	let response = client
		.get(format!("{}/game/factions", &app.address))
		.bearer_auth(token.token())
		.send()
		.await
		.expect("Failed to execute request.");

	assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn get_faction_details_returns_200() {
	let app = TestApp::new();
	let client = reqwest::Client::new();
	let user = app.create_test_user(Some(FactionCode::Human));
	let token = app.create_bearer_token(&user.id);

	let response = client
		.get(format!("{}/game/factions/human", &app.address))
		.bearer_auth(token.token())
		.send()
		.await
		.expect("Failed to execute request.");

	assert_eq!(response.status(), StatusCode::OK);
}
