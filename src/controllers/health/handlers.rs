use std::time::Duration;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Json, debug_handler};
use chrono::TimeDelta;

use crate::controllers::health::models::{
	HealthCheckBody, LivenessCheckBody, ReadyCheckBody, ServiceReadiness,
};
use crate::domain::app_state::{AppPool, AppQueue, AppState};
use crate::not_implemented;

/// Health check handler
#[debug_handler]
pub(super) async fn health_check() -> impl IntoResponse {
	let body = HealthCheckBody {
		status: "OK".to_string(),
		timestamp: chrono::Utc::now(),
	};
	Json(body)
}

#[debug_handler(state = AppState)]
pub(super) async fn readiness_check(
	State(pool): State<AppPool>,
	State(queue): State<AppQueue>,
) -> impl IntoResponse {
	let db_state = pool.state();
	let q_state = queue.state();
	let body = ReadyCheckBody {
		ready: true,
		services: ServiceReadiness {
			database: db_state.connections > 0,
			queue: q_state.up,
		},
	};
	Json(body)
}

#[debug_handler]
pub(super) async fn liveness_check() -> impl IntoResponse {
	let uptime = TimeDelta::from_std(get_uptime().unwrap_or_default()).unwrap_or_default();
	let body = LivenessCheckBody {
		alive: true,
		uptime: format_uptime(uptime),
	};
	Json(body)
}

fn format_uptime(duration: TimeDelta) -> String {
	let days = duration.num_days();
	let hours = duration.num_hours() % 24;
	let minutes = duration.num_minutes() % 60;
	let seconds = duration.num_seconds() % 60;

	format!("{days:02}d{hours:02}h{minutes:02}m{seconds:02}s")
}

fn get_uptime() -> Option<Duration> {
	let created = std::fs::metadata("/proc/self").ok()?.modified().ok()?;
	let now = std::time::SystemTime::now();

	now.duration_since(created).ok()
}

#[debug_handler]
pub(super) async fn services() -> impl IntoResponse {
	not_implemented!()
}

#[debug_handler]
pub(super) async fn metrics() -> impl IntoResponse {
	not_implemented!()
}
