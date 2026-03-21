# Blueprint Node API Reference

Complete reference for all built-in blueprint nodes.

## Event nodes

Entry points that start execution flow.

| Node | Fires when | Output pins |
|------|-----------|-------------|
| **On Ready** | Entity spawns (once) | Exec |
| **On Update** | Every frame | Exec, Delta (float), Elapsed (float) |
| **On Collision Enter** | Physics collision starts | Exec, Other Entity |
| **On Collision Exit** | Physics collision ends | Exec, Other Entity |
| **On Key Pressed** | Key goes down | Exec, Key (string) |
| **On Key Released** | Key goes up | Exec, Key (string) |
| **On Timer** | After configurable delay | Exec |
| **On Anim Event** | Animation event fires | Exec, Event Name (string) |
| **On RPC Received** | Network RPC arrives | Exec, Args |

## Flow control

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Branch** | Condition (bool) | True, False | If/else routing |
| **Sequence** | — | Out 1, Out 2, Out 3… | Run outputs in order |
| **For Loop** | Start (int), End (int) | Loop Body, Index, Completed | Iterate range |
| **While Loop** | Condition (bool) | Loop Body, Completed | Repeat while true |
| **Delay** | Seconds (float) | Completed | Pause execution |
| **Gate** | Open (bool) | Out | Only passes when open |
| **Switch (Int)** | Value (int) | Case 0, 1, 2…, Default | Route by integer |
| **Switch (String)** | Value (string) | Named cases, Default | Route by string |
| **Do Once** | Reset (bool) | Out | Fire once, ignore until reset |
| **Flip Flop** | — | A, B, Is A (bool) | Alternate between outputs |

## Math nodes

### Arithmetic

| Node | Inputs | Output | Formula |
|------|--------|--------|---------|
| **Add** | A, B | Result | A + B |
| **Subtract** | A, B | Result | A - B |
| **Multiply** | A, B | Result | A × B |
| **Divide** | A, B | Result | A / B |
| **Modulo** | A, B | Result | A % B |
| **Power** | Base, Exp | Result | Base ^ Exp |
| **Negate** | Value | Result | -Value |

### Range

| Node | Inputs | Output | Description |
|------|--------|--------|-------------|
| **Clamp** | Value, Min, Max | Result | Constrain to range |
| **Lerp** | A, B, T | Result | Linear interpolation |
| **Inverse Lerp** | A, B, Value | Result | Where does Value fall in A–B? (0–1) |
| **Remap** | Value, InMin, InMax, OutMin, OutMax | Result | Map from one range to another |
| **Random Range** | Min, Max | Result | Random float |
| **Min** | A, B | Result | Smaller value |
| **Max** | A, B | Result | Larger value |
| **Saturate** | Value | Result | Clamp to 0–1 |

### Trigonometry

| Node | Input | Output |
|------|-------|--------|
| **Sin** | Angle (rad) | Result |
| **Cos** | Angle (rad) | Result |
| **Tan** | Angle (rad) | Result |
| **Asin** | Value | Angle (rad) |
| **Acos** | Value | Angle (rad) |
| **Atan2** | Y, X | Angle (rad) |
| **Degrees to Radians** | Degrees | Radians |
| **Radians to Degrees** | Radians | Degrees |

### Rounding

| Node | Input | Output |
|------|-------|--------|
| **Abs** | Value | Result |
| **Floor** | Value | Result |
| **Ceil** | Value | Result |
| **Round** | Value | Result |
| **Sign** | Value | -1, 0, or 1 |
| **Fract** | Value | Fractional part |

### Vector math

| Node | Inputs | Output |
|------|--------|--------|
| **Distance** | A (Vec3), B (Vec3) | Float |
| **Normalize** | Vec (Vec3) | Vec3 (unit) |
| **Dot Product** | A (Vec3), B (Vec3) | Float |
| **Cross Product** | A (Vec3), B (Vec3) | Vec3 |
| **Length** | Vec (Vec3) | Float |
| **Make Vec3** | X, Y, Z (float) | Vec3 |
| **Break Vec3** | Vec (Vec3) | X, Y, Z (float) |
| **Vec3 Lerp** | A, B (Vec3), T (float) | Vec3 |

## Transform nodes

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Get Position** | Entity | Vec3 | Entity's world position |
| **Set Position** | Entity, Vec3 | — | Set entity position |
| **Get Rotation** | Entity | Vec3 (degrees) | Entity's rotation |
| **Set Rotation** | Entity, Vec3 | — | Set entity rotation |
| **Translate** | Entity, Vec3 | — | Move entity by offset |
| **Rotate** | Entity, Vec3 | — | Rotate by degrees |
| **Look At** | Entity, Target (Vec3) | — | Face a point |
| **Get Scale** | Entity | Vec3 | Entity's scale |
| **Set Scale** | Entity, Vec3 | — | Set entity scale |
| **Get Forward** | Entity | Vec3 | Local forward direction |
| **Get Right** | Entity | Vec3 | Local right direction |
| **Get Up** | Entity | Vec3 | Local up direction |

