# Audio

Add sound effects, music, and spatial audio to your game.

## Overview

Renzora's audio system supports:

- **Sound effects** — short clips triggered by gameplay events
- **Music** — background tracks with crossfading
- **Spatial audio** — 3D-positioned sounds with distance falloff
- **Audio buses** — group sounds for volume control (SFX, Music, Voice, UI)

## Supported formats

| Format | Best for |
|--------|----------|
| **OGG Vorbis** (.ogg) | Music, long audio (compressed, good quality) |
| **WAV** (.wav) | Sound effects (uncompressed, instant playback) |
| **MP3** (.mp3) | Music (compressed, widely compatible) |
| **FLAC** (.flac) | High-quality archival (lossless compression) |

## Adding audio to entities

### Audio Source component

1. Select an entity → Inspector → **Add Component → Audio Source**
2. Assign an audio file
3. Configure properties:

| Property | Default | Description |
|----------|---------|-------------|
| **Volume** | 1.0 | Playback volume (0.0–1.0) |
| **Pitch** | 1.0 | Playback speed/pitch (0.5 = half, 2.0 = double) |
| **Loop** | false | Repeat when finished |
| **Play on Start** | false | Auto-play when entity spawns |
| **Spatial** | true | 3D-positioned sound |
| **Min Distance** | 1.0 | Full volume within this radius (meters) |
| **Max Distance** | 50.0 | Silent beyond this radius |
| **Rolloff** | Inverse | How volume fades with distance (Inverse, Linear, None) |

## Playing audio from scripts

### Sound effects

```rhai
fn on_update() {
    // Play a sound on an entity (spatial, positioned at entity)
    if is_key_just_pressed("Space") {
        play_sound("my_entity", "sounds/jump.ogg");
    }

    // Play a sound at a specific world position
    if collisions_entered > 0 {
        play_sound_at(position_x, position_y, position_z, "sounds/impact.ogg");
    }

    // Stop a playing sound
    if is_key_just_pressed("Escape") {
        stop_sound("my_entity");
    }

    // Adjust volume and pitch
    set_volume("my_entity", 0.5);
    set_pitch("my_entity", 1.2);
}
```

### Background music

```rhai
fn on_ready() {
    play_music("music/main_theme.ogg");
    set_music_volume(0.6);
}

fn on_update() {
    // Change music based on game state
    if health < 20 {
        play_music("music/danger.ogg");  // auto-crossfades
    }
}
```

### Audio API reference

| Function | Description |
|----------|-------------|
| `play_sound(entity, path)` | Play a sound on an entity (uses entity's position for spatial) |
| `play_sound_at(x, y, z, path)` | Play a sound at a world position |
| `stop_sound(entity)` | Stop the sound playing on an entity |
| `set_volume(entity, vol)` | Set sound volume (0.0–1.0) |
| `set_pitch(entity, pitch)` | Set playback pitch (1.0 = normal) |
| `play_music(path)` | Play background music (crossfades if music is already playing) |
| `stop_music()` | Stop background music |
| `set_music_volume(vol)` | Set music volume (0.0–1.0) |

## Spatial audio

When **Spatial** is enabled on an Audio Source, the sound is positioned in 3D space:

- **Closer** to the listener = louder
- **Left/right** panning based on direction
- **Rolloff** controls how quickly volume drops with distance

The listener is always the active camera. In multiplayer, each client hears spatial audio from their own camera's perspective.

### Spatial audio tips

- Set **Min Distance** to the size of the object emitting the sound (a bonfire might be 3m, a whisper 0.5m)
- Set **Max Distance** based on how far the sound should carry (footsteps: 20m, explosion: 200m)
- Use **Linear rolloff** for predictable falloff in enclosed spaces
- Use **Inverse rolloff** (default) for natural outdoor sound

## Audio buses

Sounds are routed through buses for grouped volume control:

| Bus | Description |
|-----|-------------|
| **Master** | Controls all audio |
| **SFX** | Sound effects (footsteps, explosions, UI clicks) |
| **Music** | Background music tracks |
| **Voice** | Dialogue, narration |
| **Ambient** | Environmental loops (wind, rain, crowd noise) |

Set bus volumes from scripts:

```rhai
// In a settings menu
ui_set_slider("sfx_slider", 0.8);
ui_set_slider("music_slider", 0.6);
```

## Example: ambient sound zone

Create an area that plays a sound when the player enters:

```rhai
fn on_ready() {
    // This entity has a sensor collider (trigger zone)
    set_volume("self", 0.0);  // start silent
}

fn on_update() {
    if active_collisions > 0 {
        // Player is inside the zone — fade in
        let vol = min(get_volume("self") + delta * 0.5, 1.0);
        set_volume("self", vol);
    } else {
        // Player left — fade out
        let vol = max(get_volume("self") - delta * 0.5, 0.0);
        set_volume("self", vol);
    }
}
```

## Tips

- **Preload** frequently-used sounds by placing Audio Source components with Play on Start disabled
- **OGG for music** — smaller file size, streams from disk
- **WAV for SFX** — no decode latency, instant playback
- **Keep sounds short** — sound effects should be 0.1–3 seconds. Longer audio should be music
- **Normalize audio** — ensure all sound effects are at similar loudness before importing
