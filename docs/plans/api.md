# Empire Game API Implementation Plan

## Overview

This document outlines a comprehensive API design for the Empire multiplayer strategy game, designed
to support both web and mobile clients. The API is organized by path-based concerns and follows
RESTful principles with JSON responses and JWT-based authentication.

## API Structure by Path

The API is organized into the following main path sections:

- `/auth/` - Authentication and session management
- `/player/` - Player profile management
- `/game/` - Core gameplay mechanics (factions, buildings, resources, combat, etc.)
- `/social/` - Social features (guilds, diplomacy, trading)
- `/dashboard/` - Overview and summary information
- `/admin/` - Administrator dashboard and management tools
- `/health/` - System observability and health monitoring
- `/liveops/` - LiveOps dashboard with real-time analytics and metrics
- `/ws/` - Real-time features and chat

---

## /auth/ — Authentication & Session Management

### Rationale

Secure user authentication is fundamental for a multiplayer game. JWT tokens provide stateless
authentication suitable for both web and mobile clients, while session management enables proper
user state tracking.

### Endpoints

#### POST /auth/register

- **Purpose**: Create new player account
- **Body**: `{ "username": "string", "email": "string", "password": "string" }`
- **Response**: `{ "status": "success|error", "message": "string" }`
- **Rationale**: Faction selection happens after registration. It is crucial, as it determines
  passive bonuses and available buildings

#### POST /auth/login

- **Purpose**: Authenticate existing player
- **Body**: `{ "username": "string", "password": "string" }`
- **Response**:
  `{ "token": "jwt_token", "player": { "id": "uuid", "name": "string", "faction": "string" } }`

#### POST /auth/logout

- **Purpose**: Invalidate current session
- **Headers**: `Authorization: Bearer <token>` or `Cookie: rsession <token>` or
  `Cookie: rstoken <token>`
- **Response**: `{ "status": "success" }`

#### GET /auth/session

- **Purpose**: Get current player information
- **Headers**: `Cookie: rsession <token>`
- **Response**:
  `{ "id": "uuid", "name": "string", "email": "string", "faction": "string", "created_at": "datetime" }`

---

## /player/ — Player Profile Management

### Rationale

Players need to view and manage their profile information, including faction changes (if allowed)
and account settings.

### Endpoints

#### GET /player/profile

- **Purpose**: Get detailed player profile
- **Response**:
  `{ "id": "uuid", "name": "string", "email": "string", "faction": "string", "created_at": "datetime", "updated_at": "datetime" }`

#### PUT /player/profile

- **Purpose**: Update player profile
- **Body**: `{ "name": "string", "email": "string" }`
- **Response**: Updated player profile

#### PUT /player/faction

- **Purpose**: Change player faction (if game rules allow)
- **Body**: `{ "faction": "human|orc|elf|dwarf|goblin" }`
- **Response**: Updated player profile
- **Rationale**: Faction changes may have costs or restrictions, requiring separate endpoint

---

## /game/ — Core Gameplay APIs

### Rationale

Core gameplay mechanics including factions, buildings, resources, modifiers, research, religion,
combat, and trading form the heart of the Empire game experience. These endpoints provide all the
functionality needed for players to build, manage, and expand their empires.

### Factions

#### GET /game/factions

- **Purpose**: List all available factions with their bonuses
- **Response**:

```json
[
  {
    "id": "human",
    "name": "Humans",
    "bonuses": [
      {
        "type": "resource",
        "target": "wood",
        "value": 15,
        "description": "+15% Wood production"
      },
      {
        "type": "unit",
        "target": "cavalry",
        "value": 15,
        "description": "+15% Cavalry training speed and ATK/DEF"
      }
    ]
  }
]
```

#### GET /game/factions/{faction_id}

- **Purpose**: Get detailed information about specific faction
- **Response**: Detailed faction information with all bonuses and lore

### Buildings

#### GET /game/buildings

