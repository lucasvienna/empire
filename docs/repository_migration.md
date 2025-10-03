# Repository Migration Plan: From Structs to Function Modules

## Executive Summary

This document outlines the migration from Java-style repository structs to idiomatic Rust function modules. The goal is
to improve performance by eliminating unnecessary struct overhead while maintaining type safety and code organization.

## Background

### Current Architecture Issues

- Repository structs are lightweight facades around `Arc<DbPool>`
- Java/.NET influenced patterns don't align with Rust's functional strengths
- Unnecessary heap allocations for struct instantiation
- Trait dispatch overhead for simple database operations
- **Multiple connection acquisitions per request** in handlers

### Performance Motivation

- Eliminate repository struct allocation per request
- Enable better compiler optimizations through direct function calls
- Reduce call stack complexity and pointer indirection
- **Single connection per request** instead of multiple `pool.get()` calls
- Prepare foundation for request-scoped memory arenas

## Migration Strategy

### Phase 1: Core Module Structure

**Objective**: Convert repository structs to pure function modules

**Target Files**:

- `src/db/player_buildings.rs`
- `src/db/buildings.rs`
- `src/db/players.rs`
- `src/db/resources.rs`
- `src/db/modifiers.rs`

**Transformation Pattern**:

```rust
// Before: Repository struct with methods
pub struct PlayerBuildingRepository {
	pool: AppPool,
}

impl PlayerBuildingRepository {
	pub fn get_player_buildings(&self, player_key: &PlayerKey) -> Result<Vec<PlayerBuilding>> {
		let mut conn = self.pool.get()?;  // ← Connection acquired per method call
		// query logic...
	}
}

// After: Module with functions taking direct connection
pub fn get_player_buildings(conn: &mut DbConn, player_key: &PlayerKey) -> Result<Vec<PlayerBuilding>> {
	// same query logic, no connection acquisition overhead
}
```

**Key Changes**:

1. Remove struct definition and `impl` blocks
2. Convert methods to standalone functions
3. **First parameter becomes `conn: &mut DbConn`** (not pool reference)
4. **Eliminate per-function connection acquisition** - handlers manage connection lifecycle
5. Remove `FromRef<AppState>` and `Debug` implementations
6. Keep type aliases like `FullBuilding` and `UpgradeTuple`

**Architectural Insight**: The pool reference in repository structs was merely an artifact of the struct pattern. Using
the `DatabaseConnection` extractor allows each request to acquire a single connection that can be reused across all
database operations within that request.

### Phase 2: Handler Migration with DatabaseConnection Extractor

**Objective**: Update HTTP handlers to use the `DatabaseConnection` extractor and function calls

**Handler Pattern Changes**:

```rust
// Before: Multiple state extractions + multiple connection acquisitions
#[instrument(skip(srv, repo, player))]
pub async fn upgrade_building(
	State(srv): State<BuildingService>,
	State(repo): State<PlayerBuildingRepository>,
	player_bld_key: Path<PlayerBuildingKey>,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	let bld = srv.upgrade_building(&building_key)?;  // srv.pool.get() inside
	let res = repo.get_game_building(&player_key, &bld.id)  // repo.pool.get() inside
		.map(GameBuilding::from)?;
	Ok(json!(res))
}

// After: Single connection extraction + direct function calls
#[instrument(skip(conn, player))]
pub async fn upgrade_building(
	DatabaseConnection(mut conn): DatabaseConnection,
	player_bld_key: Path<PlayerBuildingKey>,
	player: Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse> {
	let bld = building_operations::upgrade_building(&mut conn, &building_key)?;
	let res = player_buildings::get_game_building(&mut conn, &player_key, &bld.id)
		.map(GameBuilding::from)?;
	Ok(json!(res))
}
```

**Benefits**:

- **Single connection per request**: Eliminates multiple `pool.get()` calls within handlers
- **Better resource utilization**: Connection reused across all database operations
- **Simpler function signatures**: Functions just take `conn: &mut DbConn`
- **Enhanced testability**: Easy to mock/stub a connection vs. mocking a pool
- **Transaction support**: Natural boundary for request-scoped transactions
- **Eliminates needless indirection**: Pool extraction happens once at handler boundary

### Phase 3: Service Layer Migration

**Objective**: Apply same pattern to service structs

**Target Services**:

- `BuildingService` → `building_operations` module
- `ModifierService` → `modifier_operations` module
- `ResourceService` → `resource_operations` module
- `SessionService` → `session_operations` module
- `PlayerService` → `player_operations` module

