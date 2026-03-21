# Resources & State

Global singletons for data that isn't attached to any entity.

## What resources are

A resource is a single instance of a type, accessible by any system. Use resources for:

- Global game state (score, level, game phase)
- Configuration (settings, key bindings)
- Engine subsystem handles (script engine, network connection)
- Caches and indexes

## Defining resources

```rust
#[derive(Resource, Default)]
pub struct GameState {
    pub score: u32,
    pub level: u32,
    pub paused: bool,
}

#[derive(Resource)]
pub struct DifficultyConfig {
    pub enemy_speed: f32,
    pub spawn_rate: f32,
}
```

## Accessing in systems

```rust
// Read-only
fn show_score(state: Res<GameState>) {
    println!("Score: {}", state.score);
}

// Mutable
fn add_score(mut state: ResMut<GameState>) {
    state.score += 10;
}

// Optional (might not exist)
fn maybe_config(config: Option<Res<DifficultyConfig>>) {
    if let Some(config) = config {
        println!("Speed: {}", config.enemy_speed);
    }
}
```

## Initializing resources

```rust
// Use Default trait
app.init_resource::<GameState>();

// Explicit value
app.insert_resource(DifficultyConfig {
    enemy_speed: 3.0,
    spawn_rate: 2.0,
});
```

Resources can also be inserted at runtime via `Commands`:
```rust
commands.insert_resource(MyResource { ... });
commands.remove_resource::<MyResource>();
```

## Renzora's built-in resources

### EditorState

```rust
pub struct EditorState {
    pub selected_entity: Option<Entity>,
    pub play_mode: PlayMode,        // Editing, Playing, Paused
    pub active_tool: EditorTool,    // Select, Move, Rotate, Scale
    pub gizmo_space: GizmoSpace,    // Local, World
    pub snap_enabled: bool,
    pub snap_grid: f32,
}
```

### ProjectConfig

```rust
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub project_dir: PathBuf,
    pub assets_dir: PathBuf,
    pub scenes_dir: PathBuf,
    pub scripts_dir: PathBuf,
    pub default_scene: Option<String>,
}
```

### ScriptEngine

```rust
pub struct ScriptEngine {
    pub rhai_engine: rhai::Engine,
    pub lua_state: mlua::Lua,
    pub extensions: Vec<Box<dyn ScriptExtension>>,
}
```

Holds the Rhai and Lua runtimes. Script extensions register functions here.

### InputState

```rust
pub struct InputState {
    pub keys_pressed: HashSet<KeyCode>,
    pub keys_just_pressed: HashSet<KeyCode>,
    pub keys_just_released: HashSet<KeyCode>,
    pub mouse_position: Vec2,
    pub mouse_delta: Vec2,
    pub mouse_buttons: [bool; 3],
    pub gamepad: GamepadState,
}
```

Snapshot of input state, synced to scripts each frame.

### NetworkState

```rust
pub struct NetworkState {
    pub mode: NetworkMode,          // Offline, Server, Client
    pub connection_id: Option<u64>,
    pub peers: Vec<PeerInfo>,
    pub ping_ms: u32,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}
```

### AssetIndex

```rust
pub struct AssetIndex {
    pub loaded: HashMap<String, HandleUntyped>,
    pub pending: HashSet<String>,
    pub failed: Vec<(String, String)>,  // (path, error)
}
```

Registry of all loaded assets.

## Local resources

Per-system state that persists across frames but is private to one system:

```rust
fn count_frames(mut counter: Local<u32>) {
    *counter += 1;
    if *counter % 60 == 0 {
        println!("60 frames elapsed");
    }
}
```

`Local<T>` is initialized with `Default::default()` on first access.

## Non-Send resources

For types that can't be sent between threads (e.g., raw pointers, GPU handles):

```rust
app.insert_non_send_resource(MyGpuHandle { ... });

fn use_gpu(handle: NonSend<MyGpuHandle>) {
    // Runs on the main thread only
}
```

## Change detection

Check if a resource was modified:

```rust
fn on_score_change(score: Res<GameState>) {
    if score.is_changed() {
        println!("Score updated to {}", score.score);
    }
}
```

Or use a run condition:
```rust
app.add_systems(Update, on_score_change.run_if(resource_changed::<GameState>));
```