- **Purpose**: List all player's buildings with current status
- **Response**:

```json
[
  {
    "id": "uuid",
    "building_id": 1,
    "name": "Keep",
    "level": 3,
    "max_level": 10,
    "max_count": 1,
    "upgrade_time": "00:45:30",
    "requirements": {
      "food": 1000,
      "wood": 800,
      "stone": 600,
      "gold": 400
    }
  }
]
```

#### GET /game/buildings/{building_id}

- **Purpose**: Get detailed information about specific building
- **Response**: Detailed building information including upgrade costs and benefits

#### POST /game/buildings/{building_id}/upgrade

- **Purpose**: Start building upgrade
- **Response**: Updated building information with upgrade timer
- **Rationale**: Upgrades take time and consume resources, requiring confirmation

#### POST /game/buildings/{building_id}/upgrade/confirm

- **Purpose**: Confirm completed upgrade (collect upgrade)
- **Response**: Updated building information
- **Rationale**: Separates upgrade initiation from completion for timing mechanics

#### GET /game/buildings/available

- **Purpose**: List buildings available for construction
- **Response**: Available buildings with construction requirements and faction restrictions
- **Rationale**: Different factions may have access to different buildings

#### POST /game/buildings/construct

- **Purpose**: Construct new building
- **Body**: `{ "building_id": 1 }`
- **Response**: New building information
- **Rationale**: Building construction is separate from upgrades and may have placement limits

### Resources

#### GET /game/resources

- **Purpose**: Get current resource status with production rates
- **Response**:

```json
{
  "resources": {
    "food": {
      "current": 1500,
      "capacity": 2000,
      "production_rate": 120
    },
    "wood": {
      "current": 800,
      "capacity": 1500,
      "production_rate": 100
    },
    "stone": {
      "current": 600,
      "capacity": 1200,
      "production_rate": 80
    },
    "gold": {
      "current": 400,
      "capacity": 800,
      "production_rate": 60
    },
    "population": {
      "current": 100,
      "capacity": 150,
      "production_rate": 5
    }
  },
  "last_collected": "2025-07-20T16:30:00Z",
  "next_full": "2025-07-20T18:15:00Z"
}
```

#### POST /game/resources/collect

- **Purpose**: Collect accumulated resources
- **Response**: Updated resource status
- **Rationale**: Manual collection allows players to optimize resource management

#### GET /game/resources/production

- **Purpose**: Get detailed production breakdown with modifiers
- **Response**:

```json
{
  "base_rates": {
    "food": 100,
    "wood": 80,
    "stone": 60,
    "gold": 50
  },
  "modifiers": [
    {
      "source": "faction",
      "type": "wood",
      "value": 15,
      "description": "Human wood bonus"
    },
    {
      "source": "building",
      "type": "food",
      "value": 25,
      "description": "Farm level 3 bonus"
    }
  ],
  "final_rates": {
    "food": 125,
    "wood": 92,
    "stone": 60,
    "gold": 50
  }
}
```

- **Rationale**: Transparency in production calculations helps players optimize their strategy

### Modifiers

#### GET /game/modifiers/active

- **Purpose**: List all active modifiers affecting the player
- **Response**:

```json
[
  {
    "id": "uuid",
    "name": "Human Wood Mastery",
    "description": "Faction bonus to wood production",
    "type": "percentage",
    "magnitude": 15,
    "target": "resource",
    "target_resource": "wood",
    "source": "faction",
    "expires_at": null,
    "stacking_group": "faction_wood"
  }
]
```

#### GET /game/modifiers/history

- **Purpose**: Get modifier application history
- **Query**: `?limit=50&offset=0`
- **Response**: Paginated list of past modifiers
- **Rationale**: Helps players understand their progression and temporary effects

#### POST /game/modifiers/apply

