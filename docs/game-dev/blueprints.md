# Visual Blueprints

Build game logic visually with a node-based graph — no coding required.

## Overview

Visual Blueprints let you create gameplay logic by connecting nodes in a graph. Each node performs an action (play a sound, apply a force, check a condition), and you wire them together to define behavior.

Blueprints have the same capabilities as Rhai and Lua scripts. Anything you can do in code, you can do with nodes.

## The Blueprint Editor

Open the Blueprint Editor from **Window → Blueprint Editor** or double-click a `.blueprint` file.

- **Left panel** — node library, organized by category
- **Center** — the graph canvas (pan with middle mouse, zoom with scroll)
- **Right panel** — properties of the selected node
- **Top bar** — search, undo/redo, compile status

### Creating a blueprint

1. Right-click in the Project panel → **New → Blueprint**
2. Name it (e.g., `player_controller.blueprint`)
3. Attach it to an entity: select entity → Inspector → Add Component → Blueprint → choose file

## Pin types

Nodes have two kinds of connections:

- **Execution pins** (white arrows) — control the order things happen. Flow goes left to right.
- **Data pins** (colored circles) — pass values between nodes. Color indicates type:

| Color | Type | Example |
|-------|------|---------|
| Green | Float | `5.0`, `delta` |
| Cyan | Int | `42`, `collisions_entered` |
| Red | Bool | `true`, `is_key_pressed` |
| Purple | String | `"hello"`, `self_entity_name` |
| Yellow | Vec3 | `(1.0, 2.0, 3.0)` |
| Blue | Entity | entity reference |
| Gray | Any | accepts any type |

## Node reference

### Event nodes

Entry points that start execution flow.

| Node | Fires when |
|------|-----------|
| **On Ready** | Entity spawns (once) |
| **On Update** | Every frame |
| **On Collision Enter** | Physics collision starts |
| **On Collision Exit** | Physics collision ends |
| **On Key Pressed** | Specific key pressed (configurable) |
| **On Key Released** | Specific key released |
| **On Timer** | After delay (configurable seconds) |
| **On RPC Received** | Network RPC arrives |

### Flow control

| Node | Description |
|------|-------------|
| **Branch** | If/else — routes execution based on a Bool input |
| **Sequence** | Runs multiple outputs in order (output 1, then 2, then 3…) |
| **For Loop** | Repeats execution N times, outputs current index |
| **While Loop** | Repeats while a condition is true |
| **Delay** | Pauses execution for N seconds |
| **Gate** | Open/close to allow or block execution flow |
| **Switch** | Routes execution based on an Int or String value |
| **Do Once** | Executes only the first time, ignores subsequent calls |
| **Flip Flop** | Alternates between two outputs each call |

### Math

| Node | Inputs | Output | Description |
|------|--------|--------|-------------|
| **Add** | A, B | Result | A + B |
| **Subtract** | A, B | Result | A - B |
| **Multiply** | A, B | Result | A * B |
| **Divide** | A, B | Result | A / B |
| **Clamp** | Value, Min, Max | Result | Constrain value to range |
| **Lerp** | A, B, T | Result | Linear interpolation |
| **Random Range** | Min, Max | Result | Random float in range |
| **Sin / Cos / Tan** | Angle | Result | Trigonometry (radians) |
| **Abs** | Value | Result | Absolute value |
| **Min / Max** | A, B | Result | Smaller / larger value |
| **Distance** | A (Vec3), B (Vec3) | Result | Distance between points |
| **Normalize** | Vec (Vec3) | Result | Unit vector |
| **Dot Product** | A (Vec3), B (Vec3) | Result | Dot product |
| **Cross Product** | A (Vec3), B (Vec3) | Result | Cross product |
| **Remap** | Value, InMin, InMax, OutMin, OutMax | Result | Remap range |
| **Floor / Ceil / Round** | Value | Result | Rounding |
| **Power** | Base, Exp | Result | Exponentiation |

### Transform

| Node | Description |
|------|-------------|
| **Get Position** | Outputs entity's Vec3 position |
| **Set Position** | Sets entity's position from Vec3 |
| **Get Rotation** | Outputs entity's Vec3 rotation (degrees) |
| **Set Rotation** | Sets entity's rotation |
| **Translate** | Moves entity by Vec3 offset |
| **Rotate** | Rotates entity by Vec3 degrees |
| **Look At** | Rotates entity to face a target Vec3 |
| **Get Scale** | Outputs entity's Vec3 scale |
| **Set Scale** | Sets entity's scale |
| **Get Forward Vector** | Entity's local forward direction |
| **Get Right Vector** | Entity's local right direction |
| **Get Up Vector** | Entity's local up direction |

### Physics

| Node | Description |
|------|-------------|
| **Apply Force** | Continuous force (x, y, z) — use in On Update |
| **Apply Impulse** | Instant velocity change (x, y, z) |
| **Apply Torque** | Rotational force (x, y, z) |
| **Set Velocity** | Override linear velocity (x, y, z) |
| **Get Velocity** | Read current velocity as Vec3 |
| **Raycast** | Cast ray from origin in direction, outputs hit info |
| **Raycast Down** | Shortcut for downward ground check |
| **Set Gravity Scale** | Change gravity multiplier (0 = float) |
| **Is On Ground** | Bool output — true if ground raycast hits |

