use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::{Authorization, HeaderMapExt};
use serde::Serialize;
use tracing::{error, info, instrument, trace, warn};

use crate::db::extractor::DatabaseConnection;
use crate::db::users::UserRepository;
use crate::domain::auth::decode_token;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

#[instrument(skip(conn, cookie_jar, req, next))]
pub async fn auth_middleware(
    DatabaseConnection(mut conn): DatabaseConnection,
    cookie_jar: CookieJar,
    mut req: Request,
    next: Next,
) -> crate::Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    info!("{:#?}", req.headers());
    let token = cookie_jar
        .get("token")
        .map(|cookie| {
            trace!("Found token in cookie");
            cookie.value().to_string()
        })
        .or_else(|| {
            let bearer = req.headers().typed_get::<Authorization<Bearer>>();
            bearer.map(|bearer| {
                trace!("Found token in Bearer header");
                bearer.token().to_string()
            })
        });

    let token = token.ok_or_else(|| {
        warn!("No token found in request");
        let json_error = ErrorResponse {
            status: "fail",
            message: "You are not logged in, please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

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

    let user_id = claims.sub;
    let user_repo = UserRepository {};
    let user = user_repo.find_by_id(&mut conn, &user_id).map_err(|e| {
        error!("Error fetching user from database: {}", e);
        let json_error = ErrorResponse {
            status: "fail",
            message: format!("Error fetching user from database: {}", e),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
    })?;

    let user = user.ok_or_else(|| {
        error!("User not found in database");
        let json_error = ErrorResponse {
            status: "fail",
            message: "The user belonging to this token no longer exists".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
