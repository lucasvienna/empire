# Architectural Changes for a Domain Layer Modifier System

## 1. Core Domain Model Changes

### New Domain Entities

- **Modifier**: Core entity representing any modifier to game mechanics
    - Properties: source, target resource/attribute, effect type, magnitude, duration
    - Types: percentage modifiers, flat bonuses/penalties, capacity increases/decreases, etc.

- **ModifierSource**: Enumeration of possible modifier origins
    - Faction passives, consumable items, skill tree nodes, research, events, guild effects

- **ModifierDuration**: Models how long a modifier lasts
    - Permanent (faction traits)
    - Temporary with expiration (consumables, events)
    - Conditional (active only when requirements are met)

- **ModifierTarget**: What the modifier affects
    - Resource production, capacity, training speed, combat stats, etc.

## 2. Repository Layer Changes

### New Repositories

- **ModifierRepository**: Stores and retrieves active modifiers for users
    - Queries for modifiers by user, expiration status, type, etc.
    - Handles CRUD operations for modifier persistence

### Enhanced Existing Repositories

- **UserRepository**: Add associations to user's active and permanent modifiers
- **FactionRepository**: Extract faction passive effects into proper modifier entities
- **SkillRepository**: Map skill tree nodes to modifier effects
- **ItemRepository**: Define consumable item effects as modifiers

## 3. Service Layer Additions

### New Services

- **ModifierService**: Core service for modifier management
    - Apply/remove modifiers to users
    - Calculate aggregate modifier effects for a user
    - Track modifier expirations
    - Handle modifier stacking rules

- **ModifierSchedulingService**: Manages temporal aspects of modifiers
    - Schedules expiration events
    - Handles recurring effects
    - Coordinates with resource accumulation timing

### Modifications to Existing Services

- **ResourceService**: Decouple base resource calculations from modifier applications
    - Accept modifier multipliers as inputs
    - Delegate modifier calculations to ModifierService

- **SkillService**: Convert skill effects into standardized modifiers
    - When a skill is learned, create corresponding modifier entities

- **ItemService**: Implement consumable item effects as modifier applications
    - When an item is used, invoke ModifierService to create temporary modifiers

## 4. Infrastructure Layer Changes

### Persistence Changes

- New database tables:
    - `modifiers`: Core modifier definitions
    - `user_active_modifiers`: Modifiers currently active for users
    - `modifier_history`: Record of past modifiers for analytics

### Cache Strategy

- Implement a specialized caching mechanism for active modifiers
    - Cache user's aggregated modifier multipliers per resource type
    - Set cache invalidation based on next modifier expiration

### Background Processing

- Job scheduler for modifier-related operations:
    - Expiration handling
    - Resource collection at optimal times
    - Modifier renewal for recurring effects

## 5. API/Interface Layer Updates

### New API Endpoints

- Endpoints to view active modifiers
- Endpoints to use consumable items (applying modifiers)
- Endpoints to display modifier effects on resource generation rates

### UI Considerations

- Visual indicators for active modifiers
- Timers for temporary modifiers
- Resource production rate displays showing modifier contributions
- Clear differentiation between positive and negative modifiers

## 6. System Interactions

### Event System

- Create a modifier event system to propagate changes:
    - `ModifierApplied`: When a new modifier takes effect
    - `ModifierExpired`: When a temporary modifier ends
    - `ModifierRemoved`: When a modifier is explicitly removed
    - `ModifierChanged`: When a modifier's parameters change

### Integration Points

- Connect modifier system with:
    - Resource generation calculations
    - Combat stat calculations
    - Training speed adjustments
    - Building construction times

## 7. Testing Architecture

### Test Infrastructure

- Develop specialized test fixtures for modifier scenarios
- Create time manipulation utilities for testing temporal modifiers
- Build modifier comparison tools for verification

## 8. Migration Strategy

### Data Migration

- Convert existing faction bonuses to proper modifier entities
- Establish initial modifier records for all users based on their faction

### Code Migration

