# Architecture

How Renzora Engine is built on top of Bevy ECS.

## Foundation: Bevy 0.18

Renzora is built on [Bevy](https://bevyengine.org/), a data-driven game engine written in Rust. Everything in Renzora uses Bevy's **Entity Component System (ECS)**:

- **Entities** — unique IDs that hold components
- **Components** — data attached to entities (position, mesh, physics, etc.)
- **Systems** — functions that run on entities with specific component combinations
- **Resources** — global shared state (not attached to entities)

## Plugin Architecture

Every feature in Renzora is a **Bevy Plugin**:

```rust
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AvianPlugin::default())
           .add_systems(Update, sync_physics_data);
    }
}
```

The editor and runtime both build an `App` and add the plugins they need. The editor adds UI panels; the runtime skips them.

## Data Flow

```
Input → Scripts → Commands → Physics → Transform → Render
         ↓
    Blueprint Interpreter
         ↓
    ScriptCommandQueue → System Handlers
```

1. **Input** collected from keyboard/mouse/gamepad
2. **Scripts** and **Blueprints** run, producing `ScriptCommand`s
3. **Command handlers** process commands (spawn, move, play sound, etc.)
4. **Physics** steps the simulation
5. **Transforms** update from physics results
6. **Renderer** draws the frame

## Key Design Patterns

### EditorPanel Trait

Every editor panel implements this trait:

```rust
pub trait EditorPanel: Send + Sync {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool, world: &World);
}
```

Panels are registered via `app.register_panel()` and managed by the docking system.

### ScriptExtension Trait

Crates extend the scripting API without modifying the scripting crate:

```rust
pub trait ScriptExtension: Send + Sync {
    fn populate_context(&self, world: &World, entity: Entity, data: &mut ExtensionData);
    fn register_rhai_functions(&self, engine: &mut rhai::Engine);
}
```

Audio, physics, UI, animation, and networking all use this to add their functions to scripts.

### EditorCommands

Deferred world mutations from the editor:

```rust
cmds.push(move |world: &mut World| {
    world.spawn(PbrBundle { ... });
});
```

This avoids borrow conflicts when modifying the world during UI rendering.

### PostProcessEffect Trait

Unified post-processing pipeline:

```rust
pub trait PostProcessEffect: Component + ExtractComponent {
    fn fragment_shader() -> ShaderRef;
}
```

40+ effects use this trait. Inactive effects have zero overhead.

## Scene Serialization

Scenes are saved as `.ron` (Rusty Object Notation) files. The engine:

1. **Denies** non-serializable components during save
2. **Stores** asset paths as relative (e.g., `textures/brick.png`)
3. **Rehydrates** on load — rebuilds meshes, materials, and physics from stored data

This ensures scenes are portable across machines and project locations.

## Asset Resolution

Assets are resolved through a priority chain:

1. Absolute path (pass-through)
2. `.rpak` archive (packed builds)
3. Project `assets/` directory
4. Exe-adjacent `assets/` directory
5. CWD `assets/` directory

This allows hot-reload in the editor while supporting packed distribution.
