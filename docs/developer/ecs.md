# ECS & Bevy

Understand the Entity Component System that powers Renzora Engine.

## Why ECS?

Renzora is built on [Bevy 0.18](https://bevyengine.org/), a data-driven game engine. Instead of inheritance hierarchies (like Unity's MonoBehaviour or Godot's Node), Bevy uses **composition**:

- **Entities** — unique IDs. Just a number. Hold nothing by themselves.
- **Components** — data structs attached to entities. Define what an entity *is*.
- **Systems** — functions that query entities by their components. Define what *happens*.
- **Resources** — global singletons, not attached to any entity.

This model gives better performance (cache-friendly data layout), easier composition, and natural parallelism.

## Entities

An entity is a unique `Entity` ID. Create entities with `Commands`:

```rust
fn spawn_player(mut commands: Commands) {
    let entity = commands.spawn((
        Transform::default(),
        Player,
        Health { current: 100.0, max: 100.0 },
    )).id();
}
```

Despawn:
```rust
fn cleanup(mut commands: Commands, query: Query<Entity, With<Dead>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
```

## Components

Any Rust struct can be a component:

```rust
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Player;  // marker component — zero size, used for filtering
```

Add/remove at runtime:
```rust
commands.entity(entity).insert(Poisoned { duration: 5.0 });
commands.entity(entity).remove::<Poisoned>();
```

## Systems

Systems are plain functions. Bevy calls them based on their parameter types:

```rust
fn regenerate_health(
    time: Res<Time>,
    mut query: Query<&mut Health, With<Player>>,
) {
    for mut health in &mut query {
        health.current = (health.current + 5.0 * time.delta_secs()).min(health.max);
    }
}
```

Register in a plugin:
```rust
app.add_systems(Update, regenerate_health);
```

## Queries

Queries select entities by component combination:

```rust
// All entities with Transform and Health
Query<(&Transform, &Health)>

// Mutable access to Health, only entities with Player tag
Query<&mut Health, With<Player>>

// Entities with Health but NOT Player
Query<&Health, Without<Player>>

// Only entities whose Health changed this frame
Query<&Health, Changed<Health>>

// Only entities that just got a Health component
Query<&Health, Added<Health>>

// Get a specific entity's components
query.get(entity)  // returns Result
```

## Resources

Global state accessible by any system:

```rust
#[derive(Resource, Default)]
pub struct GameScore {
    pub points: u32,
}

// Read-only access
fn display_score(score: Res<GameScore>) {
    println!("Score: {}", score.points);
}

// Mutable access
fn add_points(mut score: ResMut<GameScore>) {
    score.points += 10;
}
```

Initialize:
```rust
app.init_resource::<GameScore>();         // uses Default
app.insert_resource(GameScore { points: 0 }); // explicit value
```

## Events

One-shot messages between systems:

```rust
#[derive(Event)]
pub struct EnemyDied { pub entity: Entity, pub xp_value: u32 }

// Send
fn check_death(query: Query<(Entity, &Health, &XpValue)>, mut events: EventWriter<EnemyDied>) {
    for (entity, health, xp) in &query {
        if health.current <= 0.0 {
            events.send(EnemyDied { entity, xp_value: xp.0 });
        }
    }
}

// Receive
fn award_xp(mut events: EventReader<EnemyDied>, mut score: ResMut<GameScore>) {
    for event in events.read() {
        score.points += event.xp_value;
    }
}
```

Register:
```rust
app.add_event::<EnemyDied>();
```

## System scheduling

Bevy runs systems in parallel when possible. Control ordering:

```rust
app.add_systems(Update, (
    read_input,
    move_player.after(read_input),
    update_camera.after(move_player),
));

// Or chain them
app.add_systems(Update, (read_input, move_player, update_camera).chain());
```

### Schedule labels

| Schedule | When it runs |
|----------|-------------|
| `Startup` | Once, at app launch |
| `PreUpdate` | Before Update, every frame |
| `Update` | Main game logic, every frame |
| `PostUpdate` | After Update (transform propagation, rendering prep) |
| `FixedUpdate` | Fixed timestep (default 60 Hz) — use for physics |

## How Renzora layers on Bevy

Renzora adds its own plugins on top of Bevy's core:

- **`RenzoraEditorPlugin`** — egui-based editor panels, docking, gizmos
- **`RenzoraScriptPlugin`** — Rhai/Lua scripting integration
- **`RenzoraPhysicsPlugin`** — Avian3D wrapper with script bindings
- **`RenzoraAudioPlugin`** — spatial audio with script control
- **`RenzoraNetPlugin`** — client-server networking and replication
- **`RenzoraUIPlugin`** — game UI system (bevy_ui) with script bindings

Each plugin follows Bevy's `Plugin` trait — add systems, register components, initialize resources.