- **Purpose**: Apply consumable item or temporary modifier
- **Body**: `{ "item_id": "uuid", "target": "self" }`
- **Response**: Applied modifier information
- **Rationale**: Consumable items and temporary boosts are key gameplay elements

### Research

#### GET /game/research/trees

- **Purpose**: Get all research trees with current progress
- **Response**:

```json
{
  "academy": {
    "name": "Military Academy",
    "current_research": {
      "id": "infantry_training",
      "progress": 75,
      "completion_time": "2025-07-20T20:00:00Z"
    },
    "available": [
      {
        "id": "cavalry_tactics",
        "name": "Cavalry Tactics",
        "cost": {
          "gold": 500
        },
        "requirements": ["infantry_training"]
      }
    ],
    "completed": ["basic_combat", "infantry_training"]
  }
}
```

#### POST /game/research/start

- **Purpose**: Start new research
- **Body**: `{ "research_id": "cavalry_tactics", "building_type": "academy" }`
- **Response**: Research progress information

#### POST /game/research/complete

- **Purpose**: Complete finished research
- **Body**: `{ "research_id": "cavalry_tactics" }`
- **Response**: Completed research benefits

### Military

#### GET /game/military/units

- **Purpose**: List all player's military units
- **Response**:

```json
[
  {
    "type": "infantry",
    "count": 50,
    "attack": 10,
    "defense": 8,
    "upkeep": {
      "food": 2,
      "gold": 1
    },
    "training_time": "00:15:00",
    "garrison_capacity": 25
  }
]
```

#### POST /game/military/train

- **Purpose**: Train new units
- **Body**: `{ "unit_type": "infantry", "quantity": 10, "building_id": "uuid" }`
- **Response**: Training queue information

#### GET /game/military/training

- **Purpose**: Get current training queue
- **Response**: List of units being trained with completion times

#### POST /game/military/garrison

- **Purpose**: Garrison units in walls for defense
- **Body**: `{ "unit_assignments": [{ "type": "infantry", "count": 20 }] }`
- **Response**: Updated garrison status

### Religion

#### GET /game/religion/deities

- **Purpose**: List available deities with their bonuses
- **Response**:

```json
[
  {
    "id": "war_goddess",
    "name": "Bellona",
    "domain": "War",
    "bonuses": [
      {
        "type": "training_speed",
        "value": 20,
        "description": "+20% military training speed"
      }
    ],
    "miracles": [
      {
        "id": "divine_fury",
        "name": "Divine Fury",
        "cost": {
          "faith": 100
        },
        "cooldown": "24h"
      }
    ]
  }
]
```

#### POST /game/religion/worship

- **Purpose**: Choose deity to worship
- **Body**: `{ "deity_id": "war_goddess" }`
- **Response**: Worship confirmation with active bonuses

#### POST /game/religion/miracle

- **Purpose**: Invoke deity miracle
- **Body**: `{ "miracle_id": "divine_fury", "target": "self" }`
- **Response**: Miracle effect confirmation

#### GET /game/religion/faith

- **Purpose**: Get current faith points and generation rate
- **Response**: Faith status with generation breakdown

---

## /social/ — Social Features

### Rationale

Social interaction drives player engagement. Guild systems, trading, and diplomacy create community
aspects essential for multiplayer gameplay.

### Guilds

#### GET /social/guilds

- **Purpose**: List available guilds or player's current guild
- **Response**: Guild information with member counts and benefits

#### POST /social/guilds/join

- **Purpose**: Join a guild
- **Body**: `{ "guild_id": "uuid" }`
- **Response**: Guild membership confirmation

### Trading

#### GET /social/market

- **Purpose**: View trading market
- **Query**: `?resource=wood&sort=price&limit=20`
- **Response**: Available trades with prices and quantities

#### POST /social/market/trade

- **Purpose**: Create trade offer
- **Body**: `{ "offer": { "wood": 1000 }, "request": { "gold": 500 }, "duration": "24h" }`
- **Response**: Trade listing confirmation

