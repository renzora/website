# Animation

Bring entities to life with skeletal animation, clips, and state machines.

## Overview

Renzora's animation system supports:

- **Skeletal animation** — bone-based deformation for characters and creatures
- **Animation clips** — individual animations (walk, run, idle, attack)
- **State machines** — automatic transitions between clips based on conditions
- **Animation events** — trigger script callbacks at specific keyframes

## Importing animations

Animations are imported from **glTF** (.gltf, .glb) and **FBX** (.fbx) files.

1. Drop an animated model into your project's `assets/` folder
2. The engine auto-detects embedded animation clips
3. Clips appear in the **Animation** panel when the entity is selected

Each clip is named from the source file (e.g., `walk`, `run`, `idle`). Rename clips in the Animation panel.

## The Animation panel

Open **Window → Animation** to view and manage clips.

- **Clip list** — all clips on the selected entity
- **Timeline** — scrub through keyframes
- **Preview** — play/pause individual clips
- **Events** — add callback markers at keyframes

## Playing animations from scripts

```rhai
fn on_ready() {
    // Play a looping animation
    play_animation("my_character", "idle");
}

fn on_update() {
    // Switch based on movement
    if input_x != 0.0 || input_y != 0.0 {
        crossfade_animation("my_character", "run", 0.2);
    } else {
        crossfade_animation("my_character", "idle", 0.3);
    }

    // Play a one-shot animation (doesn't loop)
    if is_key_just_pressed("F") {
        play_animation_once("my_character", "attack");
    }
}
```

### Animation API reference

| Function | Description |
|----------|-------------|
| `play_animation(entity, clip)` | Play a clip on loop |
| `play_animation_once(entity, clip)` | Play a clip once, then stop |
| `stop_animation(entity)` | Stop the current animation |
| `set_animation_speed(entity, speed)` | Set playback speed (1.0 = normal, 2.0 = double, 0.5 = half) |
| `blend_animation(entity, clip_a, clip_b, weight)` | Blend two clips (weight 0.0 = all A, 1.0 = all B) |
| `crossfade_animation(entity, clip, duration)` | Smoothly transition to a new clip over `duration` seconds |

## Animation state machine

For complex characters, use a **state machine** instead of scripting every transition.

### Setting up states

1. Select an entity with animations
2. Open **Window → Animation State Machine**
3. Create states (right-click → Add State) and assign a clip to each
4. Draw transitions between states (drag from one state to another)

### Transition conditions

Each transition has one or more conditions:

| Condition type | Example |
|----------------|---------|
| **Bool parameter** | `is_grounded == true` |
| **Float threshold** | `speed > 0.5` |
| **Trigger** | `attack` (fires once, auto-resets) |
| **Time** | `clip_finished` (when current clip ends) |

### Setting parameters from scripts

```rhai
fn on_update() {
    let speed = (input_x * input_x + input_y * input_y).sqrt();
    set_anim_param("my_character", "speed", speed);
    set_anim_param("my_character", "is_grounded", true);

    if is_key_just_pressed("Space") {
        set_anim_trigger("my_character", "jump");
    }
}
```

## Animation blending

Blend between animations for smooth transitions:

```rhai
fn on_update() {
    // Blend walk and run based on speed
    let speed = (input_x * input_x + input_y * input_y).sqrt();
    let blend = clamp(speed / 5.0, 0.0, 1.0);
    blend_animation("player", "walk", "run", blend);
}
```

## Animation events

Add events at specific keyframes to trigger script callbacks:

1. In the Animation panel, select a clip
2. Right-click the timeline → **Add Event**
3. Name the event (e.g., `footstep`, `swing_hit`)

Handle events in your script:

```rhai
fn on_anim_event(event_name) {
    if event_name == "footstep" {
        play_sound_at(position_x, position_y, position_z, "sounds/footstep.ogg");
    }
    if event_name == "swing_hit" {
        // Activate hitbox, deal damage, etc.
    }
}
```

## Tips

- **Crossfade duration** of 0.15–0.3 seconds works well for most character transitions
- **Animation speed** can be negative to play clips in reverse
- **Root motion** — if your animation moves the character in the clip, enable Root Motion on the entity to apply that movement to the transform
- **Additive animations** — layer animations on top of each other (e.g., breathing on top of running)
- **Performance** — skeletal animation is GPU-accelerated. Hundreds of animated characters are fine