### Entity

| Node | Description |
|------|-------------|
| **Find Entity** | Look up entity by name → Entity output |
| **Spawn Entity** | Create a new entity → Entity output |
| **Destroy Entity** | Remove an entity from the scene |
| **Get Component** | Read a component value from an entity |
| **Set Component** | Write a component value on an entity |
| **Get Parent** | Get parent entity reference |
| **Get Children** | Get list of child entities |
| **Get Name** | Get entity's name as String |
| **Set Active** | Enable or disable an entity |
| **Self** | Reference to the entity running this blueprint |

### Animation

| Node | Description |
|------|-------------|
| **Play Animation** | Play a clip on loop |
| **Play Once** | Play a clip once, then stop |
| **Stop Animation** | Stop current animation |
| **Crossfade** | Blend to a new clip over duration |
| **Set Speed** | Change playback speed |
| **Is Playing** | Bool — true if animation is active |
| **Get Current Clip** | String — name of current clip |
| **Blend** | Mix two animations by weight (0–1) |

### Audio

| Node | Description |
|------|-------------|
| **Play Sound** | Play a sound file on an entity |
| **Play Sound At** | Play a sound at a world position |
| **Stop Sound** | Stop an entity's sound |
| **Set Volume** | Set sound volume (0–1) |
| **Set Pitch** | Set sound pitch multiplier |
| **Play Music** | Play background music track |
| **Stop Music** | Stop background music |
| **Set Music Volume** | Set music volume (0–1) |

### UI

| Node | Description |
|------|-------------|
| **Show Widget** | Make a UI widget visible |
| **Hide Widget** | Hide a UI widget |
| **Toggle Widget** | Toggle visibility |
| **Set Text** | Set a label or button's text |
| **Set Progress** | Set progress bar value (0–1) |
| **Set Health** | Set health bar (current, max) |
| **Set Color** | Set widget color (R, G, B, A) |
| **Set Image** | Set widget image from path |
| **Set Slider** | Set slider value |
| **Set Checkbox** | Set checkbox state |
| **Set Toggle** | Set toggle state |
| **Set Theme** | Switch entire UI theme |

### Material

| Node | Description |
|------|-------------|
| **Set Material Color** | Set base color (R, G, B, A) |
| **Set Material Property** | Set a float property (metallic, roughness, etc.) |
| **Set Emissive** | Set emissive color and intensity |
| **Swap Material** | Replace entity's material from file path |

### Variables

| Node | Description |
|------|-------------|
| **Get Variable** | Read a blueprint variable by name |
| **Set Variable** | Write a blueprint variable |
| **Local Variable** | Temporary value within a single execution |
| **Script Property** | Read a property exposed to the Inspector |
| **Make Vec3** | Combine X, Y, Z floats into Vec3 |
| **Break Vec3** | Split Vec3 into X, Y, Z floats |

### Networking

| Node | Description |
|------|-------------|
| **Send RPC** | Send a remote procedure call with arguments |
| **Is Server** | Bool — true if running on the server |
| **Is Owner** | Bool — true if this client owns the entity |
| **Get Network ID** | Entity's network identifier |
| **Get Player Count** | Number of connected players |

## Creating variables

1. Click **+** in the Variables panel (bottom-left of the Blueprint Editor)
2. Name it and choose a type (Float, Int, Bool, String, Vec3)
3. Set a default value
4. Drag it onto the graph to create Get/Set nodes

Check **Expose to Inspector** to make the variable editable in the Inspector panel, just like script `props()`.

## Example: player controller

Build an FPS controller with blueprints:

1. **On Update** → **Translate** (connect `input_x * speed * delta` to X, `input_y * speed * delta` to Z)
2. **On Update** → **Rotate** (connect `mouse_delta_x * look_speed * delta` to Y rotation)
3. **On Key Pressed** (Space) → **Branch** (Is On Ground?) → True → **Apply Impulse** (0, jump_force, 0)
4. **On Collision Enter** → **Play Sound** ("hit.ogg") → **Set Health** (health - 10, max_health)

Variables: `speed` (Float, 5.0), `look_speed` (Float, 2.0), `jump_force` (Float, 8.0), `health` (Float, 100.0), `max_health` (Float, 100.0) — all exposed to Inspector.

## Best practices

- **Keep graphs readable** — use Sequence nodes to organize flow, add Comment nodes to label sections
- **Use sub-blueprints** — extract reusable logic into separate blueprint files and call them with the Execute Blueprint node
- **Name your variables clearly** — `player_speed` not `s`
- **Use events over polling** — On Key Pressed is more efficient than checking `is_key_pressed` every frame in On Update
- **Test incrementally** — wire up a few nodes, hit Play, verify, then add more

## Performance

- Blueprints compile to the same internal representation as scripts — there is no performance penalty vs Rhai or Lua
- Avoid creating hundreds of nodes that execute every frame. Use events and gates to limit unnecessary work
- The Blueprint Editor shows execution time per node when profiling is enabled (**View → Show Node Timings**)
