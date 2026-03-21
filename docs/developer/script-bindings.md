# Script API Bindings

Extend the scripting API available to Rhai, Lua, and Blueprints.

## ScriptExtension trait

Every crate that adds functions to scripts implements this trait:

```rust
pub trait ScriptExtension: Send + Sync {
    /// Inject per-entity variables into the script context each frame
    fn populate_context(&self, world: &World, entity: Entity, data: &mut ExtensionData);

    /// Register global Rhai functions
    fn register_rhai_functions(&self, engine: &mut rhai::Engine);

    /// Register global Lua functions
    fn register_lua_functions(&self, lua: &mlua::Lua);
}
```

Audio, physics, UI, animation, networking, and materials all use this trait.

## Complete API reference

### Time

| Binding | Type | Access | Description |
|---------|------|--------|-------------|
| `delta` | float | read | Seconds since last frame |
| `elapsed` | float | read | Total seconds since game started |

### Transform

| Binding | Type | Access | Description |
|---------|------|--------|-------------|
| `position_x` | float | read/write | Entity X position |
| `position_y` | float | read/write | Entity Y position |
| `position_z` | float | read/write | Entity Z position |
| `rotation_x` | float | read/write | Entity X rotation (degrees) |
| `rotation_y` | float | read/write | Entity Y rotation (degrees) |
| `rotation_z` | float | read/write | Entity Z rotation (degrees) |
| `scale_x` | float | read/write | Entity X scale |
| `scale_y` | float | read/write | Entity Y scale |
| `scale_z` | float | read/write | Entity Z scale |
| `forward_x/y/z` | float | read | Entity's local forward direction |
| `right_x/y/z` | float | read | Entity's local right direction |
| `up_x/y/z` | float | read | Entity's local up direction |

### Input

| Binding | Type | Access | Description |
|---------|------|--------|-------------|
| `input_x` | float | read | Horizontal axis (-1 to 1, A/D keys) |
| `input_y` | float | read | Vertical axis (-1 to 1, W/S keys) |
| `mouse_x` | float | read | Mouse screen X position |
| `mouse_y` | float | read | Mouse screen Y position |
| `mouse_delta_x` | float | read | Mouse X movement since last frame |
| `mouse_delta_y` | float | read | Mouse Y movement since last frame |
| `mouse_button_left` | bool | read | Left mouse button held |
| `mouse_button_right` | bool | read | Right mouse button held |
| `mouse_button_middle` | bool | read | Middle mouse button held |
| `gamepad_left_x/y` | float | read | Left stick axes |
| `gamepad_right_x/y` | float | read | Right stick axes |
| `gamepad_south/east/north/west` | bool | read | Face buttons (A/B/X/Y) |
| `gamepad_left_trigger` | float | read | Left trigger (0–1) |
| `gamepad_right_trigger` | float | read | Right trigger (0–1) |

| Function | Signature | Description |
|----------|-----------|-------------|
| `is_key_pressed` | `(key: String) → bool` | True while key is held |
| `is_key_just_pressed` | `(key: String) → bool` | True the frame key goes down |
| `is_key_just_released` | `(key: String) → bool` | True the frame key goes up |

Key names: `Space`, `ShiftLeft`, `ShiftRight`, `ControlLeft`, `AltLeft`, `Escape`, `Enter`, `Tab`, `Backspace`, `A`–`Z`, `Digit0`–`Digit9`, `F1`–`F12`, `ArrowUp/Down/Left/Right`.

### Physics

| Binding | Type | Access | Description |
|---------|------|--------|-------------|
| `collisions_entered` | int | read | New collisions this frame |
| `collisions_exited` | int | read | Ended collisions this frame |
| `active_collisions` | int | read | Current overlapping colliders |

| Function | Signature | Description |
|----------|-----------|-------------|
| `apply_force` | `(x, y, z: float)` | Continuous force (call every frame) |
| `apply_impulse` | `(x, y, z: float)` | Instant velocity change |
| `apply_torque` | `(x, y, z: float)` | Rotational force |
| `set_velocity` | `(x, y, z: float)` | Override linear velocity |
| `get_velocity_x/y/z` | `() → float` | Read current velocity component |
| `raycast` | `(ox,oy,oz, dx,dy,dz: float, dist: float, name: String)` | Cast a ray, store result by name |
| `raycast_down` | `(x,y,z: float, dist: float, name: String)` | Shortcut for downward raycast |
| `find_entity_by_name` | `(name: String) → Entity` | Look up entity by name |
| `apply_impulse_to` | `(entity: Entity, x,y,z: float)` | Push another entity |

### Entity

| Binding | Type | Access | Description |
|---------|------|--------|-------------|
| `self_entity_name` | string | read | This entity's name |
| `children_count` | int | read | Number of child entities |
| `parent_entity_id` | string | read | Parent entity ID |

| Function | Signature | Description |
|----------|-----------|-------------|
| `spawn_entity` | `(name: String)` | Spawn a new empty entity |
| `destroy_entity` | `(name: String)` | Remove an entity |

### Animation