### Diplomacy

#### GET /social/diplomacy/alliances

- **Purpose**: View current alliances and diplomatic status
- **Response**: Alliance information with benefits and obligations

#### POST /social/diplomacy/propose

- **Purpose**: Propose alliance or diplomatic action
- **Body**: `{ "target_player": "uuid", "type": "alliance", "terms": {} }`
- **Response**: Diplomatic proposal status

---

## /dashboard/ — Overview Mode

### Rationale

Players need comprehensive overview of their empire status for strategic decision-making. The
dashboard provides a centralized view of all key game metrics and notifications.

### Endpoints

#### GET /dashboard/overview

- **Purpose**: Get complete empire overview
- **Response**:

```json
{
  "player": {
    "name": "string",
    "faction": "string",
    "level": 15
  },
  "resources": {
    "summary": "resource_status"
  },
  "buildings": {
    "count": 12,
    "upgrading": 2
  },
  "military": {
    "total_units": 150,
    "training": 25
  },
  "research": {
    "active_projects": 1,
    "completed": 8
  },
  "notifications": [
    {
      "type": "building_complete",
      "message": "Farm upgrade completed",
      "timestamp": "datetime"
    }
  ]
}
```

#### GET /dashboard/notifications

- **Purpose**: Get player notifications and alerts
- **Query**: `?unread=true&limit=20`
- **Response**: Paginated notifications with read status

#### POST /dashboard/notifications/{id}/read

- **Purpose**: Mark notification as read
- **Response**: Updated notification status

---

## /admin/ — Administrator Dashboard & Management

### Rationale

Game administrators need comprehensive tools to manage users, monitor system health, handle
moderation tasks, configure game settings, and maintain operational oversight. These endpoints
provide administrative control while maintaining security through role-based access control.

### User Management

#### GET /admin/users

- **Purpose**: List all users with filtering and pagination
- **Query**: `?search=username&faction=human&status=active&limit=50&offset=0`
- **Response**:

```json
{
  "users": [
    {
      "id": "uuid",
      "name": "string",
      "email": "string",
      "faction": "human",
      "status": "active",
      "level": 15,
      "last_login": "2025-07-20T16:30:00Z",
      "created_at": "2025-07-01T10:00:00Z",
      "flags": ["premium", "verified"]
    }
  ],
  "total": 1250,
  "page": 1,
  "pages": 25
}
```

#### GET /admin/users/{user_id}

- **Purpose**: Get detailed user information
- **Response**: Comprehensive user profile with game statistics, transaction history, and moderation
  notes

#### PUT /admin/users/{user_id}/status

- **Purpose**: Update user status (active, suspended, banned)
- **Body**: `{ "status": "suspended", "reason": "Terms violation", "duration": "7d" }`
- **Response**: Updated user status with audit trail

#### POST /admin/users/{user_id}/resources

- **Purpose**: Grant or adjust user resources (for support/compensation)
- **Body**:
  `{ "resources": { "gold": 1000, "food": 500 }, "reason": "Compensation for server downtime" }`
- **Response**: Updated resource status with transaction record

### System Configuration

#### GET /admin/config

- **Purpose**: Get current game configuration settings
- **Response**:

```json
{
  "game_settings": {
    "maintenance_mode": false,
    "new_registrations": true,
    "resource_multiplier": 1.0,
    "event_active": true
  },
  "feature_flags": {
    "new_building_system": true,
    "enhanced_combat": false,
    "seasonal_events": true
  }
}
```

#### PUT /admin/config

- **Purpose**: Update game configuration
- **Body**: `{ "game_settings": { "maintenance_mode": true }, "reason": "Scheduled maintenance" }`
- **Response**: Updated configuration with change audit

#### GET /admin/config/history

- **Purpose**: Get configuration change history
- **Query**: `?limit=20&offset=0`
- **Response**: Paginated list of configuration changes with timestamps and administrators

