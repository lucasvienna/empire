use std::fmt::{Display, Formatter};
use std::sync::LazyLock;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::configuration::Settings;

/// JWT secret keys used for encoding and decoding tokens.
/// This static instance is initialized lazily on first access.
static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // User associated with the token
    pub exp: usize, // Expiry time of the token
    pub iat: usize, // Issued at time of the token
}

impl Display for Claims {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Claims {{ sub: {}, exp: {}, iat: {} }}",
            self.sub, self.exp, self.iat
        )
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
    Settings: axum::extract::FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let settings = Settings::from_ref(state);
        let token = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    ArgonError,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::ArgonError => (StatusCode::INTERNAL_SERVER_ERROR, "Cryptographic error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = json!({ "error": error_message });
        (status, Json(body)).into_response()
    }
}

/// Encodes the given JWT `Claims` into a token string.
///
/// This function uses the `encoding` key stored in the `KEYS` static instance
/// to sign the provided `Claims` and produce a Base64-encoded, signed JWT token.
///
/// ### Returns:
/// - `Ok(String)`: A signed JWT token string, if the encoding operation is successful.
/// - `Err(AuthError)`: Returns `AuthError::TokenCreation` if the token creation fails due
///   to any reason (e.g., invalid key or internal encoding error).
pub fn encode_token(claims: Claims) -> Result<String, AuthError> {
    encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)
}

/// Decodes the given JWT token string into `Claims`.
///
/// This function uses the `decoding` key stored in the `KEYS` static instance
/// to verify the provided token and extract its claims. The token is expected
/// to be a Base64-encoded, signed JWT token.
///
/// ### Parameters:
/// - `token` (`&str`): The JWT token string to decode.
///
/// ### Returns:
/// - `Ok(Claims)`: Decoded claims from the token if the decoding operation is successful.
/// - `Err(AuthError)`: Returns `AuthError::InvalidToken` if the token is invalid,
///   has expired, or cannot be decoded (e.g., due to an incorrect signature or a malformed token).
pub fn decode_token(token: &str) -> Result<TokenData<Claims>, AuthError> {
    decode::<Claims>(token, &KEYS.decoding, &Validation::default())
        .map_err(|_| AuthError::InvalidToken)
}
