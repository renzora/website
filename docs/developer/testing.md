# Testing

How to write and run tests for Renzora Engine.

## Running tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test --package renzora_physics

# Specific test
cargo test test_rigid_body_spawn

# With output
cargo test -- --nocapture
```

## Unit tests

Standard Rust unit tests in the same file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_clamp() {
        let mut health = Health { current: 150.0, max: 100.0 };
        health.current = health.current.min(health.max);
        assert_eq!(health.current, 100.0);
    }

    #[test]
    fn test_damage_calculation() {
        let base = 50.0;
        let armor = 20.0;
        let result = calculate_damage(base, armor);
        assert!((result - 30.0).abs() < f32::EPSILON);
    }
}
```

## Testing systems

Bevy systems can be tested with a real `World`:

```rust
#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    #[test]
    fn test_health_regen_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, regenerate_health);

        // Spawn a test entity
        let entity = app.world_mut().spawn((
            Health { current: 50.0, max: 100.0 },
            Player,
        )).id();

        // Run one frame
        app.update();

        // Check the result
        let health = app.world().get::<Health>(entity).unwrap();
        assert!(health.current > 50.0, "Health should have regenerated");
    }
}
```

### Verify a function is a valid system

```rust
use bevy::ecs::system::assert_is_system;

#[test]
fn system_signatures_are_valid() {
    assert_is_system(regenerate_health);
    assert_is_system(process_input);
    assert_is_system(sync_physics);
}
```

## Testing with World directly

For lower-level tests without a full `App`:

```rust
#[test]
fn test_entity_queries() {
    let mut world = World::new();

    world.spawn((Health { current: 100.0, max: 100.0 }, Player));
    world.spawn((Health { current: 50.0, max: 50.0 }, Enemy));
    world.spawn((Health { current: 75.0, max: 75.0 }, Enemy));

    let mut query = world.query_filtered::<&Health, With<Enemy>>();
    let enemies: Vec<&Health> = query.iter(&world).collect();

    assert_eq!(enemies.len(), 2);
}
```

## Testing scripts

Use the `ScriptTestHarness` for script integration tests:

```rust
use renzora_scripting::test::ScriptTestHarness;

#[test]
fn test_rhai_movement_script() {
    let mut harness = ScriptTestHarness::new();

    // Set initial state
    harness.set_var("position_x", 0.0);
    harness.set_var("input_x", 1.0);
    harness.set_var("delta", 0.016);

    // Run a script
    harness.eval_rhai("position_x += input_x * 5.0 * delta;");

    // Check result
    let pos_x: f64 = harness.get_var("position_x");
    assert!((pos_x - 0.08).abs() < 0.001);
}

#[test]
fn test_lua_script() {
    let mut harness = ScriptTestHarness::new();
    harness.set_var("position_x", 0.0);

    harness.eval_lua("position_x = position_x + 10.0");

    let pos_x: f64 = harness.get_var("position_x");
    assert_eq!(pos_x, 10.0);
}
```

## Testing editor panels

Headless rendering tests for editor UI:

```rust
#[test]
fn test_debug_panel_renders() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, EguiPlugin));
    app.register_panel::<DebugStatsPanel>();

    // Run without crashing
    app.update();
    app.update();
}
```

## Integration tests

Place in `tests/` directory for cross-crate testing:

```
crates/core/renzora_physics/tests/
└── integration_test.rs
```

```rust
use renzora_physics::*;
use bevy::prelude::*;

#[test]
fn test_physics_plugin_integration() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, PhysicsPlugin));

    let entity = app.world_mut().spawn((
        Transform::default(),
        RigidBody::Dynamic,
        Collider::sphere(1.0),
    )).id();

    // Simulate several frames
    for _ in 0..60 {
        app.update();
    }

    let transform = app.world().get::<Transform>(entity).unwrap();
    assert!(transform.translation.y < 0.0, "Entity should have fallen due to gravity");
}
```

## Benchmarks

Use [criterion](https://github.com/bheisler/criterion.rs) for performance benchmarks:

```rust
// benches/physics_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_raycast(c: &mut Criterion) {
    c.bench_function("raycast_1000_entities", |b| {
        let world = setup_world_with_entities(1000);
        b.iter(|| {
            perform_raycast(&world, Vec3::ZERO, Vec3::Z, 100.0);
        });
    });
}

criterion_group!(benches, bench_raycast);
criterion_main!(benches);
```

Run:
```bash
cargo bench --package renzora_physics
```

## CI pipeline

The CI runs on every PR:

```bash
cargo fmt --all -- --check          # Formatting
cargo clippy --all-targets -- -D warnings  # Lints
cargo test --workspace              # All tests
cargo doc --no-deps                 # Documentation builds
```

All checks must pass before merge.

## Coverage

Generate coverage reports with `cargo-llvm-cov`:

```bash
cargo install cargo-llvm-cov
cargo llvm-cov --workspace --html
# Report at target/llvm-cov/html/index.html
```

There is no hard coverage requirement, but new features should include tests for critical paths.
