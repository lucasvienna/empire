//! Modifier system management module for game effects and bonuses.
//!
//! This module provides a centralised system for managing temporary and permanent modifiers
//! that affect various game aspects such as resource generation, building costs, and combat stats.
//! It combines caching and scheduling capabilities to efficiently handle modifier lifecycle and effects.
//!
//! # Key Components
//!
//! * Cache Management - Maintains cached modifier calculations with automatic invalidation
//! * Job Scheduling - Handles modifier expiration and periodic recalculations
//! * Thread Safety - All components are wrapped in Arc for safe concurrent access
//!
//! # Architecture
//!
//! The system consists of two main components:
//!
//! 1. `ModifierCache` - Caches calculated modifier effects with TTL-based invalidation
//! 2. `ModifierScheduler` - Manages background jobs for modifier lifecycle events
//!
//! # State Management
//!
//! The `ModifierSystem` struct serves as the main entry point and maintains shared state
//! between the cache and scheduler components. It provides methods for creating new instances
//! either directly or through integration with the application's job queue.

use std::sync::Arc;

use chrono::Duration;

use crate::configuration::Settings;
use crate::domain::app_state::AppQueue;
use crate::game::modifiers::modifier_cache::ModifierCache;
use crate::game::modifiers::modifier_scheduler::ModifierScheduler;

/// Central coordinator for the modifier subsystem that manages caching and scheduling.
#[derive(Clone)]
pub struct ModifierSystem {
	/// Thread-safe reference to the modifier calculation cache
	pub cache: Arc<ModifierCache>,
	/// Thread-safe reference to the modifier job scheduler
	pub scheduler: Arc<ModifierScheduler>,
}

impl ModifierSystem {
	/// Creates a new `ModifierSystem` instance with provided cache and scheduler components.
	///
	/// # Arguments
	///
	/// * `cache` - Arc-wrapped `ModifierCache` instance for caching modifier calculations
	/// * `scheduler` - Arc-wrapped `ModifierScheduler` instance for managing modifier jobs
	pub fn new(cache: Arc<ModifierCache>, scheduler: Arc<ModifierScheduler>) -> Self {
		Self { cache, scheduler }
	}

	/// Creates a new `ModifierSystem` instance integrated with the application's job queue.
	///
	/// # Arguments
	///
	/// * `settings` - Application settings containing cache configuration
	/// * `job_queue` - Reference to the application's job queue for scheduling
	pub fn with_job_queue(settings: &Settings, job_queue: &AppQueue) -> Self {
		// TODO: get these from settings
		let cache = Arc::new(ModifierCache::new(Duration::hours(1), 100));
		let scheduler = Arc::new(ModifierScheduler::new(job_queue));
		Self { cache, scheduler }
	}
}
