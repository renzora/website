# Scripting Overview

Add game logic to your entities with Rhai, Lua, or visual blueprints.

## Three ways to script

Renzora supports three scripting approaches. You can mix and match them in the same project:

- **[Rhai](/docs/scripting/rhai)** — a lightweight scripting language designed for Rust. Simple syntax, great for gameplay logic.
- **[Lua](/docs/scripting/lua)** — the industry-standard game scripting language. Use if you're coming from other engines.
- **[Visual Blueprints](/docs/scripting/blueprints)** — node-based visual scripting. No coding required.

## Script lifecycle

Every script has two key functions:

- `on_ready()` — called once when the entity spawns
- `on_update()` — called every frame

## Attaching scripts

1. Create a script file in your project's `scripts/` folder
2. Select an entity in the editor
3. In the Inspector, click "Add Component" → Script
4. Choose your script file

## Built-in variables

Every frame, your script has access to these globals:

### Time

```
delta           Seconds since last frame (use for smooth movement)
elapsed         Total seconds since game started
```

### Transform

```
position_x      Entity X position (read/write)
position_y      Entity Y position
position_z      Entity Z position
rotation_x/y/z  Entity rotation
scale_x/y/z     Entity scale
```

### Input

```
input_x         Horizontal axis (-1 to 1, from A/D keys)
input_y         Vertical axis (-1 to 1, from W/S keys)
mouse_x/y       Mouse position
mouse_delta_x/y Mouse movement since last frame
mouse_button_left/right/middle  Mouse button state
```

### Gamepad

```
gamepad_left_x/y     Left stick
gamepad_right_x/y    Right stick
gamepad_south/east/north/west  Face buttons
gamepad_left_trigger/right_trigger  Triggers
```

## What's next?

Pick your language: [Rhai](/docs/scripting/rhai), [Lua](/docs/scripting/lua), or [Blueprints](/docs/scripting/blueprints).