- Refactor current hard-coded bonuses to use modifier system
- Phase rollout of modifier-aware resource calculation
- Implement backward compatibility for older systems

## 9. Monitoring and Analytics

### Observability

- Metrics for modifier application frequency
- Monitoring for modifier calculation performance
- Alerts for unexpected modifier behaviors

### Analytics

- Track modifier usage patterns
- Measure economic impact of different modifier types
- Identify balance issues through modifier effect analysis

## Example System Interactions

To illustrate how these architectural components would interact, consider this flow for resource collection:

1. **Resource Collection Request**:
    - User triggers resource collection

2. **ResourceService Processing**:
    - Retrieves base resource rates from database
    - Requests current modifier multipliers from ModifierService
    - Applies multipliers to calculate actual collection amounts
    - Updates user resources

3. **ModifierService Calculations**:
    - Retrieves all active modifiers for user
    - Filters modifiers by resource type
    - Aggregates modifier effects according to stacking rules
    - Returns total multipliers for each resource type
    - Schedules next calculation if modifiers will expire soon

## Implementation Phases

1. **Foundation Phase**:
    - Define core domain entities
    - Implement basic modifier persistence
    - Create ModifierService with simple aggregation

2. **Integration Phase**:
    - Connect faction passive modifiers to the system
    - Integrate with resource calculations
    - Implement modifier duration tracking

3. **Expansion Phase**:
    - Add consumable item modifier support
    - Implement skill tree integration
    - Develop caching strategy

4. **Optimization Phase**:
    - Add analytics and monitoring
    - Optimize performance-critical paths
    - Implement advanced stacking rules

## Faction Modifier Implementation

### Trigger-Based Faction Modifier Management

The system automatically manages faction-specific modifiers through database triggers, ensuring a consistent application of faction bonuses:

1. **Trigger Events**
    - On user creation (`AFTER INSERT`)
    - On faction change (`AFTER UPDATE OF faction`)

2. **Trigger Function Responsibilities**
    - Removes existing faction modifiers when a user changes faction
    - Applies new faction modifiers based on the user's faction
    - Records all modifier changes in the history table
    - Validates faction modifier existence (fails if no modifiers are found for a faction)

3. **Modifier History Tracking**
    - Records application of initial faction modifiers
    - Tracks removal and application during faction changes
    - Maintains audit trail with reasons for changes

### Advantages of Trigger-Based Approach

- **Data Consistency**: Ensures faction modifiers are always applied correctly
- **Atomic Operations**: Changes to faction modifiers happen in the same transaction as faction changes
- **Audit Trail**: Automatic tracking of all faction modifier changes
- **Error Prevention**: Built-in validation prevents missing or incorrect faction modifier states

### Integration Points

- **User Creation Flow**: Automatic application of initial faction modifiers
- **Faction Change Flow**: Atomic update of all related modifiers
- **Resource Calculation**: Faction modifiers automatically included in modifier queries

## Additional Considerations for Negative Modifiers

Since we're implementing a generic modifier system that can include both positive and negative effects:

- **Stacking Rules**: Define how positive and negative modifiers interact
    - Additive vs. multiplicative stacking
    - Minimum/maximum caps on total modifiers
    - Order of operations for different modifier types

- **Debuff Resistance**: Consider implementing mechanisms for resisting negative modifiers
    - Faction traits that reduce negative effects
    - Items that provide immunity to certain debuff types

- **UI Clarity**: Ensure the interface clearly distinguishes between positive and negative effects
    - Color coding (green/red)
    - Separate listings for bonuses and penalties
    - Net effect calculations

- **Balance Mechanics**: Create systems to ensure fair play
    - Limits on stacking multiple negative modifiers
    - Diminishing returns on both positive and negative effects
    - Recovery mechanisms from severe penalties

This architectural approach keeps the modifier logic in the domain layer while providing clear integration points with other systems. It maintains separation of concerns, with the modifier system
responsible for calculating the effects and other services applying those effects to their respective domains.