### Moderation Tools

#### GET /admin/reports

- **Purpose**: List player reports and moderation queue
- **Query**: `?status=pending&type=harassment&limit=20`
- **Response**: List of reports with details and priority levels

#### POST /admin/reports/{report_id}/action

- **Purpose**: Take moderation action on a report
- **Body**:
  `{ "action": "warn_user", "target_user": "uuid", "message": "Warning message", "notes": "Admin notes" }`
- **Response**: Action confirmation with updated report status

#### GET /admin/chat/logs

- **Purpose**: Search chat logs for moderation
- **Query**: `?user_id=uuid&channel=global&search=keyword&from=2025-07-20&to=2025-07-21`
- **Response**: Filtered chat messages with context

### Analytics & Reporting

#### GET /admin/analytics/overview

- **Purpose**: Get high-level game analytics dashboard
- **Response**:

```json
{
  "active_users": {
    "daily": 1250,
    "weekly": 3500,
    "monthly": 8900
  },
  "revenue": {
    "daily": 2500.0,
    "monthly": 75000.0
  },
  "game_metrics": {
    "avg_session_duration": "45m",
    "retention_rate_d7": 0.65,
    "conversion_rate": 0.12
  }
}
```

#### GET /admin/analytics/users

- **Purpose**: Get detailed user analytics
- **Query**: `?metric=retention&period=7d&segment=new_users`
- **Response**: User behavior analytics with segmentation

#### GET /admin/analytics/economy

- **Purpose**: Get game economy analytics
- **Response**: Resource flow, inflation metrics, and economic balance indicators

---

## /health/ — System Observability & Health Monitoring

### Rationale

Comprehensive system monitoring is essential for maintaining game stability, performance, and
reliability. These endpoints provide detailed health information, performance metrics, and
operational visibility for both automated monitoring systems and manual inspection.

### Basic Health Checks

#### GET /health

- **Purpose**: Basic health check endpoint
- **Response**: `{ "status": "healthy", "timestamp": "2025-07-20T16:30:00Z" }`
- **Rationale**: Simple endpoint for load balancers and basic monitoring

#### GET /health/ready

- **Purpose**: Readiness probe for container orchestration
- **Response**:
  `{ "ready": true, "services": { "database": "ready", "redis": "ready", "queue": "ready" } }`

#### GET /health/live

- **Purpose**: Liveness probe for container orchestration
- **Response**: `{ "alive": true, "uptime": "72h15m30s" }`

### Performance Monitoring

#### GET /health/services

- **Purpose**: Status of all dependent services
- **Response**:

```json
{
  "services": [
    {
      "name": "postgresql",
      "status": "healthy",
      "response_time": 5,
      "last_check": "2025-07-20T16:30:00Z"
    },
    {
      "name": "job_queue",
      "status": "degraded",
      "response_time": 150,
      "last_check": "2025-07-20T16:30:00Z",
      "issues": ["High queue depth"]
    }
  ]
}
```

### Detailed System Metrics

#### GET /health/metrics

- **Purpose**: Comprehensive system metrics
- **Response**:

```json
{
  "system": {
    "cpu_usage": 45.2,
    "memory_usage": 68.5,
    "disk_usage": 32.1,
    "load_average": [1.2, 1.5, 1.8]
  },
  "application": {
    "active_connections": 1250,
    "requests_per_second": 150,
    "average_response_time": 85,
    "error_rate": 0.02
  },
  "database": {
    "connection_pool_usage": 75,
    "query_performance": {
      "avg_query_time": 12,
      "slow_queries": 3
    },
    "replication_lag": 0
  },
  "cache": {
    "hit_rate": 0.92,
    "memory_usage": 45.8,
    "evictions_per_second": 2.1
  }
}
```

---

## /liveops/ — LiveOps Dashboard & Real-time Analytics

### Rationale

