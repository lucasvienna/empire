use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::{Authorization, HeaderMapExt};
use chrono::Utc;
use cookie::{time, Cookie, SameSite};
use derive_more::Deref;
use serde::Serialize;
use tracing::{debug, error, instrument, trace, warn};

use crate::auth::session_service::SessionService;
use crate::db::players::PlayerRepository;
use crate::db::Repository;
use crate::domain::app_state::AppPool;
use crate::domain::auth::decode_token;

pub const TOKEN_COOKIE_NAME: &str = "token";
pub const SESSION_COOKIE_NAME: &str = "rsession";

/// A session token.
#[derive(Debug, Clone, Serialize, Deref)]
pub struct SessionToken(String);

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

#[instrument(skip(pool, cookie_jar, req, next))]
pub async fn auth_middleware(
    State(pool): State<AppPool>,
    cookie_jar: CookieJar,
    mut req: Request,
    next: Next,
) -> crate::Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
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
        let srv = SessionService::new(&pool);
        let cookie = Cookie::build((SESSION_COOKIE_NAME, token.clone()))
            .path("/")
            .same_site(SameSite::Lax)
            .secure(true)
            .http_only(true);
        let session_token = SessionToken(token.clone());
        let (session, player) = srv.validate_session_token(token).map_err(|err| {
            error!("Invalid session token!");
            debug!("{:#?}", err);
            let json_error = ErrorResponse {
                status: "fail",
                message: "Invalid session token".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(json_error))
        })?;

        let duration = session.expires_at - Utc::now();
        let cookie = cookie
            .max_age(time::Duration::seconds(duration.num_seconds()))
            .build();

        jar = jar.add(cookie);
        req.extensions_mut().insert(player);
        req.extensions_mut().insert(session);
        req.extensions_mut().insert(session_token);
    } else if let Some(token) = jwt_token {
        // fallback to jwt token
        let claims = decode_token(token.as_str())
            .map_err(|_| {
                error!("Invalid token!");
                let json_error = ErrorResponse {
                    status: "fail",
                    message: "Invalid token".to_string(),
                };
                (StatusCode::UNAUTHORIZED, Json(json_error))
            })?
            .claims;
        let player_id = claims.sub;

        let player_repo = PlayerRepository::new(&pool);
        let user = player_repo.find_by_id(&player_id).map_err(|e| {
            error!("Error fetching player from database: {}", e);
            let json_error = ErrorResponse {
                status: "fail",
                message: format!("Error fetching player from database: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
        })?;

        let user = user.ok_or_else(|| {
            error!("User not found in database");
            let json_error = ErrorResponse {
                status: "fail",
                message: "The player belonging to this token no longer exists".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(json_error))
        })?;

        req.extensions_mut().insert(user);
    } else {
        // no auth provided
        warn!("No token found in request");
        let json_error = ErrorResponse {
            status: "fail",
            message: "You are not logged in, please provide token".to_string(),
        };
        return Err((StatusCode::UNAUTHORIZED, Json(json_error)));
    }

    let response = next.run(req).await;
    Ok((jar, response))
}
