# Combat System Design

**Version**: 0.1.0 **Created**: 2025-12-26 **Status**: Draft

---

## Overview

Empire's combat system is a deterministic battle resolution system where players attack other
players or NPCs to plunder resources. Combat outcomes are calculated based on unit compositions,
unit type advantages, and faction bonuses.

> **v0.1.0 Scope**: This version focuses on **PvE combat only** (attacking NPCs). PvP combat,
> marches, garrisons, and walls are deferred to v0.2.0 to allow the core combat math to mature.

### Design Principles

1. **Deterministic**: No randomness - identical inputs always produce identical outputs
2. **Strategic**: Unit type advantages reward thoughtful army composition
3. **Balanced**: No single faction or unit type dominates
4. **Transparent**: Players can predict outcomes before attacking
5. **Simple**: Easy to understand, difficult to master

### Core Loop

```
Attack → Calculate Power → Determine Winner → Apply Losses → Transfer Plunder
```

---

## Unit System

### Unit Types

Four unit types are available in v0.1.0, with a fifth (Magical) coming in v0.2.0+:

| Unit Type   | Building   | Role               | Base ATK | Base DEF | Training Time | Version |
| ----------- | ---------- | ------------------ | -------- | -------- | ------------- | ------- |
| Infantry    | Barracks   | Frontline fighters | 10       | 15       | 60s           | v0.1.0  |
| Ranged      | Range      | Backline damage    | 15       | 5        | 90s           | v0.1.0  |
| Cavalry     | Stable     | Fast flankers      | 12       | 10       | 120s          | v0.1.0  |
| Artillery   | Workshop   | Siege weapons      | 20       | 3        | 180s          | v0.1.0  |
| **Magical** | Laboratory | Glass cannon       | 25       | 2        | 300s          | v0.2.0+ |

**Design Notes:**

- Infantry: Balanced, good for holding ground
- Ranged: Glass cannon, high damage but fragile
- Cavalry: Mobile, moderate stats, counters artillery
- Artillery: Extreme damage, extremely fragile
- Magical: Ultimate glass cannon - devastates if protected, melts if focused (requires tech
  research)

### Unit Costs

Each unit has resource costs that scale with their power:

| Unit Type   | Food | Wood | Stone | Gold | Total Value | Version |
| ----------- | ---- | ---- | ----- | ---- | ----------- | ------- |
| Infantry    | 20   | 10   | -     | -    | 30          | v0.1.0  |
| Ranged      | 15   | 20   | -     | -    | 35          | v0.1.0  |
| Cavalry     | 30   | -    | -     | 15   | 45          | v0.1.0  |
| Artillery   | 25   | 15   | 20    | -    | 60          | v0.1.0  |
| **Magical** | 40   | -    | 20    | 40   | 100         | v0.2.0+ |

---

## Unit Type Advantages

Combat uses a rock-paper-scissors advantage system where each unit type is strong against one type
and weak against another.

### Advantage Matrix

```
Infantry  →  Ranged   (strong)
Ranged    →  Cavalry  (strong)
Cavalry   →  Artillery (strong)
Artillery →  Infantry  (strong)
```

Visual:

```
    Infantry
     ↗    ↘
Artillery  Ranged
     ↖    ↙
     Cavalry
```

### Advantage Multipliers

| Matchup Type | Multiplier | Description                       |
| ------------ | ---------- | --------------------------------- |
| Strong       | 1.50x      | Attacker's unit type beats target |
| Neutral      | 1.00x      | No advantage                      |
| Weak         | 0.67x      | Target's unit type beats attacker |

### Full Matchup Table

| Attacker ↓ / Defender → | Infantry | Ranged | Cavalry | Artillery |
| ----------------------- | -------- | ------ | ------- | --------- |
| **Infantry**            | 1.00     | 1.50   | 1.00    | 0.67      |
| **Ranged**              | 0.67     | 1.00   | 1.50    | 1.00      |
| **Cavalry**             | 1.00     | 0.67   | 1.00    | 1.50      |
| **Artillery**           | 1.50     | 1.00   | 0.67    | 1.00      |

**Example**: 100 Infantry attacking 100 Ranged

- Infantry has 1.5x advantage against Ranged
- Infantry attacks at 150% effectiveness
- Ranged defends at 67% effectiveness (inverse)

### Magical Unit Combat (v0.2.0+)

Magical units operate **outside the rock-paper-scissors cycle** as glass cannons:

**Offensive Multipliers (Magical attacking):**

| Target Type | Multiplier | Notes                  |
| ----------- | ---------- | ---------------------- |
| Infantry    | 1.25x      | Slight advantage       |
| Ranged      | 1.25x      | Slight advantage       |
| Cavalry     | 1.25x      | Slight advantage       |
| Artillery   | 1.25x      | Slight advantage       |
| Magical     | 1.00x      | Mage duels are neutral |

**Defensive Multipliers (Magical defending):**

