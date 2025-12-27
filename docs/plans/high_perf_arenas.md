# Request Context Pattern for High-Performance Operations

## When Does It Make Sense?

The request context pattern with memory arenas becomes valuable when you have:

### **Hot Path Indicators**

- **High request volume** (>1000 RPS per endpoint)
- **Frequent temporary allocations** during request processing
- **Complex JSON serialization** with many nested objects
- **String processing** (parsing, formatting, concatenation)
- **Collection building** (Vec, HashMap creation/manipulation)
- **Database result transformation** into response DTOs

### **Performance Bottleneck Signals**

- Memory profiling shows high allocation rates
- Garbage collection pressure (even in Rust, allocator overhead)
- Cache misses from scattered heap allocations
- Latency spikes correlating with allocation patterns

### **Game-Specific Scenarios**

- **Leaderboard generation** with hundreds of players
- **Resource calculation** across many buildings/modifiers
- **Battle resolution** with multiple participants
- **Map data serialization** for large game worlds

## Implementation Strategy

### Phase 1: Request Context Structure

```rust
use bumpalo::Bump;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Request-scoped memory management for high-performance operations
pub struct RequestContext {
	/// Bump allocator for request-lifetime allocations
	pub arena: Bump,
	/// Reusable string buffer for formatting operations
	pub string_buffer: String,
	/// Reusable byte buffer for serialization
	pub scratch_buffer: Vec<u8>,
	/// Object pools for common types
	pub pools: ObjectPools,
}

/// Pooled objects that can be reused across requests
pub struct ObjectPools {
	/// Pre-allocated Vec<GameBuilding> for building lists
	building_vecs: Arc<Mutex<Vec<Vec<GameBuilding>>>>,
	/// Pre-allocated HashMap for resource calculations
	resource_maps: Arc<Mutex<Vec<HashMap<String, i64>>>>,
	/// String pools for JSON keys and common values
	string_pools: Arc<Mutex<Vec<String>>>,
}

impl RequestContext {
	pub fn new() -> Self {
		Self {
			arena: Bump::with_capacity(16384), // 16KB initial capacity
			string_buffer: String::with_capacity(2048),
			scratch_buffer: Vec::with_capacity(4096),
			pools: ObjectPools::new(),
		}
	}

	/// Allocate a string slice that lives for the request duration
	pub fn alloc_str(&self, s: &str) -> &str {
		self.arena.alloc_str(s)
	}

	/// Allocate and format a string in the arena
	pub fn alloc_fmt(&self, args: std::fmt::Arguments) -> &str {
		use std::fmt::Write;
		let mut temp = bumpalo::collections::String::new_in(&self.arena);
		temp.write_fmt(args).unwrap();
		temp.into_bump_str()
	}

	/// Get a reusable Vec<GameBuilding> from the pool
	pub fn get_building_vec(&self) -> PooledVec<GameBuilding> {
		self.pools.get_building_vec()
	}

	/// Get a reusable HashMap for resource calculations
	pub fn get_resource_map(&self) -> PooledHashMap<String, i64> {
		self.pools.get_resource_map()
	}

	/// Reset all buffers for reuse (called between requests)
	pub fn reset(&mut self) {
		self.arena.reset();
		self.string_buffer.clear();
		self.scratch_buffer.clear();
		// Pools handle their own reset
	}
}
```

### Phase 2: Axum Extractor

```rust
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Thread-local request context pool
thread_local! {
    static CONTEXT_POOL: Arc<Mutex<Vec<RequestContext>>> =
        Arc::new(Mutex::new(Vec::with_capacity(10)));
}

/// Extractor that provides a request context with memory arenas
pub struct ContextualizedRequest {
	pub conn: DbConn,
	pub context: RequestContext,
}

impl<S> FromRequestParts<S> for ContextualizedRequest
where
	S: Send + Sync,
	AppPool: FromRef<S>,
{
	type Rejection = (StatusCode, String);

	async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		// Get database connection
		let pool = AppPool::from_ref(state);
		let conn = pool.get().map_err(|err| {
			error!("Failed to get database connection: {}", err);
			(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
		})?;

		// Get or create request context
		let context = CONTEXT_POOL.with(|pool| {
			let mut pool = pool.try_lock().ok()?;
			pool.pop().unwrap_or_else(RequestContext::new)
		}).unwrap_or_else(RequestContext::new);

		Ok(Self { conn, context })
	}
}

impl Drop for ContextualizedRequest {
	fn drop(&mut self) {
		// Return context to pool for reuse
		self.context.reset();
		CONTEXT_POOL.with(|pool| {
			if let Ok(mut pool) = pool.try_lock() {
				if pool.len() < 10 { // Max pool size
					pool.push(std::mem::replace(&mut self.context, RequestContext::new()));
				}
			}
		});
	}
}
```

### Phase 3: Pooled Collections

