# Writing Systems

Systems contain all the logic in a Bevy/Renzora application.

## System function signatures

A system is any function whose parameters implement `SystemParam`:

```rust
fn my_system(
    time: Res<Time>,                          // read global resource
    mut score: ResMut<GameScore>,             // write global resource
    query: Query<(&Transform, &Health)>,      // read entity data
    mut commands: Commands,                    // spawn/despawn/modify entities
    mut events: EventWriter<MyEvent>,         // send events
    incoming: EventReader<OtherEvent>,        // receive events
) {
    // ...
}
```

Register with:
```rust
app.add_systems(Update, my_system);
```

## Common query patterns

```rust
// Iterate all matching entities
for (transform, health) in &query { ... }

// Mutable iteration
for mut health in &mut query { health.current -= 1.0; }

// Get a specific entity
if let Ok((transform, health)) = query.get(entity) { ... }

// Single entity (panics if 0 or 2+ match)
let (transform, health) = query.single();

// Optional single
if let Ok((transform, health)) = query.get_single() { ... }

// Combine filters
Query<&mut Health, (With<Enemy>, Without<Boss>, Changed<Health>)>
```

## System ordering

```rust
// Explicit ordering
app.add_systems(Update, (
    gather_input,
    process_movement.after(gather_input),
    apply_damage.after(process_movement),
));

// Chain shorthand (runs in order)
app.add_systems(Update, (
    gather_input,
    process_movement,
    apply_damage,
).chain());

// Before
app.add_systems(Update, cleanup.before(spawn_enemies));
```

## System sets

Group systems for shared ordering and conditions:

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Logic,
    Render,
}

app.configure_sets(Update, (
    GameSet::Input,
    GameSet::Logic.after(GameSet::Input),
    GameSet::Render.after(GameSet::Logic),
));

app.add_systems(Update, gather_input.in_set(GameSet::Input));
app.add_systems(Update, move_player.in_set(GameSet::Logic));
```

## Run conditions

Only run a system when a condition is true:

```rust
app.add_systems(Update, pause_menu.run_if(resource_equals(GameState::Paused)));
app.add_systems(Update, game_logic.run_if(not(resource_equals(GameState::Paused))));

// Custom condition
fn is_playing(state: Res<GameState>) -> bool {
    *state == GameState::Playing
}
app.add_systems(Update, game_logic.run_if(is_playing));
```

## Fixed timestep

Use `FixedUpdate` for physics and deterministic logic:

```rust
app.add_systems(FixedUpdate, physics_step);
app.insert_resource(Time::<Fixed>::from_hz(60.0)); // 60 ticks per second
```

`FixedUpdate` runs at a constant rate regardless of frame rate. Use `time.delta_secs()` inside — it's always `1/60` (or whatever your tick rate is).

## One-shot systems

Run a system once, on demand:

```rust
let system_id = app.register_system(|mut commands: Commands| {
    commands.spawn(EnemyBundle::default());
});

// Later, trigger it
commands.run_system(system_id);
```

## Exclusive systems

When you need full mutable World access (rare):

```rust
fn exclusive_system(world: &mut World) {
    // Direct world access — no other systems run in parallel
    let mut query = world.query::<&mut Health>();
    for mut health in query.iter_mut(world) {
        health.current = health.max;
    }
}
```

## Example: health regeneration

```rust
fn regenerate_health(
    time: Res<Time>,
    mut query: Query<&mut Health, (With<Player>, Without<Dead>)>,
) {
    for mut health in &mut query {
        if health.current < health.max {
            health.current += 2.0 * time.delta_secs();
            health.current = health.current.min(health.max);
        }
    }
}
```

## Example: despawn after timer

```rust
#[derive(Component)]
struct DespawnTimer(Timer);

fn tick_despawn_timers(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnTimer)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
```

## Performance tips

- **Use `Changed<T>`** to skip entities whose data hasn't changed
- **Use `With<T>` / `Without<T>`** instead of checking Option in the loop
- **Avoid `Query::iter()` inside another `Query::iter()`** — O(n²). Use events or resources to communicate
- **Prefer `FixedUpdate`** for physics to avoid frame-rate-dependent behavior
- Systems run in parallel by default — Bevy handles it. Don't add manual threading