| Attacker Type | Multiplier | Notes                  |
| ------------- | ---------- | ---------------------- |
| Infantry      | 0.75x      | Takes extra damage     |
| Ranged        | 0.75x      | Takes extra damage     |
| Cavalry       | 0.75x      | Takes extra damage     |
| Artillery     | 0.75x      | Takes extra damage     |
| Magical       | 1.00x      | Mage duels are neutral |

**Strategic Implications:**

- Magical units deal 25% bonus damage to all physical types
- Magical units take 33% extra damage from all physical types (1/0.75 = 1.33)
- Devastatingly effective when protected by frontline units
- Extremely vulnerable if enemy reaches them
- High cost (100 resources) and slow training (300s) limits spam
- No faction has Magical bonus - all factions are equal with Mages

**Optimal Use:**

- Bring Infantry/Cavalry frontline to absorb hits
- Position Mages as "backline DPS"
- Counter enemy Mages with your own (neutral matchup) or overwhelm with numbers
- Goblins can swarm Mages cost-effectively due to training speed bonus

---

## Faction Bonuses

Each faction has combat bonuses that apply to specific unit types. These are implemented as
**permanent modifiers** applied via the modifier system when a player selects their faction.

| Faction | Combat Bonus           | Effect                              |
| ------- | ---------------------- | ----------------------------------- |
| Humans  | +15% Cavalry ATK/DEF   | Cavalry units deal/take 15% more    |
| Orcs    | +15% Infantry ATK/DEF  | Infantry units deal/take 15% more   |
| Elves   | +15% Ranged ATK/DEF    | Ranged units deal/take 15% more     |
| Dwarves | +15% Artillery ATK/DEF | Artillery units deal/take 15% more  |
| Goblins | No combat bonus        | +20% training speed (swarm tactics) |

**Implementation via Modifier System:**

Faction bonuses are stored as modifiers in the database and applied via triggers on player
creation/faction change:

```sql
-- Example faction modifier records (seeded data)
INSERT INTO modifiers (name, target_type, target_unit_type, target_combat_stat, magnitude, stacking_behaviour) VALUES
  ('Orc Infantry ATK Bonus', 'combat', 'infantry', 'atk', 0.15, 'additive'),
  ('Orc Infantry DEF Bonus', 'combat', 'infantry', 'def', 0.15, 'additive'),
  ('Human Cavalry ATK Bonus', 'combat', 'cavalry', 'atk', 0.15, 'additive'),
  ('Human Cavalry DEF Bonus', 'combat', 'cavalry', 'def', 0.15, 'additive'),
  ('Elf Ranged ATK Bonus', 'combat', 'ranged', 'atk', 0.15, 'additive'),
  ('Elf Ranged DEF Bonus', 'combat', 'ranged', 'def', 0.15, 'additive'),
  ('Dwarf Artillery ATK Bonus', 'combat', 'artillery', 'atk', 0.15, 'additive'),
  ('Dwarf Artillery DEF Bonus', 'combat', 'artillery', 'def', 0.15, 'additive');
  -- Goblins have no combat modifiers, only training speed
```

When combat calculations run, the modifier system aggregates all applicable modifiers:

- Faction passives (permanent, no expiry)
- Research bonuses (permanent after unlock)
- Temporary buffs (items, events)
- Building bonuses (Walls DEF bonus when defending)

**Goblin Strategy:**

Goblins compensate for lack of combat bonuses through superior production speed:

- +20% training speed means ~17% more units over time
- Swarm tactics: overwhelm with numbers instead of quality
- Better recovery after losses (faster replenishment)
- Ideal for aggressive, high-volume attack strategies

---

## Combat Calculation

### Overview

Combat is resolved in a single calculation phase. Both sides contribute power based on their army
composition, and the ratio of attacker power to defender power determines the outcome.

### Step 1: Calculate Raw Army Power

Power calculation separates raw strength from tactical advantage. First, calculate each army's raw
power without type matchups:

**Mathematical Definition:**

Let:

- `A` = Attacker's army: set of unit stacks `{(type_i, qty_i, atk_i)}`
- `D` = Defender's army: set of unit stacks `{(type_j, qty_j, def_j)}`
- `N_A` = Total attacker units: `Σ qty_i`
- `N_D` = Total defender units: `Σ qty_j`
- `mod(player, stat, unit_type)` = Total modifier multiplier from modifier system (see below)
- `adv(type_a, type_d)` = Advantage multiplier from matchup table

**Attacker Power Formula:**

```
P_atk = Σᵢ [ qty_i × atk_i × mod(attacker, ATK, type_i) ]
```

**Defender Power Formula:**

```
P_def = Σⱼ [ qty_j × def_j × mod(defender, DEF, type_j) ]
```

**Raw Ratio:**

```
raw_ratio = P_atk / P_def
```

### Step 2: Calculate Advantage Factor

Type advantages are applied as a single weighted multiplier after raw power calculation. This
prevents the "squared advantage" problem where both sides applying multipliers creates ~2.25x
effective advantage instead of the intended 1.5x.

**Advantage Factor Formula:**

```
advantage_factor = Σᵢ Σⱼ [ (qty_i / N_A) × (qty_j / N_D) × adv(type_i, type_j) ]
```

