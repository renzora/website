# Loading Assets

By default a game loads assets when something first needs them. A sprite's image loads when the sprite appears, and a sound is decoded the first time it plays. You don't have to do anything for that to work.

Sometimes you want to decide **when** loading happens:

- A loading screen that loads what the first level needs and shows real progress.
- A title sequence that loads the next scene in the background.
- A transition that waits until the next scene's assets are in before it switches.
- An area that loads as the player gets near it and unloads after they leave.

For these, put the assets in a **load group**.

## Load groups

A load group is a named set of paths. The engine loads a group when you tell it to, reports how far it has got, and keeps its assets loaded until you unload it. A path can be:

| Path | Stands for |
|---|---|
| `art/ui/logo.png` | that file |
| `audio/music/` | every file in the folder and its subfolders (end the path with `/`), except files that aren't assets: `.md`, `.txt`, `.toml`, scripts, sidecars and hidden files |
| `scenes/level1.bsn` | the scene and every asset it references, including the prefabs it uses and their assets |

Scenes are read for their references without being spawned. That means you can preload the next level while the current one is playing.

Sounds in a group are decoded up front. Their first play then has no decoding hitch.

A group moves through these states:

```text
idle ─start─► loading ─► ready ─(scene switched)─► activated
               │  ▲
           pause  resume          stop ─► stopped ─start─► loading
               ▼  │
             paused               unload (from any state): released and forgotten
```

- **pause**: loads already in progress finish, but nothing new starts until you resume.
- **stop**: whatever hasn't finished is dropped. Whatever has finished stays loaded. Calling start again loads the rest.
- **add paths** to a group that is loading or ready, and the new paths start loading straight away.
- **unload**: the group releases its assets. Anything the scene is still using stays loaded.

A file that fails to load counts as done, so a missing file never holds a loading screen up forever. Failures are counted separately.

## From a Rust script

The groups live in the `AssetLoads` resource at `renzora::assets`.

```rust
use renzora::assets::{AssetLoads, SceneActivation};

fn on_ready(world: &mut World) {
    let mut loads = world.resource_mut::<AssetLoads>();
    loads.preload("boot", &["art/ui/", "audio/music/theme.ogg", "scenes/level1.bsn"]);
}

fn on_update(world: &mut World) {
    let loads = world.resource::<AssetLoads>();
    if let Some(p) = loads.progress("boot") {
        // p.fraction() is 0..1, by bytes when sizes are known.
        // p.loaded, p.failed and p.total are counts. p.current is the file
        // being loaded. p.state is the group's state.
    }
    if loads.is_ready("boot") { /* show "Press Start" */ }
}
```

| Method | Does |
|---|---|
| `create(name)` | Makes an empty group, idle until started |
| `append(name, &paths)` | Adds paths, making the group if needed |
| `start(name)` / `pause(name)` / `resume(name)` / `stop(name)` | Controls loading |
| `unload(name)` | Releases the group and forgets it |
| `preload(name, &paths)` | Create, append and start in one call |
| `load_scene(name, path, activation)` | Loads a scene's assets, then switches to that scene |
| `activate(name)` | Switches a `Manual` scene load to its scene, as soon as its assets are in |
| `progress(name)`, `fraction(name)`, `is_ready(name)`, `groups()` | Read how loading is going |

Requests take effect at the start of the next frame. A group's state changes one frame after you ask.

## Loading a scene with progress

```rust
// Title screen: start loading level 1 while the title animates.
loads.load_scene("level1", "scenes/level1.bsn", SceneActivation::Manual);

// When the player presses Start, switch: right away if the assets are in,
// otherwise as soon as they are.
loads.activate("level1");
```

`SceneActivation::WhenReady` switches by itself once the assets are in. `Manual` waits for `activate`.

## From any script language

The same operations are available as script functions. Lists of paths are separated by commas.

```lua
function on_ready()
    preload("boot", "art/ui/, audio/music/theme.ogg, scenes/level1.bsn")
end

function on_update()
    local p = load_progress("boot")      -- 0..1
    set_text("bar_label", math.floor(p * 100) .. "%")
    if load_ready("boot") then
        preload_scene("level1", "scenes/level1.bsn", true)
    end
end
```

| Function | Does |
|---|---|
| `preload(group, paths)` | Makes a group, fills it and starts it |
| `load_add(group, paths)` | Adds paths to a group |
| `load_create(group)` | Makes an empty group |
| `load_start` / `load_pause` / `load_resume` / `load_stop` `(group)` | Controls loading |
| `load_unload(group)` | Releases a group |
| `preload_scene(group, path, wait)` | Loads a scene's assets, then switches to it. With `wait` true, it switches only once `activate_scene` is called |
| `activate_scene(group)` | Switches to a waiting scene |
| `load_progress(group)` | 0 to 1 |
| `load_ready(group)` | true once everything has loaded or failed |
| `load_state(group)` | `idle`, `loading`, `paused`, `stopped`, `ready` or `activated` |
| `load_loaded` / `load_total` / `load_failed` `(group)` | Counts |

`load_scene(path)`, which switches scenes immediately, is unchanged.

## Loading by distance

A **Load Zone** component does dynamic loading for you:

1. When a **Load Zone Target** gets within `radius` of the zone, the zone's paths load as a group.
2. When every target is further away than `unload_radius`, the group unloads.

Put the target on the player. If nothing in the scene has one, the active camera is used.

| Field | Meaning |
|---|---|
| `group` | The group's name. Leave it empty for a name of its own. Zones with the same name share one group, which unloads only when none of them holds it |
| `paths` | Files, folders or scenes, separated by commas |
| `radius` | How close a target must come for the group to load |
| `unload_radius` | How far away every target must be for the group to unload. Keep it bigger than `radius`, so a player standing on the edge doesn't load and unload it over and over |

A zone placed at a doorway can name the next room's scene. Its assets are then in by the time the player walks through. Zones only work in a running game; the editor already loads everything it shows.

## For engine crates

A kind of asset that Bevy's asset server doesn't load can join load groups too. Its crate registers an `AssetKindHandler` in `AssetKindHandlers`, which claims file extensions and loads and unloads them. Audio uses this to decode sounds into the audio backend. A handler is called at most once per frame, because it may load synchronously.
