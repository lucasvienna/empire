# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Essential Commands

### Development Commands
- `cargo build` - Build the project
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test
- `cargo fmt` - Format code using rustfmt
- `cargo clippy` - Run linter
- `cargo run` - Build and run the application

### Database Commands
- `./scripts/init_db.sh` - Initialize PostgreSQL database with Docker (runs migrations + seeds)
- `./scripts/test_db.sh` - Set up test database (runs migrations + seeds)
- `./scripts/pq_clean.sh` - Clean up test databases (removes UUID-named databases)
- `diesel setup` - Set up database schema (migrations only, no seeds)
- `diesel migration run` - Apply pending migrations
- `diesel database reset` - Reset database and reapply all migrations
- `cargo run --bin seed` - Run database seeds (populates reference data)

### Testing Commands
- `cargo test --bin empire` - Run binary tests
- `cargo test --lib` - Run library tests
- `cargo test tests::` - Run integration tests

## Architecture Overview

Empire is a Rust-based backend server for a base-building multiplayer game using a modular architecture:

### Core Modules
- **domain/**: Business logic and data models (Player, Building, Resource, Modifier, Job)
- **game/**: Game-specific logic including resource processing, building services, and modifier systems
- **controllers/**: REST API endpoints organized by feature (auth, game, health, player, user)
- **db/**: Database models and connection handling using Diesel ORM
- **net/**: Networking layer with Axum web framework
- **auth/**: Authentication and session management
- **job_queue/**: Async task processing system

### Key Technologies
- **Axum**: Web framework for REST API
- **Diesel**: PostgreSQL ORM with connection pooling
- **Tokio**: Async runtime
- **Tracing**: Structured logging and telemetry
- **UUID/ULID**: Unique identifiers
- **Serde**: JSON serialization

### Database
- PostgreSQL with migrations in `migrations/` directory
- Schema auto-generated in `src/schema.rs`
- Uses Diesel ORM with connection pooling
- Docker setup for development environment
- **Seeding Strategy**: Hybrid approach separating schema from data
  - **Migrations** (`migrations/`): Schema structure + critical reference data (factions, core building definitions)
  - **Seeds** (`seeds/`): Extended configuration data (building levels, resources, future items/tech trees)
  - Seeds are SQL files executed in alphabetical order by `empire::db::seeds::run()`
  - All seeds are idempotent using unique constraints + `ON CONFLICT DO NOTHING`
  - Seeds run automatically in tests via `tests/common/mod.rs`

## Project Conventions

### Error Handling
- Use `anyhow::Result` for application-level functions
- Custom error types in `src/domain/error.rs` for domain-specific errors
- Proper error context with `.context()` method

### Code Style
- Follow `rustfmt.toml` configuration (module-level import grouping)
- Use `#[instrument]` macro for tracing in public methods
- Skip sensitive parameters: `#[instrument(skip(password))]`
- Consistent logging levels per `docs/tracing_guidelines.md`

### Domain Models
- **Player**: Game users with resources and buildings
- **Building**: Structures that produce resources or provide benefits
- **Resource**: Consumable materials (wood, stone, gold)
- **Modifier**: Temporary/permanent effects on game mechanics
- **Job**: Scheduled background tasks

## Important Files
- `AGENTS.md` - Comprehensive development guidelines (read this first)
- `docs/tracing_guidelines.md` - Logging and telemetry standards
- `src/lib.rs` - Main module exports
- `src/main.rs` - Application entry point
- `config/` - Environment-specific configuration files

## Development Workflow
1. Use `./scripts/init_db.sh` to set up database (runs migrations + seeds automatically)
2. Run `cargo test` to verify setup
3. Use `cargo fmt` and `cargo clippy` before committing
4. Follow tracing guidelines for logging
5. Add migrations for schema changes using Diesel CLI
6. Add reference data to `seeds/` directory (see `seeds/README.md` for conventions)
   - Use numeric prefixes to control execution order (e.g., `003_items.sql`)
   - Add unique constraints to tables in migrations
   - Make seeds idempotent with `ON CONFLICT DO NOTHING`
   - Commit seed files to version control for game balance tracking
   - Seeds automatically run in tests - no additional setup needed

## Testing
- Unit tests in `#[cfg(test)]` modules within source files
- Integration tests in `tests/` directory
- Database tests use separate test database setup
- Use `axum-test` for API endpoint testing