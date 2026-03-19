# Physics

Add physical simulation to your game with rigid bodies, colliders, and forces.

## Physics Backend

Renzora uses **Avian3D** (a Rust-native physics engine) for simulation. It runs automatically — just add physics components to your entities.

## Setting Up Physics

### 1. Add a Rigid Body

Select an entity → Inspector → **Add Component → Physics Body**

| Body Type | Description |
|-----------|-------------|
| **Dynamic** | Affected by gravity and forces. Has mass. Falls, bounces, gets pushed. |
| **Static** | Never moves. Infinite mass. Floors, walls, platforms. |
| **Kinematic** | Code-controlled. Pushes dynamic bodies but ignores forces. Moving platforms, elevators. |

### 2. Add a Collider

Select the same entity → Inspector → **Add Component → Collision Shape**

| Shape | Best For |
|-------|----------|
| **Box** | Crates, walls, platforms |
| **Sphere** | Balls, projectiles, simple characters |
| **Capsule** | Characters, humanoid entities |
| **Cylinder** | Columns, barrels |

### 3. Configure Properties

**Rigid Body:**
- **Mass** — heavier objects push lighter ones (default: 1.0 kg)
- **Gravity Scale** — 0 = floats, 1 = normal, 2 = double gravity
- **Damping** — air resistance (higher = slower movement over time)
- **Axis Locks** — prevent rotation or movement on specific axes

**Collider:**
- **Friction** — 0.0 (ice) to 1.0 (rubber)
- **Restitution** — 0.0 (no bounce) to 1.0 (full bounce)
- **Is Sensor** — pass-through trigger (detects overlap without blocking)

## Scripting Physics

### Applying Forces

```rhai
fn on_update() {
    // Continuous force (like a jet engine)
    if is_key_pressed("Space") {
        apply_force(0.0, 50.0, 0.0);  // push up
    }

    // Instant kick (like an explosion)
    if is_key_just_pressed("E") {
        apply_impulse(0.0, 10.0, 0.0);  // jump
    }

    // Rotational force
    apply_torque(0.0, 5.0, 0.0);  // spin
}
```

### Setting Velocity

```rhai
fn on_update() {
    // Direct velocity control (overrides physics)
    set_velocity(input_x * 5.0, 0.0, input_y * 5.0);
}
```

### Collision Detection

```rhai
fn on_update() {
    // Triggered the frame a collision starts
    if collisions_entered > 0 {
        print("Hit something!");
    }

    // Triggered the frame a collision ends
    if collisions_exited > 0 {
        print("No longer touching");
    }

    // Number of things currently overlapping
    if active_collisions > 0 {
        print("Still in contact");
    }
}
```

### Raycasting

Cast an invisible line to detect what's in a direction:

```rhai
fn on_update() {
    // Raycast forward from entity position
    raycast(
        position_x, position_y, position_z,  // origin
        0.0, 0.0, -1.0,                       // direction
        100.0,                                  // max distance
        "forward_ray"                           // result name
    );

    // Ground check (raycast straight down)
    raycast_down(position_x, position_y, position_z, 1.5, "ground_check");
}
```

### Cross-Entity Physics

Apply forces to other entities:

```rhai
fn on_update() {
    if is_key_just_pressed("F") {
        // Push an entity away from us
        let target = find_entity_by_name("Box");
        apply_impulse_to(target, 0.0, 5.0, 10.0);
    }
}
```

## Tips

- **Characters**: Use a Capsule collider with locked rotation (X and Z) so the character doesn't topple over
- **Projectiles**: Use a Sphere collider with high velocity and low mass
- **Triggers**: Set `Is Sensor = true` for areas that detect entry without blocking (trigger zones, pickups)
- **Performance**: Static colliders are cheapest. Use them for all non-moving scenery
- **Ground Detection**: Use `raycast_down` to check if a character is on the ground before allowing jumps