LiveOps (Live Operations) is crucial for maintaining player engagement, optimizing game economy, and
making data-driven decisions. These endpoints provide real-time analytics, player behavior insights,
A/B testing capabilities, and tools for live content management essential for modern game
operations.

### Real-time Game Analytics

#### GET /liveops/analytics/realtime

- **Purpose**: Real-time game metrics dashboard
- **Response**:

```json
{
  "current_players": 1250,
  "players_by_region": {
    "NA": 450,
    "EU": 380,
    "ASIA": 320,
    "OTHER": 100
  },
  "active_sessions": {
    "total": 1250,
    "avg_duration": "42m",
    "new_sessions_last_hour": 180
  },
  "revenue": {
    "last_hour": 125.5,
    "today": 2850.75,
    "trend": "+12%"
  },
  "game_events": {
    "buildings_constructed": 45,
    "battles_fought": 23,
    "trades_completed": 67,
    "research_completed": 34
  }
}
```

#### GET /liveops/analytics/kpis

- **Purpose**: Key Performance Indicators tracking
- **Query**: `?period=24h&kpi=retention,revenue,engagement`
- **Response**:

```json
{
  "kpis": {
    "retention": {
      "d1": 0.75,
      "d7": 0.45,
      "d30": 0.28,
      "trend": "+5%"
    },
    "revenue": {
      "arpu": 12.5,
      "arppu": 45.8,
      "conversion_rate": 0.15,
      "trend": "+8%"
    },
    "engagement": {
      "dau": 8500,
      "session_length": "38m",
      "sessions_per_user": 2.3,
      "trend": "+3%"
    }
  }
}
```

### Player Behavior Analytics

#### GET /liveops/players/segments

- **Purpose**: Player segmentation analysis
- **Response**:

```json
{
  "segments": [
    {
      "name": "High Spenders",
      "count": 150,
      "percentage": 1.8,
      "characteristics": {
        "avg_revenue": 125.5,
        "avg_level": 25,
        "avg_session_time": "65m"
      }
    },
    {
      "name": "Casual Players",
      "count": 6800,
      "percentage": 80.0,
      "characteristics": {
        "avg_revenue": 2.5,
        "avg_level": 8,
        "avg_session_time": "25m"
      }
    }
  ]
}
```

#### GET /liveops/players/behavior

- **Purpose**: Player behavior patterns and trends
- **Query**: `?metric=progression&period=7d&segment=new_players`
- **Response**: Behavioral analytics with progression funnels, drop-off points, and engagement
  patterns

#### GET /liveops/players/churn

- **Purpose**: Churn analysis and prediction
- **Response**: Churn risk scores, at-risk player identification, and retention recommendations

### Game Economy Analytics

#### GET /liveops/economy/overview

- **Purpose**: Game economy health dashboard
- **Response**:

```json
{
  "resource_flow": {
    "gold": {
      "generated": 1250000,
      "consumed": 1180000,
      "net_flow": 70000,
      "inflation_rate": 0.02
    },
    "food": {
      "generated": 2500000,
      "consumed": 2450000,
      "net_flow": 50000,
      "inflation_rate": 0.01
    }
  },
  "market_activity": {
    "trades_per_hour": 45,
    "avg_trade_value": 1250,
    "price_volatility": 0.15
  },
  "balance_indicators": {
    "resource_scarcity": "balanced",
    "progression_rate": "optimal",
    "economy_health": "healthy"
  }
}
```

#### GET /liveops/economy/balance

- **Purpose**: Economic balance analysis
- **Query**: `?resource=gold&period=7d`
- **Response**: Resource balance metrics, distribution analysis, and balance recommendations

### A/B Testing & Experiments

#### GET /liveops/experiments

- **Purpose**: List active and completed A/B tests
- **Response**:

