pub mod auth;
pub mod dashboard;
pub mod game;
pub mod health;
pub mod player;
pub mod user;

pub mod routes {
	pub use crate::controllers::auth::{auth_routes, protected_auth_routes};
	pub use crate::controllers::game::game_routes;
	pub use crate::controllers::health::health_routes;
	pub use crate::controllers::player::player_routes;
	pub use crate::controllers::user::user_routes;
}
