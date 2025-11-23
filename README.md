# Empire

## TODO

### Minimal Viable Game Loop (Priority Order)

#### 1. Complete Building System (Highest Priority)

**Audit Status (2025-11-17)**:

- ✅ Building construction/upgrade flow working (APIs, transactions, resource validation)
- ✅ Resource production working (background jobs, modifier integration)
- ❌ Building prerequisites NOT implemented (neither building-level nor tech-tree dependencies)
- ⚠️ Job queue exists but buildings use manual confirmation instead of auto-completion

**What's Working**:

- Construction: `POST /game/buildings` with resource costs, max count enforcement
- Upgrades: `PUT /game/buildings/{id}/upgrade` sets ETA, deducts resources
- Confirmation: `PUT /game/buildings/{id}/confirm_upgrade` increments level after ETA
- Resource production: Background processor generates resources based on building rates + modifiers

**Critical Gaps**:

- [x] **Building Prerequisites System**
    - [x] Create `building_requirement` table (required_building_id, required_level, required_tech_id)
    - [ ] Implement prerequisite validation in `construct_building()`
    - [ ] Implement prerequisite validation for tech tree nodes
    - [x] Add prerequisite data to seed files
- [ ] **Available Buildings API** (`get_available_buildings` currently stubbed)
    - [ ] Show all buildings with availability status (buildable vs locked)
    - [ ] Include reason for locked state (max count, missing prereqs, faction mismatch)
    - [ ] Filter by player's faction
- [ ] **Building → Production Rate Verification**
    - [ ] Confirm building levels affect resource generation rates
    - [ ] Document rate calculation in code comments

**Design Decisions**:

- Keep manual upgrade confirmation (satisfying "number goes up" moment)
- Show unavailable buildings with clear feedback on why they're locked

#### 2. Unit Training (Second Priority)

- [ ] Create units from buildings (barracks, stables, etc.)
- [ ] Implement resource costs for training
- [ ] Add training time via job queue
- [ ] Implement unit storage/capacity limits

#### 3. Basic Combat (Third Priority)

- [ ] Implement attack mechanics (players/NPCs/resource nodes)
- [ ] Simple damage calculation (attacker vs defender strength)
- [ ] Calculate unit losses based on combat outcome
- [ ] Implement resource plunder on victory

#### 4. Tech Research (Fourth Priority)

- [ ] Create research building (academy/library)
- [ ] Implement tech unlocks (units, buildings, bonuses)
- [ ] Add research time + resource costs
- [ ] Enforce tech prerequisites

### Future Enhancements (Post-MVP)

- Complex combat simulations
- Alliance/diplomacy systems
- Advanced modifier stacking
- Marketplace/trading
- Achievements/quests