```json
{
  "active": [
    {
      "id": "exp_001",
      "name": "Resource Generation Rate Test",
      "status": "running",
      "start_date": "2025-07-15T00:00:00Z",
      "participants": 1000,
      "variants": {
        "control": 500,
        "variant_a": 500
      },
      "metrics": {
        "retention_d7": {
          "control": 0.45,
          "variant_a": 0.52,
          "significance": 0.95
        }
      }
    }
  ]
}
```

#### POST /liveops/experiments

- **Purpose**: Create new A/B test
- **Body**:
  `{ "name": "string", "description": "string", "variants": [], "target_audience": {}, "metrics": [] }`
- **Response**: Created experiment configuration

#### PUT /liveops/experiments/{experiment_id}/status

- **Purpose**: Control experiment status (start, pause, stop)
- **Body**: `{ "status": "paused", "reason": "Unexpected results" }`
- **Response**: Updated experiment status

### Live Events & Content Management

#### GET /liveops/events

- **Purpose**: List active and scheduled live events
- **Response**:

```json
{
  "active": [
    {
      "id": "event_001",
      "name": "Summer Festival",
      "type": "seasonal",
      "start_time": "2025-07-20T00:00:00Z",
      "end_time": "2025-07-27T23:59:59Z",
      "participants": 3500,
      "rewards_claimed": 1250,
      "engagement_rate": 0.78
    }
  ],
  "scheduled": [
    {
      "id": "event_002",
      "name": "Harvest Moon",
      "type": "resource_boost",
      "start_time": "2025-08-01T00:00:00Z",
      "end_time": "2025-08-03T23:59:59Z"
    }
  ]
}
```

#### POST /liveops/events

- **Purpose**: Create new live event
- **Body**:
  `{ "name": "string", "type": "string", "start_time": "datetime", "end_time": "datetime", "config": {} }`
- **Response**: Created event configuration

#### PUT /liveops/events/{event_id}

- **Purpose**: Update live event configuration
- **Body**: Event configuration updates
- **Response**: Updated event details

### Performance & Technical Metrics

#### GET /liveops/performance/game

- **Purpose**: Game-specific performance metrics
- **Response**:

```json
{
  "server_performance": {
    "avg_response_time": 85,
    "requests_per_second": 150,
    "error_rate": 0.02,
    "concurrent_users": 1250
  },
  "game_metrics": {
    "actions_per_second": 45,
    "database_queries_per_second": 320,
    "cache_hit_rate": 0.92,
    "websocket_connections": 800
  },
  "regional_performance": {
    "NA": {
      "latency": 45,
      "error_rate": 0.01
    },
    "EU": {
      "latency": 52,
      "error_rate": 0.02
    },
    "ASIA": {
      "latency": 78,
      "error_rate": 0.03
    }
  }
}
```

#### GET /liveops/alerts

- **Purpose**: LiveOps alerts and notifications
- **Response**: Critical alerts about player behavior anomalies, revenue drops, or technical issues

---

## /ws/ — Real-time Features & Chat

### Rationale

Real-time updates enhance user experience by providing immediate feedback on game state changes.
WebSocket connections enable live communication and instant notifications for critical game events.

### WebSocket Endpoints

#### WS /ws/game

- **Purpose**: Real-time game updates
- **Events**:
  - `resource_update`: Resource changes and production updates
  - `building_complete`: Building upgrades finished
  - `research_complete`: Research finished
  - `attack_incoming`: Combat alerts and warnings
  - `trade_completed`: Market transactions
  - `modifier_applied`: New modifiers activated
  - `modifier_expired`: Temporary modifiers ended
  - `alliance_request`: Diplomatic proposals received
  - `guild_message`: Guild announcements

#### WS /ws/chat

- **Purpose**: Real-time chat system
- **Events**:
  - `message_received`: New chat message
  - `user_joined`: Player joined chat channel
  - `user_left`: Player left chat channel
  - `typing_indicator`: Player is typing
