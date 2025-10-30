use axum::Router;
use axum::routing::get;

use crate::controllers::user::handlers::{
	create_user, delete_user, get_user_by_id, get_users, update_user,
};
use crate::domain::app_state::AppState;

pub fn user_routes() -> Router<AppState> {
	Router::new().nest(
		"/users",
		Router::new()
			.route("/", get(get_users).post(create_user))
			.route(
				"/{id}",
				get(get_user_by_id).put(update_user).delete(delete_user),
			),
	)
}
