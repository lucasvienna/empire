# Empire Naming Conventions

## Core Principles

1. **Use abbreviations** for instances/variables, full names for types/structs
2. **Type aliases** for domain concepts (even if underlying type is same)
3. **Consistent prefixes** for similar concepts across domains
4. **Clear state transitions**: base → multiplier → effective

## Common Abbreviations

| Full Term     | Abbreviation | Usage                    |
| ------------- | ------------ | ------------------------ |
| calculate     | calc         | `calc_multiplier()`      |
| computation   | comp         | `comp_effective_stats()` |
| modifier      | mdf          | `mdf_service`            |
| multiplier    | mult         | `get_mult()`             |
| production    | prod         | `prod_rates`             |
| resource      | res          | `res_type`               |
| effective     | eff          | `eff_stats`              |
| operation     | op           | `mdf_ops`                |
| accumulator   | acc          | `acc_cap`                |
| capacity      | cap          | `storage_cap`            |
| generation    | gen          | `gen_rates`              |
| collection    | coll         | `coll_resources()`       |
| speed         | spd          | `research_spd`           |
| statistics    | stats        | `combat_stats`           |
| configuration | cfg          | `mdf_cfg`                |
| service       | srv          | `mdf_srv`                |

## Type Aliases Structure

### Pattern: `[Domain][Measurement][State]`

```rust
// Resources Domain
type ResourceBaseRates = HashMap<ResourceType, BigDecimal>;      // Unmodified generation rates
type ResourceMultipliers = HashMap<ResourceType, BigDecimal>;    // Modifier multipliers
type ResourceProductionRates = HashMap<ResourceType, BigDecimal>;// Final production rates

// Combat Domain (future)
type CombatBaseStats = HashMap<CombatStat, BigDecimal>;          // Unmodified combat values
type CombatMultipliers = HashMap<CombatStat, BigDecimal>;        // Combat modifiers
type CombatEffectiveStats = HashMap<CombatStat, BigDecimal>;     // Final combat stats

// Research Domain (future)
type ResearchBaseSpeed = BigDecimal;                             // Unmodified research speed
type ResearchMultiplier = BigDecimal;                            // Research speed modifier
type ResearchEffectiveSpeed = BigDecimal;                        // Final research speed
```

## State Progression Pattern

All domains follow this progression:

```
base → multiplier → effective/production
```

Examples:

- Resources: `base_rates` → `multipliers` → `prod_rates`
- Combat: `base_stats` → `multipliers` → `eff_stats`
- Research: `base_spd` → `multiplier` → `eff_spd`

## Function Naming Patterns

### Database Operations

```rust
get_*       // Simple fetch: get_base_rates()
query_*     // Complex query: query_active_mdfs()
load_*      // Bulk fetch: load_player_mdfs()
```

### Calculations

```rust
calc_*      // Calculate value: calc_multiplier()
comp_*      // Complex computation: comp_stacked_mult()
apply_*     // Apply to get result: apply_rate_mods()
combine_*   // Combine multiple: combine_multipliers()
```

### State Management

```rust
update_*    // Mutate existing: update_cache()
create_*    // Create new: create_modifier()
remove_*    // Delete: remove_expired_mods()
schedule_*  // Queue for later: schedule_expiration()
```

## Module & Service Naming

```rust
// Operations modules (pure functions, no state)
mdf_ops     // modifier_operations
res_ops     // resource_operations
combat_ops  // combat_operations

// Services (stateful, may cache)
mdf_srv     // ModifierService
res_srv     // ResourceService
combat_srv  // CombatService

// Processors (background jobs)
res_processor    // ResourceProcessor
mdf_processor    // ModifierProcessor
combat_processor // CombatProcessor
```

## Specific Renamings from Current Code

### Types

- `FullModifier` → `AppliedModifier`
- `ResourceModifiers` → `ResourceMultipliers`
- Keep `ResourceGeneration` as is (struct name)

