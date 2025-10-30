use std::convert::Infallible;

use axum::Json;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::{Authorization, HeaderMapExt};
use chrono::Utc;
use cookie::{Cookie, SameSite, time};
use derive_more::Deref;
use serde::Serialize;
use tracing::{debug, error, instrument, trace, warn};

use crate::auth::session_operations;
use crate::db::extractor::DatabaseConnection;
use crate::db::players;
use crate::domain::auth::{AuthenticatedUser, Claims, decode_token};

pub const TOKEN_COOKIE_NAME: &str = "rstoken";
pub const SESSION_COOKIE_NAME: &str = "rsession";

/// A session token.
#[derive(Debug, Clone, Serialize, Deref)]
pub struct SessionToken(String);

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
	pub status: &'static str,
	pub message: String,
}

#[instrument(skip_all)]
pub async fn auth_middleware(
	DatabaseConnection(mut conn): DatabaseConnection,
	cookie_jar: CookieJar,
	mut req: Request,
	next: Next,
) -> crate::Result<impl IntoResponse, Infallible> {
	let mut jar = cookie_jar.clone();

	// user auth can be provided as a jwt token, session token, or Bearer token
	let session_token = cookie_jar.get(SESSION_COOKIE_NAME).map(|cookie| {
		trace!("Found session token in cookie");
		cookie.value().to_string()
	});
	let jwt_token = cookie_jar
		.get(TOKEN_COOKIE_NAME)
		.map(|cookie| {
			trace!("Found JWT token in cookie");
			cookie.value().to_string()
		})
		.or_else(|| {
			let bearer = req.headers().typed_get::<Authorization<Bearer>>();
			bearer.map(|bearer| {
				trace!("Found JWT token in Bearer header");
				bearer.token().to_string()
			})
		});

	// try auth with session token
	if let Some(token) = session_token {
		let session_token = SessionToken(token.clone());

		match session_operations::validate_token(&mut conn, token.clone()) {
			Ok((session, player)) => {
				let duration = session.expires_at - Utc::now();
				let cookie = Cookie::build((SESSION_COOKIE_NAME, token.clone()))
					.path("/")
					.same_site(SameSite::Lax)
					.secure(true)
					.http_only(true)
					.max_age(time::Duration::seconds(duration.num_seconds()))
					.build();

				jar = jar.add(cookie);
				req.extensions_mut().insert(AuthenticatedUser(player));
				req.extensions_mut().insert(session);
				req.extensions_mut().insert(session_token);
			}
			Err(e) => {
				error!("Invalid session token!");
				debug!("{:#?}", e);
				jar = jar.remove(SESSION_COOKIE_NAME);
				let json_error = ErrorResponse {
					status: "fail",
					message: "Invalid session token".to_string(),
				};
				return Ok(unauthorized!(json_error, jar));
			}
		}
	} else if let Some(token) = jwt_token {
		// fallback to jwt token
		match decode_token(token.as_str()) {
			Ok(token_data) => {
				let claims: Claims = token_data.claims;
				if claims.exp <= Utc::now().timestamp() as usize {
					jar = jar.remove(Cookie::new(TOKEN_COOKIE_NAME, ""));
					error!("Token has expired!");
					let json_error = ErrorResponse {
						status: "fail",
						message: "Token has expired".to_string(),
					};
					return Ok(unauthorized!(json_error, jar));
				}

				let player_id = claims.sub;
				match players::find_by_id(&mut conn, &player_id) {
					Ok(Some(player)) => {
						req.extensions_mut().insert(AuthenticatedUser(player));
					}
					Ok(None) => {
						error!("User not found in database");
						jar = jar.remove(Cookie::new(TOKEN_COOKIE_NAME, ""));
						let json_error = ErrorResponse {
							status: "fail",
							message: "The player belonging to this token no longer exists"
								.to_string(),
						};
						return Ok(unauthorized!(json_error, jar));
					}
					Err(e) => {
						error!("Error fetching player from database: {}", e);
						jar = jar.remove(Cookie::new(TOKEN_COOKIE_NAME, ""));
						let json_error = ErrorResponse {
							status: "fail",
							message: format!("Error fetching player from database: {e}"),
						};
						return Ok(unauthorized!(json_error, jar));
					}
				}
			}
			Err(_) => {
				error!("Invalid token!");
				jar = jar.remove(Cookie::new(TOKEN_COOKIE_NAME, ""));
				let json_error = ErrorResponse {
					status: "fail",
					message: "Invalid token".to_string(),
				};
				return Ok(unauthorized!(json_error, jar));
			}
		}
	} else {
		// no auth provided
		warn!("No token found in request");
		let json_error = ErrorResponse {
			status: "fail",
			message: "You are not logged in, please authenticate".to_string(),
		};
		return Ok(unauthorized!(json_error, jar));
	}

	Ok((jar, next.run(req).await))
}
