# Creating Components

Define custom data types that can be attached to entities.

## What is a Component?

A component is a Rust struct that implements `Component`. It holds data — position, health, inventory, whatever your game needs. Components alone don't do anything; **systems** read and write component data.

## Basic Component

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<String>,
    pub max_slots: usize,
}
```

Attach it to an entity:
```rust
commands.spawn((
    Transform::default(),
    Inventory {
        items: vec![],
        max_slots: 20,
    },
));
```

## Serializable Components

For components to survive scene save/load, they need `Reflect`, `Serialize`, and `Deserialize`:

```rust
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component, Reflect, Serialize, Deserialize, Default, Clone)]
#[reflect(Component, Serialize, Deserialize)]
pub struct DayNightCycle {
    pub time_of_day: f32,    // 0.0 = midnight, 0.5 = noon
    pub cycle_speed: f32,     // hours per real second
    pub sun_color: [f32; 3],
}
```

Register it in your plugin:
```rust
app.register_type::<DayNightCycle>();
```

## Marker Components

Zero-size components that tag entities:

```rust
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Collectible;
```

Use them in queries to filter:
```rust
fn damage_enemies(
    mut enemies: Query<&mut Health, (With<Enemy>, Without<Player>)>,
) {
    for mut health in &mut enemies {
        health.current -= 10.0;
    }
}
```

## Resource vs Component

| | Component | Resource |
|-|-----------|----------|
| **Attached to** | Entity | Global (one instance) |
| **Access** | `Query<&MyComponent>` | `Res<MyResource>` |
| **Use for** | Per-entity data | Global state, config |

```rust
// Resource — global game state
#[derive(Resource, Default)]
pub struct GameState {
    pub score: u32,
    pub level: u32,
}

// Component — per-entity data
#[derive(Component)]
pub struct ScoreValue(pub u32);
```

## Events

For one-shot communication between systems:

```rust
#[derive(Event)]
pub struct PlayerDied {
    pub player_entity: Entity,
}

// Send event
fn check_death(
    query: Query<(Entity, &Health), With<Player>>,
    mut events: EventWriter<PlayerDied>,
) {
    for (entity, health) in &query {
        if health.current <= 0.0 {
            events.send(PlayerDied { player_entity: entity });
        }
    }
}

// Receive event
fn handle_death(mut events: EventReader<PlayerDied>) {
    for event in events.read() {
        println!("Player {:?} died!", event.player_entity);
    }
}
```

Register the event:
```rust
app.add_event::<PlayerDied>();
```