**Intuition**: The advantage factor is a weighted average of all attacker-vs-defender matchups. Each
matchup contributes proportionally to how much of each army is involved.

**Properties:**

| Matchup Type                                             | Advantage Factor              |
| -------------------------------------------------------- | ----------------------------- |
| Pure counter-pick (e.g., 100% Infantry vs 100% Ranged)   | 1.50x                         |
| Pure counter-picked (e.g., 100% Ranged vs 100% Infantry) | 0.67x                         |
| Mixed vs Mixed                                           | ~1.0x (advantages cancel out) |
| Partial counter                                          | Proportional blend            |

**Example** (60% Infantry + 40% Cavalry vs 62.5% Ranged + 37.5% Artillery):

| Attacker | Weight | Defender  | Weight | adv() | Contribution |
| -------- | ------ | --------- | ------ | ----- | ------------ |
| Infantry | 0.60   | Ranged    | 0.625  | 1.50  | 0.5625       |
| Infantry | 0.60   | Artillery | 0.375  | 0.67  | 0.1508       |
| Cavalry  | 0.40   | Ranged    | 0.625  | 0.67  | 0.1675       |
| Cavalry  | 0.40   | Artillery | 0.375  | 1.50  | 0.2250       |

**advantage_factor** = 0.5625 + 0.1508 + 0.1675 + 0.2250 = **1.106** (slight attacker advantage)

### Modifier System Integration

The `mod(player, stat, unit_type)` function queries the existing modifier system for combat bonuses:

```rust
// Pseudocode - uses existing modifier_operations::calc_multiplier
fn get_combat_modifier(
    conn: &mut DbConn,
    player_id: &PlayerKey,
    stat: CombatStat,      // ATK or DEF
    unit_type: UnitType,   // Infantry, Ranged, Cavalry, Artillery, Magical
) -> BigDecimal {
    modifier_operations::calc_multiplier(
        conn,
        player_id,
        ModifierTarget::Combat,
        Some(CombatTargetType { stat, unit_type }),  // Needs schema extension
    )
}
```

**Required Schema Extension:**

Add separate columns for unit type and combat stat targeting:

```sql
-- New enum for combat stats
CREATE TYPE combat_stat AS ENUM ('atk', 'def');

-- Add columns to modifiers table
ALTER TABLE modifiers ADD COLUMN target_unit_type unit_type NULL;
ALTER TABLE modifiers ADD COLUMN target_combat_stat combat_stat NULL;

-- Constraint: combat modifiers must have stat, unit_type is optional (NULL = all units)
ALTER TABLE modifiers ADD CONSTRAINT chk_combat_target
  CHECK (
    (target_type != 'combat') OR
    (target_combat_stat IS NOT NULL)
    -- target_unit_type can be NULL for global modifiers like "Military Tactics I: +5% all ATK"
  );

-- Index for efficient combat modifier lookups
CREATE INDEX idx_modifiers_combat_target
  ON modifiers (target_type, target_unit_type, target_combat_stat)
  WHERE target_type = 'combat';
```

**Rust Domain Model:**

```rust
// In src/domain/modifier/mod.rs

#[derive(/* derives */)]
#[diesel(sql_type = crate::schema::sql_types::CombatStat)]
pub enum CombatStat {
    Atk,
    Def,
}

// Updated Modifier struct
pub struct Modifier {
    // ... existing fields ...
    pub target_resource: Option<ResourceType>,     // For Resource modifiers
    pub target_unit_type: Option<UnitType>,        // For Combat/Training modifiers
    pub target_combat_stat: Option<CombatStat>,    // For Combat modifiers (ATK/DEF)
}
```

**Modifier Sources for Combat:**

| Source          | target_type | target_unit_type | target_combat_stat | Example                         |
| --------------- | ----------- | ---------------- | ------------------ | ------------------------------- |
| Faction passive | `combat`    | `infantry`       | `atk`              | Orcs: +15% Infantry ATK         |
| Faction passive | `combat`    | `infantry`       | `def`              | Orcs: +15% Infantry DEF         |
| Research node   | `combat`    | `NULL` (all)     | `atk`              | Military Tactics I: +5% all ATK |
| Temporary buff  | `combat`    | `cavalry`        | `atk`              | War Horn: +20% Cavalry ATK      |
| Building bonus  | `combat`    | `NULL` (all)     | `def`              | Walls Lv5: +10% all DEF         |

**Note**: When `target_unit_type` is `NULL`, the modifier applies to all unit types. The query
should use `(target_unit_type IS NULL OR target_unit_type = :unit_type)` to match both specific and
global modifiers.

**Stacking Example:**

Player is Orc with "Military Tactics I" research and active "War Horn" buff, attacking with
Infantry:

```
Faction modifier (Additive):     +15% Infantry ATK
Research modifier (Additive):    +5% all ATK
War Horn modifier (Multiplicative): +20% (but only affects Cavalry, not Infantry)

Total for Infantry ATK:
  Additive: 1.0 + 0.15 + 0.05 = 1.20
  Multiplicative: 1.0 (no applicable multipliers)
  Final: 1.20 × 1.0 = 1.20x multiplier
```

