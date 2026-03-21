# Camera System

Configure and control cameras in the editor and at runtime.

## Camera components

Cameras are entities with camera components:

```rust
// 3D perspective camera
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
));

// 2D orthographic camera
commands.spawn((
    Camera2d::default(),
    OrthographicProjection { scale: 1.0, ..default() },
));
```

## Projection types

| Projection | Use case |
|------------|----------|
| **Perspective** | 3D games (default). Objects shrink with distance. |
| **Orthographic** | 2D games, isometric, strategy. No depth scaling. |

```rust
// Custom perspective
PerspectiveProjection {
    fov: 60.0_f32.to_radians(),
    near: 0.1,
    far: 1000.0,
    aspect_ratio: 16.0 / 9.0,
}

// Custom orthographic
OrthographicProjection {
    scale: 10.0,      // visible units
    near: -1000.0,
    far: 1000.0,
    ..default()
}
```

## Camera properties

| Property | Default | Description |
|----------|---------|-------------|
| **FOV** | 60° | Field of view (perspective only) |
| **Near plane** | 0.1 | Closest visible distance |
| **Far plane** | 1000.0 | Farthest visible distance |
| **Clear color** | Sky blue | Background color |
| **HDR** | true | High dynamic range rendering |
| **MSAA** | 4x | Multi-sample anti-aliasing |
| **Order** | 0 | Render order (for multiple cameras) |
| **Target** | Window | Render target (window or texture) |

## Editor camera

The editor uses an orbit camera controller:

| Control | Action |
|---------|--------|
| **Middle mouse drag** | Orbit around focus point |
| **Scroll wheel** | Zoom in/out |
| **Shift + middle drag** | Pan |
| **F** | Focus on selected entity |
| **Right click + WASD** | Fly mode |
| **Numpad** | Snap to axis views (1=front, 3=right, 7=top) |

## Runtime camera controllers

Renzora provides configurable runtime controllers:

### First-person

```rust
commands.spawn((
    Camera3d::default(),
    FirstPersonController {
        move_speed: 5.0,
        look_speed: 2.0,
        sprint_multiplier: 2.0,
        jump_force: 8.0,
    },
));
```

### Third-person

```rust
commands.spawn((
    Camera3d::default(),
    ThirdPersonController {
        target: player_entity,
        distance: 5.0,
        min_distance: 2.0,
        max_distance: 15.0,
        height_offset: 2.0,
        look_speed: 2.0,
    },
));
```

### Orbit

```rust
commands.spawn((
    Camera3d::default(),
    OrbitController {
        target: Vec3::ZERO,
        distance: 10.0,
        auto_rotate: false,
        auto_rotate_speed: 1.0,
    },
));
```

## Camera scripting API

```rhai
fn on_update() {
    // Set camera field of view
    set_camera_fov("main_camera", 90.0);

    // Move camera
    set_camera_position("main_camera", 0.0, 10.0, -5.0);

    // Look at a point
    camera_look_at("main_camera", target_x, target_y, target_z);

    // Screen-to-world ray (for mouse picking)
    let world_pos = camera_screen_to_world("main_camera", mouse_x, mouse_y);

    // World-to-screen position (for UI markers)
    let screen_pos = camera_world_to_screen("main_camera", enemy_x, enemy_y, enemy_z);
}
```

## Multiple cameras

Use multiple cameras for split-screen, minimaps, or render-to-texture:

```rust
// Main camera (renders first)
commands.spawn((
    Camera3d::default(),
    Camera { order: 0, ..default() },
));

// Minimap camera (renders second, to a texture)
commands.spawn((
    Camera3d::default(),
    Camera {
        order: 1,
        target: RenderTarget::Image(minimap_texture.clone()),
        ..default()
    },
    OrthographicProjection { scale: 100.0, ..default() },
    Transform::from_xyz(0.0, 50.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
));
```

### Split-screen

```rust
// Player 1 — left half
Camera { viewport: Some(Viewport { x: 0.0, y: 0.0, width: 0.5, height: 1.0 }), order: 0, .. }

// Player 2 — right half
Camera { viewport: Some(Viewport { x: 0.5, y: 0.0, width: 0.5, height: 1.0 }), order: 1, .. }
```

## Post-processing per camera

Attach post-processing effects to individual cameras:

```rust
// Only this camera gets bloom and vignette
commands.spawn((
    Camera3d::default(),
    Bloom { intensity: 0.3, threshold: 1.0, ..default() },
    Vignette { intensity: 0.4, ..default() },
));
```

The minimap camera can have different effects (or none) from the main camera.

## Camera shake

Built-in camera shake system:

```rust
// Trigger from code
commands.entity(camera).insert(CameraShake {
    intensity: 0.5,
    duration: 0.3,
    frequency: 15.0,
    decay: true,
});
```

From scripts:
```rhai
fn on_update() {
    if collisions_entered > 0 {
        camera_shake("main_camera", 0.3, 0.2);  // intensity, duration
    }
}
```
