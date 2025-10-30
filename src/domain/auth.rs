use std::fmt::{Display, Formatter};
use std::sync::OnceLock;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::{Json, RequestPartsExt};
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use derive_more::Deref;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::warn;
use uuid::Uuid;

use crate::configuration::Settings;
use crate::domain::player::Player;

/// Static secret keys holder used for encoding and decoding JWTs.
/// Using `OnceLock` ensures one-time initialization.
/// This avoids repeatedly retrieving the secret and creating keys.
static KEYS: OnceLock<Keys> = OnceLock::new();

/// One-time initialization of the JWT keys.
/// This must be called before any JWT operations are performed.
pub fn init_keys(secret: &SecretString) {
	if KEYS
		.set(Keys::new(secret.expose_secret().as_bytes()))
		.is_err()
	{
		warn!("JWT keys were already initialized");
	}
}

/// Retrieves the initialized keys.
/// Panics if `init_keys` has not been called.
fn keys() -> &'static Keys {
	KEYS.get()
		.expect("JWT keys must be initialized with `init_keys`")
}

/// Container struct that encapsulates JWT encoding and decoding keys.
pub struct Keys {
	encoding: EncodingKey,
	decoding: DecodingKey,
}

impl Keys {
	/// Constructs new encoding and decoding keys from the given secret.
	pub fn new(secret: &[u8]) -> Self {
		Self {
			encoding: EncodingKey::from_secret(secret),
			decoding: DecodingKey::from_secret(secret),
		}
	}
}

/// JWT claims struct holding essential token data.
/// Fields follow JWT claim conventions: subject ID, expiration, issued-at.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
	pub sub: Uuid,  // User identifier associated with the token
	pub exp: usize, // Expiration time (timestamp)
	pub iat: usize, // Issued at time (timestamp)
}

impl Display for Claims {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		// Custom Display implementation for convenient logging/debugging.
		write!(
			f,
			"Claims {{ sub: {}, exp: {}, iat: {} }}",
			self.sub, self.exp, self.iat
		)
	}
}

/// Extractor implementation that extracts and verifies JWT Claims from request headers.
/// Utilizes Axum's FromRequestParts trait and integrates with app state for settings.
///
/// This replaces manually extracting and decoding tokens in handlers,
/// enabling clean and consistent authorization extraction.
impl<S> FromRequestParts<S> for Claims
where
	S: Send + Sync,
	Settings: FromRef<S>,
{
	type Rejection = AuthError;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		// Extract Bearer token from the Authorization header using typed headers.
		let TypedHeader(Authorization(bearer)) = parts
			.extract::<TypedHeader<Authorization<Bearer>>>()
			.await
			.map_err(|_| AuthError::InvalidToken)?;

		// Decode and validate the token using the pre-initialized decoding key.
		decode::<Claims>(bearer.token(), &keys().decoding, &Validation::default())
			.map(|token_data| token_data.claims)
			.map_err(|_| AuthError::InvalidToken)
	}
}

/// Response body struct for returning an access token and token type name.
/// Common pattern in OAuth2-like authentication responses.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthBody {
	pub access_token: String,
	pub token_type: String,
}

impl AuthBody {
	/// Convenient constructor setting token_type to the conventional "Bearer".
	pub fn new(access_token: String) -> Self {
		Self {
			access_token,
			token_type: "Bearer".to_string(),
		}
	}
}

/// Enumerates all authentication-related error cases in a clear and concise way.
#[derive(Debug)]
pub enum AuthError {
	WrongCredentials,
	MissingCredentials,
	TokenCreation,
	ArgonError,
	InvalidToken,
	MissingSession,
	MismatchedModality,
}

impl AuthError {
	/// Creates a new AuthError with a custom status code and message.
	pub fn new(status: StatusCode, message: &str) -> Self {
		match status {
			StatusCode::BAD_REQUEST if message.contains("session") => Self::MissingSession,
			StatusCode::BAD_REQUEST if message.contains("modality") => Self::MismatchedModality,
			_ => {
				warn!(
					"Using custom error handler with status: {}, message: {}",
					status, message
				);
				Self::MismatchedModality
			}
		}
	}
}

impl IntoResponse for AuthError {
	/// Converts an AuthError into an HTTP response with an appropriate status code and JSON body.
	fn into_response(self) -> Response {
		let (status, error_message) = match self {
			AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
			AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
			AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
			AuthError::ArgonError => (StatusCode::INTERNAL_SERVER_ERROR, "Cryptographic error"),
			AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
			AuthError::MissingSession => (StatusCode::BAD_REQUEST, "Missing session"),
			AuthError::MismatchedModality => {
				(StatusCode::BAD_REQUEST, "Authentication modality mismatch")
			}
		};
		let body = json!({ "error": error_message });
		(status, Json(body)).into_response()
	}
}

/// Encodes claims into a JWT token string using the static encoding key.
///
/// Returns a signed, base64-encoded JWT string on success, otherwise returns a token creation error.
pub fn encode_token(claims: Claims) -> Result<String, AuthError> {
	encode(&Header::default(), &claims, &keys().encoding).map_err(|_| AuthError::TokenCreation)
}

/// Decodes a JWT token string into Claims, verifying using the static decoding key.
///
/// Returns decoded claims on success or an invalid token error if validation fails.
pub fn decode_token(token: &str) -> Result<TokenData<Claims>, AuthError> {
	decode::<Claims>(token, &keys().decoding, &Validation::default())
		.map_err(|_| AuthError::InvalidToken)
}

/// Represents an authenticated user, regardless of the authentication method.
#[derive(Debug, Clone, Deref)]
pub struct AuthenticatedUser(pub Player);
