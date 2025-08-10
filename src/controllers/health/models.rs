use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Struct representing the health check response
#[derive(Serialize, Deserialize)]
pub struct HealthCheckBody {
	pub status: String,
	pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceReadiness {
	pub database: bool,
	pub queue: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ReadyCheckBody {
	pub ready: bool,
	pub services: ServiceReadiness,
}

#[derive(Serialize, Deserialize)]
pub struct LivenessCheckBody {
	pub alive: bool,
	pub uptime: String,
}