```rust
use std::ops::{Deref, DerefMut};

/// A Vec<T> borrowed from a pool that automatically returns on drop
pub struct PooledVec<T> {
	vec: Vec<T>,
	pool: Arc<Mutex<Vec<Vec<T>>>>,
}

impl<T> Deref for PooledVec<T> {
	type Target = Vec<T>;
	fn deref(&self) -> &Self::Target { &self.vec }
}

impl<T> DerefMut for PooledVec<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.vec }
}

impl<T> Drop for PooledVec<T> {
	fn drop(&mut self) {
		self.vec.clear();
		if let Ok(mut pool) = self.pool.try_lock() {
			if pool.len() < 20 { // Max pool size
				pool.push(std::mem::take(&mut self.vec));
			}
		}
	}
}

impl ObjectPools {
	pub fn new() -> Self {
		Self {
			building_vecs: Arc::new(Mutex::new(Vec::new())),
			resource_maps: Arc::new(Mutex::new(Vec::new())),
			string_pools: Arc::new(Mutex::new(Vec::new())),
		}
	}

	pub fn get_building_vec(&self) -> PooledVec<GameBuilding> {
		let vec = self.building_vecs
			.try_lock()
			.ok()
			.and_then(|mut pool| pool.pop())
			.unwrap_or_else(Vec::new);

		PooledVec {
			vec,
			pool: Arc::clone(&self.building_vecs),
		}
	}
}
```

### Phase 4: High-Performance Handler Pattern

```rust
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

// Hot path handler using request context
pub async fn get_player_leaderboard(
	ContextualizedRequest { mut conn, context }: ContextualizedRequest,
) -> impl IntoResponse {
	// Use arena for temporary string allocations
	let region_filter = context.alloc_str("global");

	// Use pooled collections to avoid allocations
	let mut players = context.get_building_vec();

	// Perform database operations
	let raw_players = player_operations::get_leaderboard_players(&mut conn, region_filter)?;

	// Transform using arena allocations
	for raw_player in raw_players {
		let formatted_name = context.alloc_fmt(format_args!(
			"[{}] {}",
			raw_player.faction_code,
			raw_player.name
		));

		players.push(GameBuilding {
			name: formatted_name.to_owned(), // Still need owned for response
			// ... other fields
		});
	}

	// Serialize efficiently (could use scratch_buffer for custom serialization)
	Json(LeaderboardResponse {
		players: players.clone(), // Vec is pooled and will be returned
		region: region_filter,
	})
}
```

### Phase 5: Zero-Copy Serialization (Advanced)

```rust
use serde::ser::{Serialize, Serializer, SerializeStruct};

/// Custom serialization directly into scratch buffer
pub struct ArenaSerializedResponse<'ctx> {
	context: &'ctx RequestContext,
	data: &'ctx [GameBuilding],
}

impl<'ctx> Serialize for ArenaSerializedResponse<'ctx> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// Custom serialization logic that uses arena-allocated strings
		let mut state = serializer.serialize_struct("LeaderboardResponse", 2)?;

		// Use pre-allocated strings from arena
		state.serialize_field("players", self.data)?;
		state.serialize_field("timestamp",
		                      self.context.alloc_fmt(format_args!("{}", chrono::Utc::now())))?;

		state.end()
	}
}
```

## Performance Considerations

### **Memory Layout Benefits**

- **Contiguous allocations**: Arena allocations are sequential, improving cache locality
- **No fragmentation**: Bump allocator never fragments memory
- **Batch deallocation**: Entire request's allocations freed at once

### **When NOT to Use**

- **Low-traffic endpoints** (<100 RPS)
- **Simple CRUD operations** with minimal data transformation
- **Short-lived requests** with minimal allocations
- **Endpoints with large persistent data** (arena overhead becomes significant)

### **Benchmarking Strategy**

```rust
#[cfg(test)]
mod benches {
	use criterion::{black_box, criterion_group, criterion_main, Criterion};

	fn bench_with_arena(c: &mut Criterion) {
		c.bench_function("leaderboard_with_arena", |b| {
			b.iter(|| {
				let context = RequestContext::new();
				// Benchmark arena-based implementation
				black_box(process_leaderboard_with_arena(&context))
			})
		});
	}

	fn bench_without_arena(c: &mut Criterion) {
		c.bench_function("leaderboard_standard", |b| {
			b.iter(|| {
				// Benchmark standard implementation
				black_box(process_leaderboard_standard())
			})
		});
	}
}
```

## Migration Path

1. **Identify hot paths** through profiling
2. **Implement basic RequestContext** extractor
3. **Convert one high-traffic endpoint** as proof of concept
4. **Measure performance improvement** (aim for >10% latency reduction)
5. **Gradually migrate** other performance-critical handlers
6. **Add object pooling** for frequently allocated types
7. **Implement zero-copy serialization** for largest responses

The request context pattern is most effective when you have clear evidence of allocation-heavy hot
paths. Start small, measure everything, and expand based on concrete performance gains.