| Function | Signature | Description |
|----------|-----------|-------------|
| `play_animation` | `(entity: String, clip: String)` | Play clip on loop |
| `play_animation_once` | `(entity: String, clip: String)` | Play clip once |
| `stop_animation` | `(entity: String)` | Stop current animation |
| `set_animation_speed` | `(entity: String, speed: float)` | Set playback speed |
| `blend_animation` | `(entity: String, a: String, b: String, weight: float)` | Blend two clips |
| `crossfade_animation` | `(entity: String, clip: String, duration: float)` | Smooth transition |
| `set_anim_param` | `(entity: String, param: String, value: Any)` | Set state machine parameter |
| `set_anim_trigger` | `(entity: String, trigger: String)` | Fire a one-shot trigger |

### Audio

| Function | Signature | Description |
|----------|-----------|-------------|
| `play_sound` | `(entity: String, path: String)` | Play sound on entity |
| `play_sound_at` | `(x,y,z: float, path: String)` | Play sound at position |
| `stop_sound` | `(entity: String)` | Stop entity's sound |
| `set_volume` | `(entity: String, vol: float)` | Set volume (0–1) |
| `set_pitch` | `(entity: String, pitch: float)` | Set pitch multiplier |
| `play_music` | `(path: String)` | Play background music |
| `stop_music` | `()` | Stop background music |
| `set_music_volume` | `(vol: float)` | Set music volume (0–1) |

### UI

| Function | Signature | Description |
|----------|-----------|-------------|
| `ui_show` | `(name: String)` | Show a UI widget |
| `ui_hide` | `(name: String)` | Hide a UI widget |
| `ui_toggle` | `(name: String)` | Toggle visibility |
| `ui_set_text` | `(name: String, text: String)` | Set widget text |
| `ui_set_progress` | `(name: String, val: float)` | Set progress (0–1) |
| `ui_set_health` | `(name: String, cur: float, max: float)` | Set health bar |
| `ui_set_color` | `(name: String, r,g,b,a: int)` | Set color (0–255) |
| `ui_set_slider` | `(name: String, val: float)` | Set slider value |
| `ui_set_checkbox` | `(name: String, val: bool)` | Set checkbox |
| `ui_set_toggle` | `(name: String, val: bool)` | Set toggle |
| `ui_set_image` | `(name: String, path: String)` | Set widget image |
| `ui_set_theme` | `(theme: String)` | Switch UI theme |

### Material

| Function | Signature | Description |
|----------|-----------|-------------|
| `set_material_color` | `(entity: String, r,g,b,a: int)` | Set base color (0–255) |
| `set_material_property` | `(entity: String, prop: String, val: float)` | Set float property |
| `set_material_emissive` | `(entity: String, r,g,b: int, intensity: float)` | Set emissive |
| `swap_material` | `(entity: String, path: String)` | Replace material |

### Network

| Function | Signature | Description |
|----------|-----------|-------------|
| `rpc_send` | `(name: String, ...args)` | Send RPC |
| `rpc_send_to` | `(target: int, name: String, ...args)` | Send to specific client |
| `is_server` | `() → bool` | Running on server? |
| `is_owner` | `() → bool` | Entity owned by this client? |
| `get_network_id` | `() → int` | Entity's network ID |
| `get_player_count` | `() → int` | Connected players |

## Adding new bindings

### Register a Rhai function

```rust
pub struct InventoryScriptExtension;

impl ScriptExtension for InventoryScriptExtension {
    fn register_rhai_functions(&self, engine: &mut rhai::Engine) {
        engine.register_fn("inventory_add", |item: String, count: i64| -> bool {
            // Implementation
            true
        });

        engine.register_fn("inventory_count", |item: String| -> i64 {
            // Implementation
            0
        });

        engine.register_fn("inventory_has", |item: String| -> bool {
            // Implementation
            false
        });
    }

    fn register_lua_functions(&self, lua: &mlua::Lua) {
        let globals = lua.globals();
        globals.set("inventory_add", lua.create_function(|_, (item, count): (String, i64)| {
            Ok(true)
        }).unwrap()).unwrap();
    }

    fn populate_context(&self, world: &World, entity: Entity, data: &mut ExtensionData) {
        // Inject per-entity variables
        if let Some(inventory) = world.get::<Inventory>(entity) {
            data.set("inventory_size", inventory.items.len() as f64);
            data.set("inventory_full", inventory.items.len() >= inventory.max_slots);
        }
    }
}
```

Register:
```rust
app.register_script_extension(InventoryScriptExtension);
```

### Type mapping

| Rust | Rhai | Lua |
|------|------|-----|
| `f64` | `FLOAT` | `number` |
| `i64` | `INT` | `integer` |
| `bool` | `bool` | `boolean` |
| `String` | `String` | `string` |
| `()` | `()` | `nil` |
| `Vec<Dynamic>` | `Array` | `table` (sequence) |
| `Map` | `Map` | `table` (hash) |

## Testing bindings

```rust
#[cfg(test)]
mod tests {
    use renzora_scripting::test::ScriptTestHarness;

    #[test]
    fn test_inventory_binding() {
        let mut harness = ScriptTestHarness::new();
        harness.register_extension(InventoryScriptExtension);

        let result = harness.eval_rhai("inventory_add(\"sword\", 1)");
        assert_eq!(result.as_bool().unwrap(), true);
    }
}
```