### Step 3: Calculate Final Combat Ratio

```
combat_ratio = raw_ratio × advantage_factor
```

- Ratio > 1.0: Attacker has advantage
- Ratio = 1.0: Evenly matched
- Ratio < 1.0: Defender has advantage

### Step 4: Determine Winner

```
Winner = combat_ratio >= 1.0 ? Attacker : Defender
```

**Note**: Ties (ratio = 1.0) favor the attacker. Defenders need >1.0 equivalent power to win.

### Step 5: Calculate Losses (Asymmetric)

Losses are calculated asymmetrically based on role (attacker vs defender) to reward defenders and
punish failed attacks. The `effective_ratio` is always >= 1.0 (inverted if defender wins).

```
effective_ratio = combat_ratio >= 1.0 ? combat_ratio : 1.0 / combat_ratio
```

**Winner Losses (2-15%):**

```
winner_loss_pct = clamp(20% / effective_ratio, 2%, 15%)
```

| Effective Ratio  | Winner Losses |
| ---------------- | ------------- |
| 1.0 (barely won) | 15%           |
| 1.5              | 13%           |
| 2.0              | 10%           |
| 5.0              | 4%            |
| 10+              | 2%            |

**Losing Attacker (40-90%):**

Failed attacks are punished harshly - you marched into enemy territory and lost.

```
losing_attacker_pct = clamp(40% + 25% × (effective_ratio - 1), 40%, 90%)
```

| Effective Ratio   | Attacker Losses |
| ----------------- | --------------- |
| 1.0 (barely lost) | 40%             |
| 1.5               | 52%             |
| 2.0               | 65%             |
| 3.0+              | 90%             |

**Losing Defender (20-35%):**

Defenders can retreat to safety - home turf advantage even in defeat.

```
losing_defender_pct = clamp(20% + 10% × (effective_ratio - 1), 20%, 35%)
```

| Effective Ratio   | Defender Losses |
| ----------------- | --------------- |
| 1.0 (barely lost) | 20%             |
| 1.5               | 25%             |
| 2.5+              | 35%             |

**Design Rationale:**

- Attacking is high-risk/high-reward: win decisively or get crushed
- Defending is safer: even a loss is survivable
- Close fights are bloody for everyone (15% winner, 40%+ loser)
- Pairs well with v0.2.0 walls giving defenders additional power

### Step 6: Apply Losses (Weighted Distribution)

Total losses are calculated, then **distributed based on disadvantage weighting**. Units facing
unfavorable matchups suffer higher casualties than those with advantages.

**Loss Distribution Formula:**

For each unit type `i` in the losing army, calculate its vulnerability weight:

```
vulnerability_i = Σⱼ [ (qty_j / N_enemy) × inverse_advantage(type_i, type_j) ]
```

Where `inverse_advantage` is:

