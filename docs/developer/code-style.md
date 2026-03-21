# Code Style

Coding conventions for Renzora Engine contributors.

## Rust style

Follow **rustfmt** defaults. Run before every commit:

```bash
cargo fmt --all
```

The CI pipeline rejects unformatted code.

## Naming conventions

| Item | Convention | Example |
|------|-----------|---------|
| Functions, methods | `snake_case` | `spawn_entity()` |
| Variables, fields | `snake_case` | `player_health` |
| Types, traits, enums | `PascalCase` | `PhysicsPlugin`, `EditorPanel` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_PLAYERS` |
| Modules, files | `snake_case` | `asset_pipeline.rs` |
| Crates | `snake_case` with `renzora_` prefix | `renzora_physics` |

## Module organization

```
crates/core/renzora_physics/src/
├── lib.rs          # Plugin struct, public API, re-exports
├── components.rs   # Component definitions
├── systems.rs      # System functions
├── resources.rs    # Resource definitions
└── events.rs       # Event definitions
```

- `lib.rs` exports the public API. Internal modules are `pub(crate)`.
- One concept per file. Split large files by responsibility.
- Keep `use` imports organized: std → external crates → internal modules.

## Error handling

| Context | Approach |
|---------|----------|
| Application code (runtime, editor) | `anyhow::Result` |
| Library crates | `thiserror` with custom error types |
| Systems | Log errors, don't panic. Use `warn!()` or `error!()` |
| Infallible operations | `.expect("reason")` with a clear message |

```rust
// Good: clear error context
let file = std::fs::read_to_string(&path)
    .with_context(|| format!("Failed to read scene file: {}", path.display()))?;

// Good: log and continue
if let Err(e) = save_scene(&scene) {
    error!("Failed to save scene: {e}");
}

// Avoid: unwrap without context
let file = std::fs::read_to_string(&path).unwrap(); // Don't do this
```

## Documentation

- `///` doc comments on all public items
- First line is a brief summary (shows in IDE hover)
- Skip internal/private items unless the logic is non-obvious

```rust
/// Synchronizes physics body transforms with Bevy's Transform component.
///
/// Runs after the physics step to update visual positions. Handles
/// interpolation when the physics tick rate differs from the frame rate.
fn sync_physics_transforms(
    mut query: Query<(&mut Transform, &RigidBody)>,
) { ... }
```

## Unsafe code

- Avoid `unsafe` unless absolutely necessary (FFI, GPU interop)
- Every `unsafe` block must have a `// SAFETY:` comment explaining the invariant
- Prefer safe abstractions over `unsafe` callers

```rust
// SAFETY: The raw window handle outlives the surface because the window
// is owned by the App and the surface is dropped in Plugin::cleanup.
unsafe { instance.create_surface(&window) }
```

## Clippy lints

CI runs clippy with warnings as errors:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Don't `#[allow]` lints without justification.

## Commit messages

Format: `<type>: <summary>`

| Type | For |
|------|-----|
| `feat` | New feature |
| `fix` | Bug fix |
| `refactor` | Code restructuring (no behavior change) |
| `docs` | Documentation only |
| `test` | Test additions or fixes |
| `ci` | CI/CD changes |
| `chore` | Dependencies, tooling, config |

Examples:
```
feat: add terrain LOD system
fix: prevent crash when loading empty scenes
refactor: split physics module into components and systems
```

## PR guidelines

- One logical change per PR
- Title matches commit format: `type: summary`
- Description explains **why**, not just what
- Add tests for new features and bug fixes
- Request review from a crate owner
- Squash-merge to main
