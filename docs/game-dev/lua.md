# Lua Scripting

Write game logic with Lua 5.4 — the industry-standard game scripting language.

## Your first script

Create a file called `scripts/player.lua`:

```lua
function on_ready()
    print("Player spawned!")
end

function on_update()
    local speed = 5.0
    position_x = position_x + input_x * speed * delta
    position_z = position_z + input_y * speed * delta
end
```

Attach it to an entity in the Inspector. When you hit play, the entity moves with WASD.

## Script properties

Expose variables to the Inspector so designers can tweak values without editing code:

```lua
function props()
    return {
        speed = { default = 5.0, min = 0.0, max = 100.0 },
        jump_force = { default = 10.0, min = 0.0, max = 50.0 },
        can_fly = { default = false }
    }
end

function on_update()
    position_x = position_x + input_x * speed * delta
end
```

The `speed`, `jump_force`, and `can_fly` properties appear as editable fields in the Inspector with appropriate widgets (sliders, checkboxes).

## Working with collisions

```lua
function on_update()
    if collisions_entered > 0 then
        print("Hit something!")
    end

    if active_collisions > 0 then
        print("Still touching")
    end
end
```

## Entity hierarchy

```lua
function on_ready()
    print(self_entity_name)
    print("Children: " .. children_count)
    print("Parent: " .. parent_entity_id)
end
```

## Lua syntax basics

```lua
-- Variables
local x = 42
local name = "hello"

-- Functions
function add(a, b)
    return a + b
end

-- Tables (like maps/objects)
local player = { health = 100, name = "Hero" }
print(player.health)

-- Control flow
if x > 10 then
    print("big")
elseif x > 5 then
    print("medium")
else
    print("small")
end

-- Loops
for i = 1, 10 do
    print(i)
end

while x > 0 do
    x = x - 1
end

-- Arrays
local items = { "sword", "shield", "potion" }
for i, item in ipairs(items) do
    print(i, item)
end
```

## Differences from Rhai

| Feature | Lua | Rhai |
|---------|-----|------|
| **Maps** | `{ key = value }` | `#{ key: value }` |
| **Nil/null** | `nil` | `()` |
| **String concat** | `..` | `+` |
| **Not equal** | `~=` | `!=` |
| **Array index** | 1-based | 0-based |
| **Local vars** | `local x = 5` | `let x = 5` |
| **Block end** | `end` | `}` |
| **Logical ops** | `and`, `or`, `not` | `&&`, `\|\|`, `!` |

## Full API reference

### Globals (read/write every frame)

| Variable | Type | Description |
|----------|------|-------------|
| `delta` | float | Seconds since last frame |
| `elapsed` | float | Total seconds since game started |
| `position_x/y/z` | float | Entity world position |
| `rotation_x/y/z` | float | Entity rotation (degrees) |
| `scale_x/y/z` | float | Entity scale |
| `input_x` | float | Horizontal axis (-1 to 1, A/D keys) |
| `input_y` | float | Vertical axis (-1 to 1, W/S keys) |
| `mouse_x/y` | float | Mouse screen position |
| `mouse_delta_x/y` | float | Mouse movement since last frame |
| `mouse_button_left/right/middle` | bool | Mouse button held state |
| `gamepad_left_x/y` | float | Left stick axes |
| `gamepad_right_x/y` | float | Right stick axes |
| `gamepad_south/east/north/west` | bool | Face buttons |
| `gamepad_left_trigger/right_trigger` | float | Trigger axes (0–1) |
| `collisions_entered` | int | Collisions started this frame |
| `collisions_exited` | int | Collisions ended this frame |
| `active_collisions` | int | Current overlapping colliders |
| `self_entity_name` | string | This entity's name |
| `children_count` | int | Number of child entities |
| `parent_entity_id` | string | Parent entity ID |

### Functions

| Function | Description |
|----------|-------------|
| `is_key_pressed(key)` | True while key is held |
| `is_key_just_pressed(key)` | True the frame key goes down |
| `is_key_just_released(key)` | True the frame key goes up |
| `apply_force(x, y, z)` | Continuous force (per frame) |
| `apply_impulse(x, y, z)` | Instant velocity change |
| `apply_torque(x, y, z)` | Rotational force |
| `set_velocity(x, y, z)` | Override linear velocity |
| `raycast(ox,oy,oz, dx,dy,dz, dist, name)` | Cast ray, store result |
| `raycast_down(x, y, z, dist, name)` | Downward raycast |
| `find_entity_by_name(name)` | Get entity handle by name |
| `apply_impulse_to(entity, x, y, z)` | Push another entity |
| `spawn_entity(name)` | Spawn a new entity |
| `destroy_entity(name)` | Remove an entity |
| `play_animation(entity, clip)` | Play animation clip (loop) |
| `play_animation_once(entity, clip)` | Play animation once |
| `stop_animation(entity)` | Stop current animation |
| `set_animation_speed(entity, speed)` | Set playback speed |
| `blend_animation(entity, a, b, weight)` | Blend two clips |
| `crossfade_animation(entity, clip, dur)` | Crossfade to clip |
| `play_sound(entity, path)` | Play sound on entity |
| `play_sound_at(x, y, z, path)` | Play sound at position |
| `stop_sound(entity)` | Stop entity's sound |
| `set_volume(entity, vol)` | Set sound volume (0–1) |
| `set_pitch(entity, pitch)` | Set sound pitch |
| `play_music(path)` | Play background music |
| `stop_music()` | Stop background music |
| `set_music_volume(vol)` | Set music volume (0–1) |
| `ui_show(name)` | Show a UI widget |
| `ui_hide(name)` | Hide a UI widget |
| `ui_toggle(name)` | Toggle widget visibility |
| `ui_set_text(name, text)` | Set widget text |
| `ui_set_progress(name, val)` | Set progress bar (0–1) |
| `ui_set_health(name, cur, max)` | Set health bar |
| `ui_set_color(name, r, g, b, a)` | Set widget color |
| `ui_set_slider(name, val)` | Set slider value |
| `ui_set_checkbox(name, val)` | Set checkbox state |
| `ui_set_toggle(name, val)` | Set toggle state |
| `ui_set_image(name, path)` | Set widget image |
| `ui_set_theme(theme)` | Switch UI theme |
| `set_material_color(entity, r, g, b, a)` | Set material base color |
| `set_material_property(entity, prop, val)` | Set material float property |
| `set_material_emissive(entity, r, g, b, intensity)` | Set emissive color |
| `swap_material(entity, path)` | Replace entity's material |
| `rpc_send(name, ...)` | Send network RPC |
| `is_server()` | True if running on server |
| `is_owner()` | True if entity is locally owned |
| `get_network_id()` | Get entity's network ID |
| `get_player_count()` | Get connected player count |

## When to use Lua vs Rhai

- **Choose Lua** if you're coming from Unity, Godot, Roblox, or other engines that use Lua. The syntax will feel familiar.
- **Choose Rhai** if you're a Rust developer or prefer Rust-like syntax. Rhai has tighter Rust integration and slightly less overhead.
- **Both have identical APIs** — every function and variable available in Rhai is also available in Lua.
- You can mix both in the same project. Different entities can use different scripting languages.
