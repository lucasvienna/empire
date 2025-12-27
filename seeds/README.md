# Database Seeds

This directory contains SQL files for seeding reference/configuration data into the database.

## Purpose

Seeds contain **extended configuration data** that is:

- Required for the game to function
- Identical across all environments (dev, test, prod)
- Subject to frequent modification during balancing
- Version-controlled for game updates and patches

## What Belongs Here vs Migrations

### Migrations (`migrations/` directory)

- Database schema (tables, columns, indexes, constraints)
- Critical reference data that's tightly coupled to schema
- Examples: faction enum values, core building definitions

### Seeds (`seeds/` directory)

- Bulk configuration data
- Game balance parameters
- Examples: building levels, building resources, items, tech trees, unit definitions

## File Naming Convention

Seed files are executed in **alphabetical order** by filename. Use numeric prefixes to control
execution order:

```
001_building_levels.sql
002_building_resources.sql
003_items.sql
004_tech_tree.sql
```

## Running Seeds

### Initial Setup

```bash
./scripts/init_db.sh  # Runs migrations + seeds automatically
```

### Manual Execution

```bash
cargo run --bin seed  # Run all seed files
```

### Test Database

```bash
./scripts/test_db.sh  # Creates test DB with migrations + seeds
```

## Idempotency

Seed files are **idempotent** and can be run multiple times safely. This is achieved through:

1. **Unique constraints** on reference data tables (defined in migrations)
2. **ON CONFLICT DO NOTHING** clauses in all INSERT statements

Example:

```sql
INSERT INTO building_level (building_id, level, upgrade_seconds, req_food)
VALUES (1, 0, 0, 0)
ON CONFLICT (building_id, level) DO NOTHING;
```

This ensures that:

- Re-running seeds won't create duplicate data
- Seed execution is safe in all environments
- Tests can run seeds without worrying about conflicts

## Adding New Seeds

1. Create a new SQL file with the next numeric prefix: `00X_entity_name.sql`
2. Write INSERT statements for your reference data
3. **Important**: Add `ON CONFLICT (...) DO NOTHING` to make seeds idempotent
4. Ensure the table has appropriate unique constraints in the migration
5. Test locally: `cargo run --bin seed`
6. Verify idempotency: run the seed command multiple times
7. Commit the seed file to version control
8. Seeds will run automatically for other developers via `init_db.sh` and in tests

## Version Control

Seed files are **committed to git** and represent the canonical game configuration at each version.
Changes to seed files should:

- Be reviewed like code changes
- Include clear commit messages explaining balance changes
- Be deployed alongside code that depends on them

## Future Considerations

As seed data grows:

- Consider splitting large files by entity or faction
- Add seed file validation/linting
- Create separate dev-only sample data mechanism
- Add seed versioning/tracking table to prevent duplicate runs