### Functions

- `calc_multiplier()` → `calc_target_mult()`
- `apply_stacking_rules()` → `calc_stacked_mult()`
- `apply_rate_modifiers()` → `calc_prod_rates()`
- `combine_rates_with_modifiers()` → `apply_rate_mods()`
- `get_production_rates()` → `get_base_rates()`
- `get_modifier_multipliers()` → `get_res_mults()`

### Method Consistency

- Async + cached: `get_or_calc_*()`
- Sync, no cache: `calc_*()`
- Database fetch: `get_*()` or `query_*()`

## Domain-Specific Patterns

### Resources

```rust
// Types (use full names)
ResourceBaseRates         // From DB, unmodified
ResourceMultipliers       // Calculated modifiers
ResourceProductionRates   // Final rates for production

// Functions (use abbreviations)
get_res_base_rates()      // Fetch from DB
calc_res_mults()          // Calculate multipliers
calc_res_prod_rates()     // Apply mults to base
produce_resources()       // Execute production
collect_resources()       // Transfer to storage

// Variables (use abbreviations)
let res_mults: ResourceMultipliers = ...;
let base_rates: ResourceBaseRates = ...;
let prod_rates: ResourceProductionRates = ...;
```

### Combat (Future)

```rust
// Types (use full names)
CombatBaseStats       // From equipment/skills
CombatMultipliers     // From buffs/debuffs
CombatEffectiveStats  // Final for calculation

// Functions (use abbreviations)
get_combat_base_stats()
calc_combat_mults()
calc_combat_eff_stats()
execute_combat_round()
apply_combat_damage()

// Variables (use abbreviations)
let combat_mults: CombatMultipliers = ...;
let base_stats: CombatBaseStats = ...;
let eff_stats: CombatEffectiveStats = ...;
```

### Research (Future)

```rust
// Types (use full names)
ResearchBaseSpeed       // From buildings/tech
ResearchMultiplier      // From modifiers
ResearchEffectiveSpeed  // Final speed

// Functions (use abbreviations)
get_research_base_spd()
calc_research_mult()
calc_research_eff_spd()
complete_research()
queue_next_research()

// Variables (use abbreviations)
let research_mult: ResearchMultiplier = ...;
let base_spd: ResearchBaseSpeed = ...;
let eff_spd: ResearchEffectiveSpeed = ...;
```

## Cache Keys Pattern

```rust
// Format: [domain]:[player_id]:[target]:[optional_subtype]
"res:12345:mult:food"      // Resource multiplier for food
"combat:12345:mult:attack"  // Combat multiplier for attack
"research:12345:mult"        // Research speed multiplier
```

## Error Messages

Use full words in error messages for clarity:

```rust
// Bad
"Failed to calc res mults"

// Good
"Failed to calculate resource multipliers for player {}"
```

## Migration Priority

1. **High Priority** (confusing current names):
   - `FullModifier` → `AppliedModifier`
   - `ResourceModifiers` → `ResourceMultipliers`
   - Function names for clarity

2. **Medium Priority** (working but inconsistent):
   - Srv method names
   - Cache key formats

3. **Low Priority** (nice to have):
   - Internal variable names
   - Test function names

## Examples

### Before

```rust
let modifiers = modifier_service.get_modifier_multipliers(player_id).await?;
let base_rates = resource_service.get_production_rates(player_id)?;
let production_rates = resource_operations::combine_rates_with_modifiers(&base_rates, &modifiers);
```

### After

```rust
// Types use full names, instances use abbreviations
let res_mults: ResourceMultipliers = mdf_srv.get_res_mults(player_id).await?;
let base_rates: ResourceBaseRates = res_srv.get_base_rates(player_id)?;
let prod_rates: ResourceProductionRates = res_ops::apply_rate_mods(&base_rates, &res_mults);
```

This creates clearer, more consistent code while keeping names concise.