- 1.5 if enemy type beats this type (we're weak)
- 1.0 if neutral matchup
- 0.67 if we beat enemy type (we're strong)

Normalize weights to sum to 1.0:

```
weight_i = vulnerability_i / Σ vulnerability_k
```

Apply losses per unit type:

```
losses_i = round(total_losses × weight_i)
```

**Example**: Cavalry + Infantry vs pure Ranged (Ranged beats Cavalry)

- Cavalry vulnerability: 1.5 (weak vs Ranged)
- Infantry vulnerability: 0.67 (strong vs Ranged)
- Total: 2.17
- Cavalry weight: 1.5 / 2.17 = 69%
- Infantry weight: 0.67 / 2.17 = 31%
- If 10 total losses: 7 Cavalry die, 3 Infantry die

**Rounding Rules:**

- Winners: Round DOWN (favor winners)
- Losers: Round UP (punish losers)
- Minimum 1 loss per unit type if that type has units

### Step 7: Calculate Plunder

Winner takes a percentage of loser's **plunderable** resources. Only stored resources can be
plundered - accumulators (uncollected production) are safe.

**Plunder Percentage:**

```
plunder_pct = clamp(10% + 5% × (effective_ratio - 1), 10%, 50%)
```

| Effective Ratio | Plunder % |
| --------------- | --------- |
| 1.0             | 10%       |
| 2.0             | 15%       |
| 5.0             | 30%       |
| 9.0+            | 50%       |

**Protected Resource Pool:**

Defenders have 20% of their stored resources protected from plunder. This scales naturally with
Keep/Warehouse levels and prevents complete economic wipeouts.

```
protected_amount = stored_resources × 0.20
plunderable_amount = stored_resources × 0.80
actual_plunder = plunderable_amount × plunder_pct
```

**Resource Transfer:**

```
for each resource:
    plunderable = stored_resources × 0.80
    plundered_amount = floor(plunderable × plunder_pct)
    transfer(plundered_amount, from: loser, to: winner)
```

**Why Accumulators Are Safe:**

- Accumulators represent "uncollected" production (grain in fields, ore in mines)
- Thematically: raiders loot warehouses, not unharvested crops
- Mechanically: prevents griefing idle players to zero

**Disincentive for Attacking Broke Players:**

If potential plunder is less than Monster Lair rewards (~50 food, ~25 wood), the attacker gains
little while still paying full unit losses. The API may display a warning for low-value targets.

---

## Complete Formula Summary

### Combat Snapshots

Before combat begins, both armies and their modifiers are **snapshotted**:

1. **Army Snapshot**: Unit quantities frozen at combat start
2. **Modifier Cache**: All applicable modifiers batch-loaded once per player

This prevents race conditions (e.g., units finishing training mid-combat) and avoids repeated
database queries during the cross-product calculations.

```rust
// Pseudocode implementation

fn calculate_combat(
    attacker_snapshot: &ArmySnapshot,
    defender_snapshot: &ArmySnapshot,
) -> CombatResult {
    // Step 1: Calculate raw power (no advantages)
    let attacker_power = calculate_raw_power(attacker_snapshot, true);
    let defender_power = calculate_raw_power(defender_snapshot, false);
    let raw_ratio = attacker_power / defender_power;

    // Step 2: Calculate advantage factor
    let advantage_factor = calculate_advantage_factor(
        &attacker_snapshot.army,
        &defender_snapshot.army,
    );

    // Step 3: Final combat ratio
    let combat_ratio = raw_ratio * advantage_factor;

    // Step 4: Determine outcome
    let attacker_wins = combat_ratio >= 1.0;
    let effective_ratio = if attacker_wins { combat_ratio } else { 1.0 / combat_ratio };

    // Step 5: Calculate losses (asymmetric)
    let winner_loss_pct = (0.20 / effective_ratio).clamp(0.02, 0.15);

    let (attacker_loss_pct, defender_loss_pct) = if attacker_wins {
        let loser_pct = (0.20 + 0.10 * (effective_ratio - 1.0)).clamp(0.20, 0.35);
        (winner_loss_pct, loser_pct)
    } else {
        let loser_pct = (0.40 + 0.25 * (effective_ratio - 1.0)).clamp(0.40, 0.90);
        (loser_pct, winner_loss_pct)
    };

    // Step 7: Calculate plunder (only if attacker wins)
    let plunder_pct = if attacker_wins {
        (0.10 + 0.05 * (effective_ratio - 1.0)).clamp(0.10, 0.50)
    } else {
        0.0
    };

    CombatResult {
        attacker_wins,
        combat_ratio,
        attacker_loss_pct,
        defender_loss_pct,
        plunder_pct,
    }
}

fn calculate_raw_power(snapshot: &ArmySnapshot, is_attacker: bool) -> f64 {
    let mut total_power = 0.0;
    let stat_type = if is_attacker { CombatStat::ATK } else { CombatStat::DEF };

    for unit_stack in &snapshot.army.units {
        let base_stat = if is_attacker { unit_stack.base_atk } else { unit_stack.base_def };

        // Use cached modifier (no DB query)
        let modifier = snapshot.modifiers.get(stat_type, unit_stack.unit_type);

        total_power += unit_stack.quantity as f64 * base_stat as f64 * modifier;
    }

    total_power
}

fn calculate_advantage_factor(attacker: &Army, defender: &Army) -> f64 {
    let mut factor = 0.0;
    let n_atk = attacker.total_units() as f64;
    let n_def = defender.total_units() as f64;

    for atk_stack in &attacker.units {
        let atk_weight = atk_stack.quantity as f64 / n_atk;

        for def_stack in &defender.units {
            let def_weight = def_stack.quantity as f64 / n_def;
            let advantage = get_advantage_multiplier(atk_stack.unit_type, def_stack.unit_type);

            factor += atk_weight * def_weight * advantage;
        }
    }

    factor
}

// --- Snapshot Creation (called once before combat) ---

/// Create army snapshot with cached modifiers
fn create_army_snapshot(conn: &mut DbConn, army: Army) -> Result<ArmySnapshot> {
    let modifiers = batch_load_combat_modifiers(conn, &army.player_id)?;
    Ok(ArmySnapshot { army, modifiers })
}

/// Batch load all combat modifiers for a player (called once per snapshot)
fn batch_load_combat_modifiers(
    conn: &mut DbConn,
    player_id: &PlayerKey,
) -> Result<ModifierCache> {
    // Load all combat modifiers in a single query, cache by (stat, unit_type)
    // Implementation queries active_modifiers joined with modifiers table
    // See calc_combat_multiplier below for query pattern
}

// In src/game/combat/modifier_operations.rs
pub fn calc_combat_multiplier(
    conn: &mut DbConn,
    player_id: &PlayerKey,
    unit_type: UnitType,
    stat: CombatStat,
) -> Result<BigDecimal> {
    use crate::schema::active_modifiers::dsl as am;
    use crate::schema::modifiers::dsl as m;

    // Match both specific unit type AND global modifiers (NULL = all units)
    let applied_mods: Vec<AppliedModifier> = am::active_modifiers
        .inner_join(m::modifiers)
        .filter(am::player_id.eq(player_id))
        .filter(m::target_type.eq(ModifierTarget::Combat))
        .filter(
            m::target_unit_type.eq(unit_type)
                .or(m::target_unit_type.is_null())  // Global modifiers
        )
        .filter(m::target_combat_stat.eq(stat))
        .select((ActiveModifier::as_select(), Modifier::as_select()))
        .load::<(ActiveModifier, Modifier)>(conn)?
        .into_iter()
        .map(|(am, m)| m.into_full(am))
        .collect();

    Ok(apply_stacking_rules(&applied_mods))
}
```

---

## Combat Examples

### Example 1: Simple Infantry vs Infantry (Same Faction)

**Scenario**: 100 Human Infantry attack 80 Human Infantry

```
Attacker: 100 Infantry (ATK 10, no faction bonus for Infantry as Human)
Defender: 80 Infantry (DEF 15, no faction bonus for Infantry as Human)

Attacker Power = 100 × 10 × 1.0 × 1.0 = 1000
Defender Power = 80 × 15 × 1.0 × 1.0 = 1200

Combat Ratio = 1000 / 1200 = 0.833

Winner: Defender (ratio < 1.0)
Effective Ratio: 1.2 (inverted)

Defender Losses: max(1%, min(5%, 100/1.2)) = 5% = 4 Infantry
Attacker Losses: min(80%, 30% + 20% × 0.2) = 34% = 34 Infantry

Result: Defender wins, keeps resources
- Defender: 76 Infantry remaining
- Attacker: 66 Infantry remaining
```

### Example 2: Type Advantage

**Scenario**: 100 Orc Infantry attack 100 Elf Ranged

```
Attacker: 100 Orc Infantry (ATK 10 × 1.15 faction = 11.5)
Defender: 100 Elf Ranged (DEF 5 × 1.15 faction = 5.75)

Infantry vs Ranged: 1.5x advantage

Attacker Power = 100 × 11.5 × 1.5 = 1725
Defender Power = 100 × 5.75 × 0.67 = 385.25

Combat Ratio = 1725 / 385.25 = 4.48

Winner: Attacker
Attacker Losses: max(1%, min(5%, 100/4.48)) = 5% (capped) = 5 Infantry
Defender Losses: min(80%, 30% + 20% × 3.48) = 80% (capped) = 80 Ranged
Plunder: min(50%, 10% + 5% × 3.48) = 27.4%

Result: Decisive attacker victory
- Attacker: 95 Infantry remaining, gains 27.4% of defender's resources
- Defender: 20 Ranged remaining
```

### Example 3: Mixed Army Composition

**Scenario**: Mixed Human army attacks Mixed Orc army

```
Attacker (Human):
- 50 Infantry (ATK 10)
- 50 Cavalry (ATK 12 × 1.15 faction = 13.8)

Defender (Orc):
- 40 Infantry (DEF 15 × 1.15 faction = 17.25)
- 40 Artillery (DEF 3)

Matchups:
- Infantry vs Infantry: 1.0x
- Infantry vs Artillery: 0.67x (Artillery beats Infantry)
- Cavalry vs Infantry: 1.0x
- Cavalry vs Artillery: 1.5x (Cavalry beats Artillery)

Attacker Power (weighted by defender composition):
- 50 Inf vs 40 Inf (50% of def): 50 × 10 × 1.0 × 0.5 = 250
- 50 Inf vs 40 Art (50% of def): 50 × 10 × 0.67 × 0.5 = 167.5
- 50 Cav vs 40 Inf (50% of def): 50 × 13.8 × 1.0 × 0.5 = 345
- 50 Cav vs 40 Art (50% of def): 50 × 13.8 × 1.5 × 0.5 = 517.5
Total Attacker Power = 1280

Defender Power (weighted by attacker composition):
- 40 Inf vs 50 Inf (50% of atk): 40 × 17.25 × 1.0 × 0.5 = 345
- 40 Inf vs 50 Cav (50% of atk): 40 × 17.25 × 1.0 × 0.5 = 345
- 40 Art vs 50 Inf (50% of atk): 40 × 3 × 1.5 × 0.5 = 90
- 40 Art vs 50 Cav (50% of atk): 40 × 3 × 0.67 × 0.5 = 40.2
Total Defender Power = 820.2

Combat Ratio = 1280 / 820.2 = 1.56

Winner: Attacker
Attacker Losses: max(1%, min(5%, 100/1.56)) = 5% = 5 units
Defender Losses: min(80%, 30% + 20% × 0.56) = 41.2% = 33 units
Plunder: min(50%, 10% + 5% × 0.56) = 12.8%

Result: Attacker victory
- Attacker: ~95 units remaining (proportional: 47 Inf, 48 Cav)
- Defender: ~47 units remaining (proportional: 23 Inf, 24 Art)
- Attacker gains 12.8% of defender's resources
```

---

## Edge Cases

### No Defenders

If defender has 0 units:

- Attacker automatically wins
- Attacker loses 0%
- Plunder is 50% (maximum)

### No Attackers

If attacker has 0 units:

- Attack is invalid (rejected by API)

### Single Unit vs Single Unit

Minimum losses apply:

- Winner loses at least 1 unit (if they have any)
- Loser loses at least 1 unit

### Equal Forces

When `combat_ratio = 1.0`:

- Attacker wins (tie-breaker)
- Both sides lose ~30%
- Plunder is 10%

### Overwhelming Force (Ratio > 10)

Losses and plunder are capped:

- Winner loses 1%
- Loser loses 80%
- Plunder is 50%

---

## PvE Combat System

Empire v0.1.0 focuses primarily on **PvE (Player vs Environment)** combat. Players attack NPC
targets to gain resources and test army compositions before engaging in PvP.

### Design Goals

- Provide safe progression for new players
- Allow resource farming without player conflict
- Create varied challenges with different army compositions
- Enable strategic practice before PvP
- Offer endgame challenge via infinite tower

### NPC Target Types

NPCs are attackable targets that don't have player accounts. They exist on the world map and respawn
after being defeated.

#### World Map Targets

| NPC Type           | Difficulty | Army Size | Resources | Respawn  | Description                   |
| ------------------ | ---------- | --------- | --------- | -------- | ----------------------------- |
| **Monster Lair**   | Trivial    | 5-10      | Minimal   | 30 min   | Wild beasts, tutorial targets |
| **Bandit Camp**    | Easy       | 15-25     | Low       | 1 hour   | Outlaws with mixed units      |
| **Outlaw Hideout** | Medium     | 40-60     | Medium    | 4 hours  | Organized criminals           |
| **Rebel Fort**     | Hard       | 100-150   | High      | 12 hours | Fortified military deserters  |

**NPC Army Compositions:**

| NPC Type       | Infantry | Ranged | Cavalry | Artillery | Strategy Hint                    |
| -------------- | -------- | ------ | ------- | --------- | -------------------------------- |
| Monster Lair   | 100%     | 0%     | 0%      | 0%        | Any army works                   |
| Bandit Camp    | 60%      | 30%    | 10%     | 0%        | Bring Infantry for Ranged        |
| Outlaw Hideout | 40%      | 40%    | 20%     | 0%        | Mixed army recommended           |
| Rebel Fort     | 30%      | 30%    | 20%     | 20%       | Need balanced force with Cavalry |

#### The Infinite Tower

The **Tower of Trials** is an endgame PvE challenge with infinite floors of increasing difficulty.
Each floor has a preset army composition that tests different strategies.

**Tower Mechanics:**

- Progress is persistent (floor reached is saved)
- Each floor has fixed army composition (deterministic challenge)
- Difficulty scales exponentially with floor number
- Rewards increase with floor depth
- No respawn timer - always available
- Leaderboard tracks highest floor reached

**Floor Scaling:**

```
Floor N army size = base_size × (1.1 ^ N)
Floor N rewards = base_reward × (1.05 ^ N)
```

**Example Floors:**

| Floor | Army Size | Composition Focus      | Challenge Type            |
| ----- | --------- | ---------------------- | ------------------------- |
| 1-5   | 20-30     | Single unit type       | Learn counter-picking     |
| 6-10  | 40-60     | Two unit types         | Mixed army basics         |
| 11-20 | 80-150    | Three unit types       | Full composition strategy |
| 21-50 | 200-500   | Optimized compositions | Min-max required          |
| 51+   | 600+      | Perfect counters       | Whale territory           |

**Floor Themes (Rotating):**

- **Infantry Gauntlet** (Floors 1, 5, 9...): Heavy Infantry, bring Ranged or Artillery
- **Archer's Gallery** (Floors 2, 6, 10...): Heavy Ranged, bring Infantry
- **Cavalry Charge** (Floors 3, 7, 11...): Heavy Cavalry, bring Ranged
- **Siege Line** (Floors 4, 8, 12...): Heavy Artillery, bring Cavalry

### NPC Combat Differences

- **No Faction Bonuses**: NPCs fight at base stats only
- **No Counterattack**: Attacking NPCs doesn't risk your realm
- **Respawn System**: World targets regenerate army + resources after timer
- **Tower Persistence**: Tower progress is saved, no respawn needed
- **No Plunder Cap**: Tower rewards are fixed per floor, not plunder-based

### NPC Rewards

| Target Type    | Food     | Wood     | Stone   | Gold    | Special         |
| -------------- | -------- | -------- | ------- | ------- | --------------- |
| Monster Lair   | 50-100   | 25-50    | 0       | 0       | -               |
| Bandit Camp    | 100-200  | 100-200  | 50-100  | 25-50   | -               |
| Outlaw Hideout | 300-500  | 300-500  | 200-300 | 100-200 | -               |
| Rebel Fort     | 800-1200 | 800-1200 | 500-800 | 300-500 | -               |
| Tower (per 5)  | Scaling  | Scaling  | Scaling | Scaling | Cosmetics (TBD) |

---

## API Integration

### Attack Request

```json
POST /game/combat/attack
{
  "target_type": "player",  // or "npc" for PvE targets
  "target_id": "01HXYZ...",
  "units": {
    "infantry": 50,
    "ranged": 30,
    "cavalry": 20,
    "artillery": 0
  }
}
```

### Attack Response

```json
{
  "result": "victory", // or "defeat"
  "combat_ratio": 1.56,
  "attacker_losses": {
    "infantry": 3,
    "ranged": 2,
    "cavalry": 1,
    "artillery": 0,
    "total": 6
  },
  "defender_losses": {
    "infantry": 15,
    "ranged": 10,
    "cavalry": 8,
    "artillery": 0,
    "total": 33
  },
  "plunder": {
    "food": 500,
    "wood": 300,
    "stone": 200,
    "gold": 100
  },
  "combat_log_id": "01HXYZ..."
}
```

---

## Future Expansion (Out of Scope for v0.1.0)

### v0.2.0+: Defensive Structures

- Walls add flat DEF bonus to all defending units
- Wall level determines bonus magnitude
- Garrison mechanic for placing units on walls

### v0.3.0+: Advanced Combat

- Terrain modifiers (hills, forests, rivers)
- Weather effects (rain reduces Ranged effectiveness)
- Combat formations (defensive stance, aggressive stance)
- **Hero units** with special abilities:
  - **Disruptor**: Negates enemy faction bonuses (anti-elite specialist)
  - **Commander**: Boosts nearby unit ATK/DEF
  - **Assassin**: Targets enemy heroes directly
  - **Healer**: Reduces friendly casualties

### v0.4.0+: Social Combat

- Alliance reinforcements
- Joint attacks on targets
- War declarations and peace treaties
- Siege mechanics for prolonged attacks

---

## Design Rationale

### Why Attacker Wins Ties

In v0.1.0, when combat ratio equals exactly 1.0, the **attacker wins**. This design decision
supports the core game loop and leaves room for future defender advantages.

**Arguments for Attacker Wins Ties:**

| Benefit                   | Explanation                                             |
| ------------------------- | ------------------------------------------------------- |
| Encourages offensive play | Supports "Build → Train → Attack → Expand" core loop    |
| Prevents turtling meta    | Players can't passively sit and accumulate without risk |
| Simple mental model       | "Equal force = I can win" is intuitive                  |
| Rewards initiative        | Taking action should be rewarded over passive defense   |

**Arguments Against (Considered but Rejected for v0.1.0):**

| Concern                  | Mitigation                                                |
| ------------------------ | --------------------------------------------------------- |
| No home advantage        | Walls will provide this in v0.2.0+                        |
| Snowball effect          | Loss caps (80% max) prevent complete wipeouts             |
| New player vulnerability | PvE focus means new players don't need to PvP immediately |

**Future: Walls as Defender Advantage**

Rather than inherent defender bonuses, Empire will use **Walls** (v0.2.0+) to provide defender
advantages:

- Players who invest in Walls earn defensive bonuses
- Creates strategic choice: offense (more units) vs defense (walls)
- Active players can attack before enemies build walls
- Defensive players can turtle if they invest properly

This design gives players **agency** over their defensive capabilities rather than free passive
bonuses.

---

## Balance Considerations

### Current Balance State

- Infantry: Good all-rounder, cheap to produce
- Ranged: High risk/reward, devastating against Cavalry
- Cavalry: Fast training speed faction bonus, mobile
- Artillery: Expensive but powerful siege units

### Known Balance Questions

1. ~~Should Goblins get a minor combat bonus?~~ **Resolved**: No - 20% training speed enables swarm
   tactics
2. Is 80% max losses too punishing? (Current: Acceptable, prevents complete wipeout)
3. Should plunder affect storage buildings? (Current: No, takes from raw totals)
4. Should attacker win ties? (Current: Yes - see Design Rationale section)

### Tuning Levers

If balance adjustments needed:

- Adjust advantage multipliers (currently 1.5x / 0.67x)
- Adjust faction bonus percentages (currently 15%)
- Adjust loss formulas (base %, scaling %)
- Adjust plunder caps (currently 10-50%)

---

## Implementation Checklist

### Core Combat Engine

- [ ] Create `src/game/combat/mod.rs`
- [ ] Create `src/game/combat/calculator.rs` with pure calculation functions
- [ ] Create `src/domain/combat.rs` with domain models
- [ ] Implement advantage matrix lookup
- [ ] Implement faction bonus lookup
- [ ] Implement power calculation (cross-product formula)
- [ ] Implement weighted loss distribution
- [ ] Implement plunder calculation

### PvE System

- [ ] Create NPC target types (Monster, Bandit, Outlaw, Rebel)
- [ ] Create NPC spawn/respawn system
- [ ] Create Tower of Trials floor generation
- [ ] Create Tower progress persistence
- [ ] Create Tower leaderboard

### API & Persistence

- [ ] Create combat history table for logging
- [ ] Create combat API endpoints
- [ ] Create NPC attack endpoints
- [ ] Create Tower endpoints

### Testing

- [ ] Write unit tests for combat calculator
- [ ] Write unit tests for loss distribution
- [ ] Write integration tests for full combat flow
- [ ] Write integration tests for PvE targets

---

**Last Updated**: 2025-12-26 **Maintainer**: Lucas Vienna (@lucasvienna) **Status**: Draft -
Awaiting Review
