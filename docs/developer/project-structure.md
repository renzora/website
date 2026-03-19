# Project Structure

Understanding how the Renzora Engine codebase is organized.

## Repository Layout

```
engine/
├── Cargo.toml                 # Workspace root
├── Makefile.toml              # Build tasks (cargo-make)
├── src/
│   ├── editor.rs              # Editor binary entry point
│   └── runtime.rs             # Runtime library entry point
├── crates/
│   ├── core/                  # Engine runtime crates
│   │   ├── renzora_core/      # Project config, components, play state
│   │   ├── renzora_runtime/   # Scene loading, asset reader, camera
│   │   ├── renzora_assets/    # Asset resolver, hot reload, watcher
│   │   ├── renzora_scripting/ # Rhai/Lua interpreter, script commands
│   │   ├── renzora_blueprint/ # Visual scripting graph + interpreter
│   │   ├── renzora_physics/   # Avian3D wrapper, rigid bodies, colliders
│   │   ├── renzora_audio/     # Kira audio, spatial, mixer
│   │   ├── renzora_animation/ # Skeletal animation, state machines, tweening
│   │   ├── renzora_game_ui/   # bevy_ui game interface (19 widget types)
│   │   ├── renzora_material/  # Material graph, WGSL shader generation
│   │   ├── renzora_terrain/   # Heightmap sculpting, splatmap painting
│   │   ├── renzora_network/   # Lightyear 0.26 multiplayer
│   │   ├── renzora_lifecycle/ # Project boot sequence graph
│   │   ├── renzora_lighting/  # Sun component, directional light
│   │   ├── renzora_import/    # GLTF/GLB import pipeline
│   │   ├── renzora_rpak/      # Asset archive format
│   │   └── ...                # Water, RT, shader utils
│   ├── editor/                # Editor-only crates (~27 panels)
│   │   ├── renzora_editor/    # Editor shell, plugin system
│   │   ├── renzora_hierarchy/ # Entity tree panel
│   │   ├── renzora_inspector/ # Properties panel
│   │   ├── renzora_viewport/  # 3D/2D viewport
│   │   ├── renzora_gizmo/     # Transform manipulation
│   │   ├── renzora_export/    # Build packaging
│   │   ├── renzora_keybindings/ # Keyboard shortcuts
│   │   ├── renzora_auth/      # Website auth + marketplace
│   │   └── ...                # 20+ more panels
│   ├── ui/                    # UI framework
│   │   ├── renzora_ui/        # Docking, panels, widgets
│   │   ├── renzora_theme/     # Color theming
│   │   └── renzora_splash/    # Splash screen
│   └── postprocessing/        # 40+ post-process effects
│       ├── renzora_bloom/
│       ├── renzora_dof/
│       ├── renzora_ssao/
│       └── ...
├── assets/                    # Engine default assets
├── docs/                      # Engine documentation (markdown)
└── _legacy_src/               # Legacy code (reference only)
```

## Key Crates

### Core vs Editor

**Core crates** (`crates/core/`) run in both the editor and the exported game. They contain the runtime engine — physics, audio, scripting, rendering.

**Editor crates** (`crates/editor/`) only run in the editor. They contain UI panels, tools, and workflows. They're excluded from exported builds.

### Entry Points

- **`src/editor.rs`** — the editor binary. Sets up Bevy with the editor plugin, which adds all panels and tools.
- **`src/runtime.rs`** — the runtime library. Used by export templates to run the game without the editor.

### Build System

The engine uses **cargo-make** (`Makefile.toml`) for common tasks:

```bash
cargo make run          # Run the editor
cargo make build        # Build release
cargo make check        # Type check all crates
```

## Adding a New Crate

1. Create the crate: `cargo new crates/core/renzora_myfeature --lib`
2. Add it to the workspace `Cargo.toml` members list
3. Add it as a dependency to `renzora_runtime` (if core) or `renzora_editor` (if editor-only)
4. Implement the feature using Bevy's plugin system
