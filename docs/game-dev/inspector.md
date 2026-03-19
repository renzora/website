# Inspector

View and edit the components attached to the selected entity.

## Using the Inspector

Select an entity in the Hierarchy or viewport. The Inspector panel (right side) shows all its components with editable fields.

## Common Components

### Transform
Every entity has a Transform with:
- **Position** — X, Y, Z coordinates in world space
- **Rotation** — degrees around each axis
- **Scale** — size multiplier per axis

Click and drag a number field to adjust, or click to type a precise value.

### Mesh
3D entities have a Mesh component defining their shape. You can change the mesh type or assign a custom model file (`.glb`, `.gltf`).

### Material / Mesh Color
Controls the visual appearance. Set a base color directly, or assign a material graph (`.material` file) for advanced effects. Drag a material from the Asset Browser onto the entity.

### Script
Attach a `.rhai` script file to run game logic on this entity. When a script defines a `props()` function, the properties appear as editable fields in the Inspector:

```rhai
fn props() {
    #{
        speed: #{ default: 5.0, min: 0.0, max: 100.0 },
        health: #{ default: 100.0 },
        is_enemy: #{ default: false }
    }
}
```

This creates:
- **speed** — slider from 0 to 100 (default 5)
- **health** — number field (default 100)
- **is_enemy** — checkbox (default false)

### Physics Body
Controls physics simulation:
- **Body Type** — Dynamic (affected by forces), Static (immovable), Kinematic (code-controlled)
- **Mass** — weight in kg
- **Gravity Scale** — 0 = weightless, 1 = normal, negative = reverse gravity
- **Damping** — linear and angular drag
- **Axis Locks** — lock rotation or translation on specific axes

### Collision Shape
Defines the physics collision boundary:
- **Shape** — Box, Sphere, Capsule, or Cylinder
- **Size** — dimensions of the shape
- **Offset** — position offset from the entity center
- **Friction** — 0 (ice) to 1 (rubber)
- **Restitution** — bounciness (0 = no bounce, 1 = full bounce)
- **Is Sensor** — trigger zone that detects overlap without blocking

### Audio Player
Sound source component:
- **Clip** — path to audio file
- **Volume** — 0 to 2
- **Pitch** — playback speed (0.1 to 4.0)
- **Looping** — repeat playback
- **Autoplay** — play on spawn
- **Spatial** — enable 3D positioning
- **Bus** — Master, SFX, Music, or Ambient

### Entity Tag
A unique identifier string for finding entities from scripts:

```rhai
find_entity_by_tag("player")
```

## Adding Components

Click **"Add Component"** at the bottom of the Inspector to see all available component types. You can search by name.

## Removing Components

Right-click a component header to remove it from the entity.
