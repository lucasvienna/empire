use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher, password_hash};

use crate::configuration::JwtSettings;
use crate::domain::auth::{AuthError, Claims, encode_token};
use crate::domain::player::Player;

/// Hashes a password using the Argon2id algorithm with a randomly generated salt.
///
/// # Parameters
/// - `pwd`: The password to be hashed. Accepts any type that can be referenced as a slice of bytes (`AsRef<[u8]>`).
///
/// # Returns
/// - `Ok(String)`: Contains the password hash in PHC string format (e.g., `$argon2id$v=19$...`).
/// - `Err(password_hash::Error)`: If there is an issue with hashing the password.
pub fn hash_password(pwd: impl AsRef<[u8]>) -> Result<String, password_hash::Error> {
	let salt = SaltString::generate(&mut OsRng);
	// Argon2 with default params (Argon2id v19)
	let argon2 = Argon2::default();

	// Hash password to PHC string ($argon2id$v=19$...)
	let password_hash = argon2.hash_password(pwd.as_ref(), &salt)?.to_string();

	Ok(password_hash)
}

/// Creates a JSON Web Token (JWT) for a player with the provided settings.
///
/// This function generates a JWT containing the claim information of the player. The token
/// includes a subject (`sub`, the player ID), an expiry time (`exp`), and an issued-at time (`iat`).
/// It uses the secret configured in the `LazyLock` `KEYS` to sign the token.
///
/// # Parameters
/// - `player`: The `User` for whom the token is being generated.
/// - `jwt_settings`: Configuration settings containing the expiration time for the token.
///
/// # Returns
/// - `Ok(String)`: The generated JWT token as a string if no errors occur.
/// - `Err(AuthError)`: If token generation fails (e.g., due to encoding issues).
///
/// # Errors
/// - Returns `AuthError::TokenCreation` if encoding the token fails.
pub fn create_token_for_user(
	user: Player,
	jwt_settings: &JwtSettings,
) -> Result<String, AuthError> {
	let now = chrono::Utc::now();
	let expires_in = chrono::Duration::seconds(jwt_settings.expires_in as i64);
	let claims = Claims {
		sub: user.id,
		exp: (now + expires_in).timestamp() as usize,
		iat: now.timestamp() as usize,
	};

	let token = encode_token(claims)?;

	Ok(token)
}
