# AGENTS.md - Empire

*Last updated 2025-06-15*

> **Purpose** – This file is the onboarding manual for every AI assistant (Claude, Cursor, GPT, etc.) and every human
> who edits this repository.  
> It encodes our coding standards, guard-rails, and workflow tricks so the *human 30%* (architecture, tests, domain
> judgment) stays in human hands.[^1]

---

## 1. Project Overview

Empire is the server backend for a base-building multi-client game built in Rust. It handles data, storage, scheduled
tasks,
and all logic in a server-authoritative manner. Key components:

- **domain**: Core data structures (DB, business, etc)
- **game**: Main game logic and routines
- **net**: Networking utilities and server scaffolding
- **controllers**: REST API routers & controllers

**Golden Rule**: When unsure about implementation details or requirements, ALWAYS consult the developer rather than
making assumptions.

---

## 2. Non-Negotiable Golden Rules

| #   | AI *may* do                                                                                                                                                                                         | AI *must NOT* do                                                                                                                                     |
|-----|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| G-1 | Whenever unsure about something that's related to the project, ask the developer for clarification before making changes.                                                                           | ❌ Write changes or use tools when you are not sure about something project specific, or if you don't have context for a particular feature/decision. |
| G-2 | Generate code **only inside** relevant source directories (e.g., `src/game/` for the main API, `migrations/` for the SQL blueprints, `crates/` for non-core libraries) or explicitly pointed files. | ❌ Touch `tests/`, `AGENTS.md`, or any `test` modules (humans own tests & specs).                                                                     |
| G-3 | Add/update **`AIDEV-NOTE:` anchor comments** near non-trivial edited code.                                                                                                                          | ❌ Delete or mangle existing `AIDEV-` comments.                                                                                                       |
| G-4 | Follow Rust formatting (`rustfmt.toml`) and use `cargo fmt` for code formatting.                                                                                                                    | ❌ Re-format code to any other style or manually format instead of using tooling.                                                                     |
| G-5 | For changes >300 LOC or >3 files, **ask for confirmation**.                                                                                                                                         | ❌ Refactor large modules without human guidance.                                                                                                     |
| G-6 | Stay within the current task context. Inform the dev if it'd be better to start afresh.                                                                                                             | ❌ Continue work from a prior prompt after "new task" – start a fresh session.                                                                        |

---

## 3. Rust Coding Standards

* **Rust Version**: 1.87+, use stable features unless explicitly requiring nightly
* **Formatting**: Use `cargo fmt` with project's `rustfmt.toml` configuration
* **Naming**: `snake_case` (functions/variables), `PascalCase` (structs, types, traits), `SCREAMING_SNAKE_CASE` (
  constants)
* **Error Handling**: Prefer `Result<T, E>` and `?` operator. Use `anyhow` for application errors, custom error types
  for domain errors
* **Documentation**: Use `///` for public APIs, `//` for implementation details
* **Testing**: Use `#[cfg(test)]` modules for unit tests, separate integration tests in `tests/` directory

**Error Handling Patterns**:

```rust
use anyhow::{Context, Result};
use crate::domain::error::GameError;

// For application-level functions
async fn process_game_data() -> Result<GameData> {
	let raw_data = fetch_data().await
		.context("Failed to fetch game data")?;

	parse_game_data(raw_data)
		.context("Failed to parse game data")
}

// For domain-specific errors
fn validate_building_placement(building: &Building) -> Result<(), GameError> {
	if !building.is_valid_position() {
		return Err(GameError::InvalidBuildingPlacement);
	}
	Ok(())
}
```

---

## 4. Project Layout & Core Components

| Directory     | Description                                        |
|---------------|----------------------------------------------------|
| `config/`     | DEV and PROD default configuration                 |
| `crates/`     | Large modules that became library crates           |
| `docs/`       | Markdown specs for the game                        |
| `migrations/` | SQL table definitions and data seeds               |
| `scripts/`    | Quick setup and maintenance scripts for local work |
| `src/`        | Main source code root                              |
| `tests/`      | Integration tests                                  |

**Key Domain Models**:

- **Player**: The users aka players of the game
- **Building**: Individual constructions in the player's realm
- **Resource**: A consumable supply, used for buildings and troops
- **Modifier**: A numeric unit that affects other systems
- **Job**: An async task to be executed at a given point in time

**Dependencies**:

- **axum**: Web framework for REST API
- **diesel**: ORM for PostgreSQL interactions
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **anyhow**: Error handling
- **uuid**: Unique identifiers

---

## 5. Anchor Comments

Add specially formatted comments throughout the codebase for inline knowledge that can be easily `grep`ped.

### Guidelines:

- Use `AIDEV-NOTE:`, `AIDEV-TODO:`, or `AIDEV-QUESTION:` (all-caps prefix) for comments aimed at AI and developers
- Keep them concise (≤ 120 chars)
- **Important:** Before scanning files, always first try to **locate existing anchors** `AIDEV-*` in relevant
  subdirectories
- **Update relevant anchors** when modifying associated code
- **Do not remove `AIDEV-NOTE`s** without explicit human instruction
- Add relevant anchor comments whenever code is:
    * Complex or non-obvious
    * Performance-critical
    * Part of a larger architectural pattern
    * Contains potential gotchas or edge cases

