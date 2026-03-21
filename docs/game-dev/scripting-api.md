# Scripting API Reference

Complete reference for all functions and variables available in Rhai, Lua, and Blueprint scripts.

## Lifecycle functions

| Function | Called when |
|----------|-----------|
| `on_ready()` | Entity spawns (once) |
| `on_update()` | Every frame |
| `on_anim_event(name)` | Animation event fires |
| `on_rpc_<name>(...)` | Network RPC received |
| `on_player_connect(id, name)` | Player joins (server only) |
| `on_player_disconnect(id, name)` | Player leaves (server only) |
| `on_server_tick()` | Every server tick (server only) |
| `props()` | Inspector properties definition |

## Global variables

### Time

| Variable | Type | Description |
|----------|------|-------------|
| `delta` | float | Seconds since last frame. Use for frame-rate independent movement. |
| `elapsed` | float | Total seconds since game started |

### Transform (read/write)

| Variable | Type | Description |
|----------|------|-------------|
| `position_x` | float | World X position |
| `position_y` | float | World Y position |
| `position_z` | float | World Z position |
| `rotation_x` | float | X rotation in degrees |
| `rotation_y` | float | Y rotation in degrees |
| `rotation_z` | float | Z rotation in degrees |
| `scale_x` | float | X scale factor |
| `scale_y` | float | Y scale factor |
| `scale_z` | float | Z scale factor |

### Direction vectors (read-only)

| Variable | Type | Description |
|----------|------|-------------|
| `forward_x/y/z` | float | Entity's local forward direction |
| `right_x/y/z` | float | Entity's local right direction |
| `up_x/y/z` | float | Entity's local up direction |

### Input (read-only)

| Variable | Type | Range | Description |
|----------|------|-------|-------------|
| `input_x` | float | -1 to 1 | Horizontal axis (A/D or Left/Right) |
| `input_y` | float | -1 to 1 | Vertical axis (W/S or Up/Down) |
| `mouse_x` | float | pixels | Mouse screen X position |
| `mouse_y` | float | pixels | Mouse screen Y position |
| `mouse_delta_x` | float | pixels | Mouse X movement since last frame |
| `mouse_delta_y` | float | pixels | Mouse Y movement since last frame |
| `mouse_button_left` | bool | — | Left mouse button held |
| `mouse_button_right` | bool | — | Right mouse button held |
| `mouse_button_middle` | bool | — | Middle mouse button held |

### Gamepad (read-only)

| Variable | Type | Range | Description |
|----------|------|-------|-------------|
| `gamepad_left_x` | float | -1 to 1 | Left stick horizontal |
| `gamepad_left_y` | float | -1 to 1 | Left stick vertical |
| `gamepad_right_x` | float | -1 to 1 | Right stick horizontal |
| `gamepad_right_y` | float | -1 to 1 | Right stick vertical |
| `gamepad_south` | bool | — | A / Cross |
| `gamepad_east` | bool | — | B / Circle |
| `gamepad_north` | bool | — | Y / Triangle |
| `gamepad_west` | bool | — | X / Square |
| `gamepad_left_trigger` | float | 0 to 1 | Left trigger |
| `gamepad_right_trigger` | float | 0 to 1 | Right trigger |

### Touch (read-only, mobile)

| Variable | Type | Description |
|----------|------|-------------|
| `touch_count` | int | Number of active touches |

| Function | Returns | Description |
|----------|---------|-------------|
| `touch_x(index)` | float | X position of touch at index |
| `touch_y(index)` | float | Y position of touch at index |

### Physics (read-only)

| Variable | Type | Description |
|----------|------|-------------|
| `collisions_entered` | int | New collisions this frame |
| `collisions_exited` | int | Ended collisions this frame |
| `active_collisions` | int | Currently overlapping colliders |

### Entity (read-only)

| Variable | Type | Description |
|----------|------|-------------|
| `self_entity_name` | string | This entity's name |
| `children_count` | int | Number of child entities |
| `parent_entity_id` | string | Parent entity ID |

## Functions by category

### Input

| Function | Returns | Description |
|----------|---------|-------------|
| `is_key_pressed(key)` | bool | True while key is held |
| `is_key_just_pressed(key)` | bool | True the single frame key goes down |
| `is_key_just_released(key)` | bool | True the single frame key goes up |

**Key names**: `A`–`Z`, `Digit0`–`Digit9`, `F1`–`F12`, `Space`, `Enter`, `Escape`, `Tab`, `Backspace`, `ShiftLeft`, `ShiftRight`, `ControlLeft`, `ControlRight`, `AltLeft`, `AltRight`, `ArrowUp`, `ArrowDown`, `ArrowLeft`, `ArrowRight`.

### Physics