**Service Transformation Example**:

```rust
// Before: BuildingService struct
pub struct BuildingService {
	pool: AppPool,
	modifier_system: ModifierSystem,
}

impl BuildingService {
	pub fn upgrade_building(&self, building_key: &PlayerBuildingKey) -> Result<PlayerBuilding> {
		let mut conn = self.pool.get()?;  // ← Connection acquired per service method
		// upgrade logic...
	}
}

// After: building_operations module
pub fn upgrade_building(
	conn: &mut DbConn,
	modifier_system: &ModifierSystem,
	building_key: &PlayerBuildingKey
) -> Result<PlayerBuilding> {
	// same upgrade logic, using provided connection
}
```

### Phase 4: Request Context Framework (Future)

**Objective**: Implement request-scoped memory management for high-performance operations

**Request Context Structure**:

```rust
pub struct RequestContext {
	// Pre-allocated buffers to avoid heap churn
	scratch_buffer: Vec<u8>,
	string_buffer: String,
	// Bump allocator for request-scoped data
	arena: bumpalo::Bump,
	// Reusable connection holder
	conn_holder: Option<DbConn>,
}

impl RequestContext {
	pub fn new() -> Self {
		Self {
			scratch_buffer: Vec::with_capacity(8192),
			string_buffer: String::with_capacity(2048),
			arena: bumpalo::Bump::with_capacity(16384),
			conn_holder: None,
		}
	}

	pub fn alloc_str(&self, s: &str) -> &str {
		self.arena.alloc_str(s)
	}

	pub fn reset(&mut self) {
		self.scratch_buffer.clear();
		self.string_buffer.clear();
		self.arena.reset();
		self.conn_holder = None;
	}
}
```

## Implementation Details

### Module Organization

```
src/db/
├── player_buildings.rs     // Functions: get_*, create_*, update_*, delete_*
├── buildings.rs            // Functions: lookup_*, validate_*, calculate_*
├── players.rs             // Functions: authenticate_*, profile_*, settings_*
├── resources.rs           // Functions: calculate_*, update_*, get_*
├── modifiers.rs           // Functions: apply_*, expire_*, calculate_*
├── active_modifiers.rs    // Functions: get_*, create_*, cleanup_*
└── mod.rs                 // Re-exports and common types
```

### Function Naming Conventions

**Database Operations**:

- **Queries**: `get_*`, `find_*`, `list_*`, `lookup_*`
- **Mutations**: `create_*`, `update_*`, `delete_*`, `insert_*`
- **Validations**: `can_*`, `validate_*`, `check_*`, `verify_*`
- **Calculations**: `calculate_*`, `compute_*`, `determine_*`

**Business Logic**:

- **Actions**: `upgrade_*`, `construct_*`, `apply_*`, `process_*`
- **Transformations**: `convert_*`, `transform_*`, `map_*`
- **Aggregations**: `summarize_*`, `total_*`, `count_*`

### Type Definitions Preservation

Keep all existing type aliases and domain structures:

```rust
// Complex query result types
pub type FullBuilding = (PlayerBuilding, Building, BuildingLevel, BuildingResource);
pub type UpgradeTuple = (PlayerBuilding, Option<i32>);

// Domain key types
pub use crate::domain::player::PlayerKey;
pub use crate::domain::building::BuildingKey;
pub use crate::domain::player::buildings::PlayerBuildingKey;
```

### Error Handling Pattern

Maintain consistent error handling across all functions:

```rust
pub fn get_player_buildings(conn: &mut DbConn, player_key: &PlayerKey) -> Result<Vec<PlayerBuilding>> {
	let buildings = player_building::table
		.filter(player_id.eq(player_key))
		.get_results(conn)
		.map_err(|e| (ErrorKind::DatabaseError, format!("Query failed: {}", e)))?;

	Ok(buildings)
}
```

**Note**: Connection acquisition errors are handled at the handler level by the `DatabaseConnection` extractor,
eliminating the need for connection-related error handling within individual functions.

## Performance Expected Improvements

### Quantifiable Benefits

**Memory Efficiency**:

- Eliminate ~64-128 bytes per repository struct per request
- Reduce allocator pressure from struct instantiation
- **Single connection per request** instead of multiple connection acquisitions
- Better memory locality with direct function calls

**CPU Performance**:

- Remove trait dispatch overhead (1-3 CPU cycles per call)
- **Eliminate repeated `pool.get()` calls** within request handlers
- Enable aggressive function inlining optimizations
- Reduce call stack depth and complexity
- Better branch prediction with direct calls

**Cache Efficiency**:

- Improved instruction cache utilization
- Reduced pointer chasing through struct fields
- **Connection reuse** reduces connection pool contention
- Better data locality in hot code paths

**Database Performance**:

- **Reduced connection pool pressure** from fewer acquisitions per request
- **Natural transaction boundaries** at request scope
- Lower latency from connection reuse within handlers

### Measurable Metrics

**Latency Improvements**:

- Expected 5-15% reduction in handler latency for database-heavy operations
- Reduced 95th percentile response times
- Lower memory allocation rates per request

**Throughput Gains**:

- Increased requests per second under load
- Better CPU utilization efficiency
- Reduced memory fragmentation

## Risk Assessment

### Low Risk Changes

- **Function signature preservation**: Same inputs/outputs, just different calling convention
- **Query logic unchanged**: All existing Diesel queries remain identical
- **Type safety maintained**: Compile-time verification of all changes
- **Error handling preserved**: Same error types and propagation patterns

### Moderate Risk Areas

- **Import path changes**: All handler files need updated import statements
- **Handler routing**: Function calls replace method calls (compilation catches issues)
- **Test suite updates**: Unit tests need new function call patterns
- **Documentation updates**: API docs and inline comments need revision

### High Risk Areas

- **Concurrency assumptions**: Ensure no hidden state dependencies in structs
- **Transaction boundaries**: Verify transaction scoping works with function calls
- **Connection management**: Ensure proper connection lifecycle management

### Mitigation Strategies

**Incremental Migration**:

- Convert one repository at a time
- Maintain parallel implementations during transition
- Use feature flags to toggle between old/new patterns

**Comprehensive Testing**:

- Unit tests for each converted function
- Integration tests for handler behavior
- Performance regression testing
- Load testing to verify improvements

**Performance Validation**:

- Benchmark each phase completion
- Memory profiling before/after each change
- CPU profiling for hot code paths
- Latency distribution analysis

**Rollback Preparedness**:

- Git branching strategy with clear rollback points
- Automated deployment rollback procedures
- Performance monitoring alerting
- Gradual rollout with canary deployments

## Implementation Timeline

### Week 1: Foundation & Proof of Concept

**Goals**: Establish pattern and validate approach

- [ ] **Day 1-2**: Convert `PlayerBuildingRepository` to function module with `&mut DbConn` parameters
- [ ] **Day 3**: Update handlers to use `DatabaseConnection` extractor + converted functions
- [ ] **Day 4**: Write migration script template emphasizing connection-first approach
- [ ] **Day 5**: Establish benchmarking baseline comparing connection reuse vs. acquisition overhead

**Deliverables**:

- Working function-based player buildings module with connection parameters
- Updated handlers using `DatabaseConnection` extractor pattern
- Performance measurements showing reduced connection acquisition overhead
- Migration template for remaining repositories

### Week 2: Core Repository Migration

**Goals**: Complete all database layer conversions

- [ ] **Day 1**: Convert `BuildingRepository`, `PlayerRepository`
- [ ] **Day 2**: Convert `ResourceRepository`, `ModifierRepository`
- [ ] **Day 3**: Convert `ActiveModifiersRepository`, remaining repositories
- [ ] **Day 4**: Update all affected handlers
- [ ] **Day 5**: Full test suite validation and performance analysis

**Deliverables**:

- All repository structs converted to function modules
- All handlers updated to new pattern
- Performance comparison report

### Week 3: Service Layer Migration

**Goals**: Apply pattern to service layer

- [ ] **Day 1**: Convert `BuildingService` to `building_operations`
- [ ] **Day 2**: Convert `ModifierService` to `modifier_operations`
- [ ] **Day 3**: Convert remaining services (`ResourceService`, `SessionService`)
- [ ] **Day 4**: Update handlers for service changes
- [ ] **Day 5**: Integration testing across all endpoints

**Deliverables**:

- All service structs converted to function modules
- Comprehensive integration test results
- Updated API documentation

### Week 4: Optimization & Documentation

**Goals**: Polish and optimize the new architecture

- [ ] **Day 1**: Code review and cleanup of converted modules
- [ ] **Day 2**: Performance tuning based on profiling results
- [ ] **Day 3**: Documentation updates and inline comments
- [ ] **Day 4**: Final performance validation and benchmarking
- [ ] **Day 5**: Prepare foundation for request context implementation