Example:

```rust
// AIDEV-NOTE: Performance hotpath - avoid allocations in game loop
async fn update_all_buildings(buildings: &mut [Building]) -> Result<()> {
	// ... implementation
}

// AIDEV-TODO: Consider using a more efficient data structure for large player counts
type PlayerMap = std::collections::HashMap<PlayerId, Player>;
```

---

## 6. Commit Discipline

* **Granular commits**: One logical change per commit
* **Tag AI-generated commits**: e.g., `feat: optimize resource calculation [AI]`
* **Clear commit messages**: Follow conventional commits format, explain the *why*
* **Use `git worktree`** for parallel/long-running AI branches
* **Review AI-generated code**: Never merge code you don't understand

Example commit messages:

```
feat: add building upgrade validation [AI]
fix: resolve race condition in resource updates [AI]
refactor: extract common game loop logic [AI]
```

---

## 7. Key File & Pattern References

**API Route Definitions**:

- Location: `src/controllers/` (e.g., `src/controllers/auth.rs`, `src/controllers/game.rs`)
- Pattern: Axum routers with handler functions, request/response structs, middleware

**Error Types**:

- Location: `src/domain/error.rs`
- Pattern: Custom error enums implementing `std::error::Error`, conversion with `From<>`

**Domain Models**:

- Location: `src/domain/` (e.g., `src/domain/player.rs`, `src/domain/building.rs`)
- Pattern: Structs with validation, serde derives, database mappings

**Database Schema**:

- Location: `migrations/` (Diesel migrations) and `src/schema.rs` (generated schema)
- Pattern: Diesel ORM with connection pooling, repository pattern for data access

**Game Logic**:

- Location: `src/game/` (core game systems and rules)
- Pattern: Service structs with business logic, event-driven updates

---

## 8. Common Rust Patterns in This Project

**Resource Management**:

```rust
// Use RAII and Drop trait for cleanup
struct GameSession {
	db_conn: DbConnection,
	// ... other resources
}

impl Drop for GameSession {
	fn drop(&mut self) {
		// Cleanup happens automatically
	}
}
```

**Async Patterns**:

```rust
// Use async/.await consistently
async fn process_player_action(action: PlayerAction) -> Result<GameState> {
	let validation = validate_action(&action).await?;
	let new_state = apply_action(validation).await?;
	save_state(&new_state).await?;
	Ok(new_state)
}
```

**Configuration**:

```rust
// Use config crate for environment-based configuration
#[derive(Debug, Deserialize)]
struct DatabaseConfig {
	url: String,
	max_connections: u32,
}
```

---

## 9. Directory-Specific AGENTS.md Files

* **Always check for `AGENTS.md` files in specific directories** before working on code within them
* If a directory's `AGENTS.md` is outdated or incorrect, **update it**
* If you make significant changes to a directory's structure or patterns, **document these in its `AGENTS.md`**
* If a directory lacks an `AGENTS.md` but contains complex logic worth documenting, **suggest creating one**

---

## 10. Common Pitfalls

* **Ownership Issues**: Forgetting to consider Rust's ownership model when designing APIs
* **Async Context**: Not properly handling async contexts and potential blocking operations
* **Error Propagation**: Using `unwrap()` or `expect()` instead of proper error handling
* **Database Connections**: Not properly managing connection pools or transactions
* **Performance**: Creating unnecessary allocations in hot code paths
* **Testing**: Writing tests that don't account for async behavior or database state

---

## 11. Domain-Specific Terminology

* **Player**: A user account that owns buildings and resources in the game
* **Building**: A structure that produces resources or provides game benefits
* **Resource**: Materials like wood, stone, gold that are consumed and produced
* **Realm**: A player's territory containing their buildings
* **Modifier**: Temporary or permanent effects that change game mechanics
* **Job**: Scheduled background tasks (building construction, resource production)

---

## 12. AI Assistant Workflow

When responding to user instructions, follow this process:

1. **Consult Guidance**: Review relevant `AGENTS.md` files for the request
2. **Clarify Ambiguities**: Ask targeted questions if requirements are unclear
3. **Plan & Break Down**: Create a rough plan referencing project conventions
4. **Execute or Confirm**: For trivial tasks, proceed immediately; for complex tasks, present plan for review
5. **Track Progress**: Use internal to-do lists for multi-step tasks
6. **Re-plan if Stuck**: Return to planning phase if blocked
7. **Update Documentation**: Update anchor comments and documentation after completion
8. **Request Review**: Ask user to review completed work
9. **Session Boundaries**: Suggest fresh session if context becomes unclear

---

## 13. Files to NOT Modify

These files control which files should be ignored by AI tools:

* `.agentignore`: Specifies files ignored by IDE (build directories, logs, caches, etc.)
* `.agentindexignore`: Controls IDE indexing exclusions for performance

**Never modify these ignore files** without explicit permission. When adding new files, check these patterns to ensure
proper indexing.

[^1]: This principle emphasizes human oversight for critical aspects like architecture, testing, and domain-specific
decisions, ensuring AI assists rather than fully dictates development.