| Function | Returns | Description |
|----------|---------|-------------|
| `apply_force(x, y, z)` | — | Continuous force, call every frame |
| `apply_impulse(x, y, z)` | — | One-time velocity change |
| `apply_torque(x, y, z)` | — | Rotational force |
| `set_velocity(x, y, z)` | — | Override velocity directly |
| `get_velocity_x()` | float | Current X velocity |
| `get_velocity_y()` | float | Current Y velocity |
| `get_velocity_z()` | float | Current Z velocity |
| `raycast(ox, oy, oz, dx, dy, dz, max_dist, name)` | — | Cast ray, store result |
| `raycast_down(x, y, z, dist, name)` | — | Downward ground check |
| `find_entity_by_name(name)` | entity | Look up entity handle |
| `apply_impulse_to(entity, x, y, z)` | — | Push another entity |

### Entity management

| Function | Returns | Description |
|----------|---------|-------------|
| `spawn_entity(name)` | — | Create a new empty entity |
| `destroy_entity(name)` | — | Remove an entity from the scene |

### Animation

| Function | Returns | Description |
|----------|---------|-------------|
| `play_animation(entity, clip)` | — | Play clip on loop |
| `play_animation_once(entity, clip)` | — | Play clip once, then stop |
| `stop_animation(entity)` | — | Stop current animation |
| `set_animation_speed(entity, speed)` | — | Set playback speed (1.0 = normal) |
| `blend_animation(entity, clip_a, clip_b, weight)` | — | Blend two clips (0.0–1.0) |
| `crossfade_animation(entity, clip, duration)` | — | Smooth transition to clip |
| `set_anim_param(entity, param, value)` | — | Set state machine parameter |
| `set_anim_trigger(entity, trigger)` | — | Fire a one-shot state machine trigger |

### Audio

| Function | Returns | Description |
|----------|---------|-------------|
| `play_sound(entity, path)` | — | Play sound on entity (spatial) |
| `play_sound_at(x, y, z, path)` | — | Play sound at world position |
| `stop_sound(entity)` | — | Stop entity's sound |
| `set_volume(entity, vol)` | — | Set volume (0.0–1.0) |
| `set_pitch(entity, pitch)` | — | Set pitch (1.0 = normal) |
| `play_music(path)` | — | Play background music |
| `stop_music()` | — | Stop background music |
| `set_music_volume(vol)` | — | Set music volume (0.0–1.0) |

### UI

| Function | Returns | Description |
|----------|---------|-------------|
| `ui_show(name)` | — | Show a UI widget |
| `ui_hide(name)` | — | Hide a UI widget |
| `ui_toggle(name)` | — | Toggle widget visibility |
| `ui_set_text(name, text)` | — | Set widget text content |
| `ui_set_progress(name, val)` | — | Set progress bar (0.0–1.0) |
| `ui_set_health(name, cur, max)` | — | Set health bar values |
| `ui_set_color(name, r, g, b, a)` | — | Set widget color (0–255) |
| `ui_set_slider(name, val)` | — | Set slider position |
| `ui_set_checkbox(name, val)` | — | Set checkbox state |
| `ui_set_toggle(name, val)` | — | Set toggle state |
| `ui_set_image(name, path)` | — | Set widget image |
| `ui_set_theme(theme)` | — | Switch UI theme ("dark", "light", "high_contrast") |

### Material

| Function | Returns | Description |
|----------|---------|-------------|
| `set_material_color(entity, r, g, b, a)` | — | Set base color (0–255) |
| `set_material_property(entity, prop, val)` | — | Set float property (metallic, roughness, opacity, alpha_cutoff) |
| `set_material_emissive(entity, r, g, b, intensity)` | — | Set emissive color and brightness |
| `swap_material(entity, path)` | — | Replace entire material from file |

### Terrain

| Function | Returns | Description |
|----------|---------|-------------|
| `terrain_get_height(x, z)` | float | Get terrain height at world position |
| `terrain_set_height(x, z, h)` | — | Set terrain height (runtime sculpting) |

### Camera

| Function | Returns | Description |
|----------|---------|-------------|
| `set_camera_fov(camera, fov)` | — | Set field of view in degrees |
| `set_camera_position(camera, x, y, z)` | — | Move camera |
| `camera_look_at(camera, x, y, z)` | — | Point camera at target |
| `camera_screen_to_world(camera, sx, sy)` | vec3 | Convert screen to world position |
| `camera_world_to_screen(camera, x, y, z)` | vec2 | Convert world to screen position |
| `camera_shake(camera, intensity, duration)` | — | Trigger camera shake |

### Network

| Function | Returns | Description |
|----------|---------|-------------|
| `rpc_send(name, ...)` | — | Send RPC (client→server or server→all) |
| `rpc_send_to(target_id, name, ...)` | — | Send RPC to specific client |
| `rpc_send_except(exclude_id, name, ...)` | — | Send to all except one client |
| `is_server()` | bool | Running on dedicated server? |
| `is_owner()` | bool | This client owns this entity? |
| `get_network_id()` | int | Entity's network identifier |
| `get_player_count()` | int | Number of connected players |

### Utility

| Function | Returns | Description |
|----------|---------|-------------|
| `print(msg)` | — | Log a message to the console |
| `clamp(val, min, max)` | float | Constrain value to range |
| `lerp(a, b, t)` | float | Linear interpolation |
| `random_range(min, max)` | float | Random float in range |