**Deliverables**:

- Optimized function modules with performance improvements
- Complete documentation update
- Request context design document

## Success Criteria

### Functional Requirements

- [ ] **API Compatibility**: All existing API endpoints function identically
- [ ] **Data Integrity**: Database operations maintain consistency and correctness
- [ ] **Error Handling**: Same error types and messages for client compatibility
- [ ] **Transaction Behavior**: Database transactions work correctly with function calls
- [ ] **Concurrent Safety**: No race conditions or data corruption under load

### Performance Requirements

- [ ] **Latency**: Measurable improvement in p95 response times (target: 5-15% reduction)
- [ ] **Memory**: Reduced allocation frequency and memory usage per request
- [ ] **Throughput**: Maintained or improved requests per second under load
- [ ] **CPU Utilization**: More efficient CPU usage patterns
- [ ] **Cache Performance**: Better instruction and data cache hit rates

### Code Quality Requirements

- [ ] **Type Safety**: Full compile-time verification of all changes
- [ ] **Readability**: Clear, self-documenting function signatures
- [ ] **Consistency**: Uniform patterns across all converted modules
- [ ] **Testability**: Maintained or improved unit test coverage
- [ ] **Documentation**: Comprehensive inline and external documentation

### Operational Requirements

- [ ] **Deployment**: Smooth deployment with zero downtime
- [ ] **Monitoring**: All existing metrics and alerting continue working
- [ ] **Debugging**: Maintained or improved error reporting and logging
- [ ] **Rollback**: Ability to quickly revert if issues arise

## Future Enhancements

### Request Context Integration

After completing the core migration, implement advanced memory management:

**Memory Arenas**:

```rust
pub struct RequestArena {
	bump: bumpalo::Bump,
	pools: ObjectPools,
}

pub fn with_request_context<T>(
	f: impl FnOnce(&RequestArena) -> Result<T>
) -> Result<T> {
	let arena = RequestArena::new();
	let result = f(&arena)?;
	// Arena automatically cleans up when dropped
	Ok(result)
}
```

**Handler Integration**:

```rust
pub async fn get_buildings_optimized(
	DatabaseConnection(mut conn): DatabaseConnection,
	player: Extension<AuthenticatedUser>,
) -> impl IntoResponse {
	with_request_context(|arena| {
		let buildings = player_buildings::get_game_buildings_in_arena(
			&mut conn,
			&player.id,
			arena
		)?;
		Ok(json!(buildings))
	}).await
}
```

### Advanced Optimizations

**Compile-Time Query Optimization**:

- Procedural macros for query validation
- Static query analysis and optimization hints
- Compile-time connection pool sizing

**Zero-Copy Serialization**:

- Direct serialization from database types
- Avoiding intermediate allocations for JSON responses
- Streaming responses for large result sets

**Database Connection Pooling**:

- Function-call-aware connection pooling strategies
- Connection affinity based on operation types
- Prepared statement caching optimization

### Monitoring and Observability

**Performance Metrics**:

- Function-level performance tracking
- Memory allocation patterns per endpoint
- Database connection utilization analytics

**Debugging Tools**:

- Enhanced tracing for function call chains
- Memory allocation profiling integration
- Query performance analysis tools

## Conclusion

This migration represents a fundamental architectural shift from object-oriented patterns to functional programming
principles that align with Rust's strengths. The benefits extend beyond performance improvements to include:

**Technical Benefits**:

- Elimination of unnecessary abstraction layers
- Better compiler optimization opportunities
- More predictable performance characteristics
- Simplified testing and debugging

**Architectural Benefits**:

- Clearer separation of concerns
- More explicit dependency management
- Better alignment with Rust idioms
- Foundation for advanced optimization techniques

**Long-term Strategic Benefits**:

- Reduced technical debt from inappropriate patterns
- Better performance scaling characteristics
- Easier onboarding for Rust-native developers
- Platform for implementing cutting-edge performance optimizations

The incremental migration approach ensures minimal risk while delivering measurable performance improvements. Each phase
builds upon the previous one, culminating in a more efficient, maintainable, and idiomatic Rust codebase that can serve
as the foundation for advanced performance optimization techniques like memory arenas and zero-copy operations.

This migration positions the codebase to take full advantage of Rust's performance capabilities while maintaining the
safety and reliability that makes Rust an excellent choice for systems programming.