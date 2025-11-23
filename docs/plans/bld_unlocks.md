# Proposal: Building Count Unlocks via Dedicated Table

## Problem

Building limits are currently static (`max_count` in `building` table). We need dynamic limits based on Keep level and
tech tree nodes, without overloading the modifier system (designed for temporal/rate-based calculations).

## Solution: Purpose-Built Unlocks Table

Create `building_count_unlocks` table that defines **additive** unlocks for specific buildings.

### Why This Design?

**Semantic clarity**: This is a capability unlock system, not a modifier system. Keeping them separate prevents
architectural confusion.

**Data-driven iteration**: Game designers can add/remove/balance unlocks via SQL without code deploys.

**Incremental adoption**: Works immediately for Keep levels. When tech tree arrives, same table handles tech unlocks—no
refactoring.

**Query simplicity**: Single JOIN to get all applicable unlocks. No JSONB parsing, no enum proliferation.

## How It Works

### Schema

```sql
building_count_unlocks
( building_id → which building gets more slots
    unlock_source_type → 'keep_level' | 'tech_node'
    source_identifier → '5' for Keep lvl 5, 'crop_rotation' for tech ID
    additional_count → +1, +2, etc.)
```

### Calculation Flow

1. **Base limit**: Read static `max_count` from `building` table (e.g., 4 farms)
2. **Keep unlocks**: Query unlocks where `unlock_source_type = 'keep_level'` AND
   `source_identifier <= player's Keep level`
3. **Tech unlocks**: Query unlocks where `unlock_source_type = 'tech_node'` AND
   `source_identifier IN player's researched techs`
4. **Sum**: `effective_limit = base + keep_bonuses + tech_bonuses`
5. **Check**: `current_count < effective_limit`

### Why Not Other Approaches?

**vs. Modifiers**: Modifiers handle continuous values (production rates, combat multipliers). Unlocks are discrete
gates—different semantics.

**vs. JSONB in tech_nodes**: Keeps unlock logic centralized and queryable. Avoids parsing blobs in construction
hot-path.

**vs. Hardcoded**: Data-driven = faster balance iteration. Code deploys are expensive.

## Trade-offs

**Accepts:**

- New table to maintain (but minimal—just unlock definitions)
- DB query on construction check (mitigated: cache per player session, or read-through cache)

**Gains:**

- Clear separation of concerns
- Works today (Keep), ready tomorrow (tech)
- Easy to answer: "What does Keep 5 unlock?" → `SELECT * WHERE source_identifier='5'`

## Migration Path

**Phase 1** (Now): Implement table + Keep level checks. Embassy/farms get dynamic limits based on Keep.

**Phase 2** (Later): Add tech tree schema. `player_tech` table tracks researched nodes. Same unlocks table handles
tech-based limits.

**Phase 3** (Future): Complex unlocks (e.g., "Tech A OR Tech B") → small helper function in `can_construct()`, still
reads from unlocks table.

## Files Touched

- **Migration**: Create `building_count_unlocks` table, seed Keep/tech unlock definitions
- **`src/db/`**: New module `building_unlocks.rs` for queries
- **`src/db/player_buildings.rs:254`**: Update `can_construct()` to calculate effective limit
- **`src/domain/`**: New `BuildingUnlock` struct
- **Schema**: Diesel regeneration

---

**Next step**: Confirm approach, then create detailed implementation plan with stages.