- **Channels**:
  - `global`: Server-wide chat
  - `guild`: Guild-specific chat
  - `alliance`: Alliance member chat
  - `private`: Direct messages between players

### Chat REST Endpoints

#### GET /ws/chat/channels

- **Purpose**: Get available chat channels for player
- **Response**: List of accessible channels with unread counts

#### GET /ws/chat/history/{channel}

- **Purpose**: Get chat message history
- **Query**: `?limit=50&before=message_id`
- **Response**: Paginated chat messages

#### POST /ws/chat/send

- **Purpose**: Send chat message
- **Body**: `{ "channel": "global", "message": "Hello world!", "type": "text" }`
- **Response**: Message confirmation with timestamp

#### POST /ws/chat/typing

- **Purpose**: Send typing indicator
- **Body**: `{ "channel": "global", "typing": true }`
- **Response**: Typing status confirmation

---

## Error Handling

### Standard Error Response Format

```json
{
  "error": {
    "code": "INSUFFICIENT_RESOURCES",
    "message": "Not enough wood to upgrade building",
    "details": {
      "required": {
        "wood": 1000
      },
      "available": {
        "wood": 750
      }
    }
  }
}
```

### Common Error Codes

- `AUTHENTICATION_REQUIRED`: Missing or invalid token
- `INSUFFICIENT_RESOURCES`: Not enough resources for action
- `BUILDING_LIMIT_REACHED`: Cannot construct more buildings of this type
- `RESEARCH_IN_PROGRESS`: Cannot start new research while another is active
- `INVALID_FACTION`: Faction not available or invalid
- `COOLDOWN_ACTIVE`: Action is on cooldown

## Rate Limiting

### Rationale

Prevent abuse and ensure fair gameplay while maintaining responsive user experience.

### Limits

- Authentication endpoints: 5 requests per minute
- Resource collection: 1 request per 30 seconds
- Building actions: 10 requests per minute
- General API: 100 requests per minute per user

## API Versioning

### Rationale

Support multiple client versions and enable gradual feature rollouts.

### Strategy

- URL versioning: `/api/v1/`
- Header versioning: `Accept: application/vnd.empire.v1+json`
- Backward compatibility for at least 2 major versions

## Security Considerations

### Authentication

- JWT tokens with 24-hour expiration
- Refresh token mechanism for seamless re-authentication
- Rate limiting on authentication endpoints

### Authorization

- Role-based access control (player, admin, moderator)
- Resource ownership validation
- Action permission checks

### Data Validation

- Input sanitization for all endpoints
- Resource limit validation
- Game rule enforcement

## Performance Optimization

### Caching Strategy

- Redis caching for frequently accessed data
- Player resource status cached for 30 seconds
- Building information cached until updates
- Faction data cached indefinitely

### Database Optimization

- Indexed queries for player lookups
- Efficient resource calculation queries
- Batch operations for bulk updates

## Mobile Client Considerations

### Offline Support

- Cache critical game data locally
- Queue actions when offline
- Sync when connection restored

### Push Notifications

- Building completion alerts
- Attack warnings
- Resource capacity warnings
- Guild messages

### Bandwidth Optimization

- Compressed JSON responses
- Delta updates for resource changes
- Image optimization for building/unit graphics

## Implementation Priority

### Phase 1: Core Systems

1. Authentication & player management
2. Resource system with basic production
3. Building system with upgrades
4. Basic dashboard and notifications

### Phase 2: Game Mechanics

1. Modifier system implementation
2. Research trees
3. Military units and training
4. Faction-specific features

### Phase 3: Social Features

1. Guild system
2. Trading market
3. Diplomacy and alliances
4. Religion system

### Phase 4: Advanced Features

1. Real-time WebSocket updates
2. Advanced combat mechanics
3. Events and seasonal content
4. Analytics and reporting

This API design provides a comprehensive foundation for the Empire game, supporting both current
features and future expansion while maintaining consistency, security, and performance across web
and mobile platforms.
