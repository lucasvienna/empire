# Empire Project Guidelines

## Project Overview

Empire is a multiplayer strategy game built in Rust using modern web technologies. The game features:

- **5 Factions**: Humans, Orcs, Elves, Dwarves, and Goblins, each with unique bonuses
- **Resource Management**: 4 main resources (Food, Wood, Stone, Gold) with production buildings
- **Building System**: Multiple categories including Core (Keep), Resources, Research, Military, Religious, Social, and Decorative buildings
- **Combat System**: Various unit types (Infantry, Ranged, Cavalry, Siege, Artillery) with faction-specific bonuses
- **Research Trees**: Academy (military), University (economic/political), Laboratory (magic)
- **Social Features**: Guilds, trading market, alliances, and diplomacy
- **Religion System**: Deity worship with different buffs and miracle abilities

## Technical Architecture

### Core Technologies
- **Language**: Rust (Edition 2021)
- **Web Framework**: Axum with async/await support
- **Database**: PostgreSQL with Diesel ORM
- **Authentication**: JWT tokens with Argon2 password hashing
- **Logging**: Tracing ecosystem with structured logging
- **Testing**: Comprehensive test suite with axum-test, claims, fake, and quickcheck

### Project Structure
```
empire/
├── src/
│   ├── auth/              # Authentication and session management
│   ├── controllers/       # HTTP request handlers
│   │   └── game/         # Game-specific controllers
│   ├── db/               # Database models and queries
│   ├── domain/           # Business logic and domain models
│   │   ├── building/     # Building-related logic
│   │   ├── modifier/     # Game modifier system
│   │   └── player/       # Player-related logic
│   ├── game/             # Core game mechanics
│   │   ├── modifiers/    # Modifier implementations
│   │   └── resources/    # Resource management
│   ├── job_queue/        # Background job processing
│   └── net/              # Network utilities
├── crates/
│   └── rpc/              # RPC communication layer
├── docs/                 # Project documentation
├── migrations/           # Database migrations
├── tests/                # Integration and API tests
└── scripts/              # Utility scripts
```

## Development Guidelines

### Code Style and Quality
- Follow Rust standard formatting (use `cargo +nightly fmt`)
- Use `cargo clippy` for linting and code quality checks
- Implement comprehensive error handling with `anyhow` crate
- Use `derive_more` for reducing boilerplate code
- Follow the established tracing guidelines (see `docs/tracing_guidelines.md`)

### Tracing and Logging
- Use `#[instrument]` macro for all public methods
- Skip sensitive parameters: `#[instrument(skip(password, token))]`
- Follow standardized log levels:
  - `error`: Critical issues requiring immediate attention
  - `warn`: Non-critical issues that might need attention
  - `info`: Important operational events
  - `debug`: Detailed operational information
  - `trace`: Very detailed debugging information
- Include context in all log messages (user IDs, session IDs, etc.)
- Never log sensitive information (passwords, tokens, etc.)

### Database Management
- Use Diesel migrations for all schema changes
- Follow the established naming conventions for tables and columns
- Use ULIDs for primary keys where appropriate
- Implement proper foreign key relationships
- Add database triggers for default data insertion

## Testing Guidelines

### Test Structure
- **Unit Tests**: Located alongside source code in `src/` modules
- **Integration Tests**: Located in `tests/` directory
- **API Tests**: Use `axum-test` for HTTP endpoint testing
- **Database Tests**: Use test database with proper cleanup

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test module
cargo test tests::api

# Run tests with output
cargo test -- --nocapture

# Run tests in single thread (for database tests)
cargo test -- --test-threads=1
```

### Test Requirements
- All new features must include comprehensive tests
- API endpoints must have integration tests
- Database operations must have transaction tests
- Use `fake` crate for generating test data
- Use `claims` crate for better assertions
- Property-based testing with `quickcheck` for complex logic

## Build and Development

### Prerequisites
- Rust 1.88+ (Edition 2021)
- PostgreSQL database
- Docker (optional, for containerized deployment)

### Development Setup
```bash
# Install dependencies
cargo build

# Set up database
diesel setup
diesel migration run

# Run development server
cargo run

# Run with specific configuration
cargo run -- --config config/development.toml
```

### Build Commands
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Check code without building
cargo check

# Format code
cargo +nightly fmt

# Lint code
cargo clippy

# Run security audit
cargo audit
```

### Environment Configuration
- Use `.env` files for local development
- Configuration files in `config/` directory
- Support for multiple environments (development, staging, production)
- Use `secrecy` crate for sensitive configuration values

## Deployment

### Docker Support
- Dockerfile provided for containerized deployment
- Multi-stage build for optimized production images
- Health checks and proper signal handling

### Database Migrations
- Always run migrations before deploying new versions
- Test migrations on staging environment first
- Keep migrations backward compatible when possible

## Code Review Guidelines

### Before Submitting
- Ensure all tests pass: `cargo test`
- Format code: `cargo +nightly fmt`
- Fix all clippy warnings: `cargo clippy`
- Update documentation if needed
- Add appropriate tracing/logging
- Test database migrations if applicable

### Review Checklist
- Code follows established patterns and conventions
- Proper error handling and logging
- Security considerations (no sensitive data exposure)
- Performance implications considered
- Database queries are efficient
- Tests cover new functionality
- Documentation is updated

## Security Considerations

- Use Argon2 for password hashing
- Implement proper JWT token validation
- Sanitize all user inputs
- Use parameterized queries to prevent SQL injection
- Follow principle of least privilege for database access
- Regular security audits with `cargo audit`

## Performance Guidelines

- Use async/await for I/O operations
- Implement proper database connection pooling with r2d2
- Use appropriate indexing for database queries
- Monitor and log performance metrics
- Use `tokio` runtime efficiently
- Consider caching for frequently accessed data

## Contributing

When contributing to the Empire project:
1. Follow all guidelines outlined in this document
2. Refer to existing documentation in `docs/` folder
3. Maintain consistency with established patterns
4. Write comprehensive tests for new features
5. Update documentation as needed
6. Follow the established Git workflow and commit message conventions