## Physics nodes

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Apply Force** | X, Y, Z | — | Continuous force (per frame) |
| **Apply Impulse** | X, Y, Z | — | Instant velocity change |
| **Apply Torque** | X, Y, Z | — | Rotational force |
| **Set Velocity** | X, Y, Z | — | Override velocity |
| **Get Velocity** | — | Vec3 | Current velocity |
| **Raycast** | Origin, Direction, Distance | Hit (bool), Point, Normal, Entity | Cast a ray |
| **Raycast Down** | Position, Distance | Hit (bool), Height | Ground check |
| **Set Gravity Scale** | Scale (float) | — | Change gravity (0 = float) |
| **Is On Ground** | — | Bool | Downward raycast check |

## Entity nodes

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Self** | — | Entity | This blueprint's entity |
| **Find Entity** | Name (string) | Entity | Find by name |
| **Spawn Entity** | Name (string) | Entity | Create empty entity |
| **Destroy Entity** | Entity | — | Remove from scene |
| **Get Name** | Entity | String | Entity's name |
| **Get Parent** | Entity | Entity | Parent entity |
| **Get Children** | Entity | Array | Child entities |
| **Set Active** | Entity, Active (bool) | — | Enable/disable |
| **Get Component** | Entity, Name | Value | Read component value |
| **Set Component** | Entity, Name, Value | — | Write component value |

## Animation nodes

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Play Animation** | Entity, Clip | — | Loop a clip |
| **Play Once** | Entity, Clip | — | Play once |
| **Stop Animation** | Entity | — | Stop playback |
| **Crossfade** | Entity, Clip, Duration | — | Smooth transition |
| **Blend** | Entity, A, B, Weight | — | Mix two clips |
| **Set Speed** | Entity, Speed | — | Playback speed |
| **Is Playing** | Entity | Bool | Animation active? |
| **Get Current Clip** | Entity | String | Current clip name |

## Audio nodes

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Play Sound** | Entity, Path | — | Play on entity |
| **Play Sound At** | Position (Vec3), Path | — | Play at position |
| **Stop Sound** | Entity | — | Stop playback |
| **Set Volume** | Entity, Volume | — | 0.0–1.0 |
| **Set Pitch** | Entity, Pitch | — | Pitch multiplier |
| **Play Music** | Path | — | Background music |
| **Stop Music** | — | — | Stop music |
| **Set Music Volume** | Volume | — | 0.0–1.0 |

## UI nodes

| Node | Inputs | Description |
|------|--------|-------------|
| **Show Widget** | Name | Make visible |
| **Hide Widget** | Name | Make hidden |
| **Toggle Widget** | Name | Toggle visibility |
| **Set Text** | Name, Text | Set label/button text |
| **Set Progress** | Name, Value (0–1) | Set progress bar |
| **Set Health** | Name, Current, Max | Set health bar |
| **Set Color** | Name, R, G, B, A (0–255) | Set widget color |
| **Set Image** | Name, Path | Set image source |
| **Set Slider** | Name, Value | Set slider position |
| **Set Checkbox** | Name, Value (bool) | Set checkbox |
| **Set Toggle** | Name, Value (bool) | Set toggle |
| **Set Theme** | Theme (string) | Switch UI theme |

## Material nodes

| Node | Inputs | Description |
|------|--------|-------------|
| **Set Material Color** | Entity, R, G, B, A | Set base color |
| **Set Material Property** | Entity, Property, Value | Set float property |
| **Set Emissive** | Entity, R, G, B, Intensity | Set glow |
| **Swap Material** | Entity, Path | Replace material |

## Variable nodes

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Get Variable** | Name | Value | Read blueprint variable |
| **Set Variable** | Name, Value | — | Write blueprint variable |
| **Local Variable** | — | Value | Temporary within execution |
| **Script Property** | Name | Value | Read Inspector-exposed property |
| **Make Vec3** | X, Y, Z | Vec3 | Compose vector |
| **Break Vec3** | Vec3 | X, Y, Z | Decompose vector |

## Network nodes

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Send RPC** | Name, Args | — | Send remote call |
| **Send RPC To** | Target, Name, Args | — | Send to specific client |
| **Is Server** | — | Bool | On dedicated server? |
| **Is Owner** | — | Bool | Entity owned locally? |
| **Get Network ID** | — | Int | Entity's net ID |
| **Get Player Count** | — | Int | Connected players |

## Camera nodes

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Set Camera FOV** | Camera, FOV | — | Set field of view |
| **Camera Look At** | Camera, Target (Vec3) | — | Point camera |
| **Screen To World** | Camera, X, Y | Vec3 | Screen → world ray |
| **World To Screen** | Camera, Position | Vec2 | World → screen pos |
| **Camera Shake** | Camera, Intensity, Duration | — | Trigger shake |

## Terrain nodes

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Get Terrain Height** | X, Z | Height (float) | Sample terrain |
| **Set Terrain Height** | X, Z, Height | — | Modify terrain |
