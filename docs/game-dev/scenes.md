# Scenes & Hierarchy

Organize your game world with entities and the scene hierarchy.

## The Hierarchy Panel

The Hierarchy panel (left side of the editor) shows every entity in the current scene as a tree. Entities can be nested inside other entities to create parent-child relationships.

## Adding Entities

Click the **+** button at the top of the Hierarchy to add new entities:

### 3D Objects
- **Cube** — standard box primitive
- **Sphere** — UV sphere
- **Plane** — flat ground surface
- **Cylinder** — cylindrical shape
- **Capsule** — pill shape (useful for character colliders)
- **Cone** — tapered shape
- **Torus** — donut shape

### 2D Objects
- **Sprite** — 2D image/texture
- **TileMap** — grid-based level

### Lights
- **Directional Light** — sun-like light with shadows
- **Point Light** — light radiating from a point
- **Spot Light** — cone-shaped light

### Other
- **Camera** — 3D or 2D camera
- **Audio Emitter** — sound source
- **Audio Listener** — microphone (attach to player/camera)
- **Empty** — empty container entity (use as organizer)

## Selecting and Transforming

Click an entity in the Hierarchy or viewport to select it. The transform gizmo appears:

| Key | Tool | Description |
|-----|------|-------------|
| `W` | Translate | Move the entity |
| `E` | Rotate | Rotate the entity |
| `R` | Scale | Resize the entity |
| `Q` | Select | No gizmo, click to select |

### Blender-Style Modal Transforms

Press these keys to start a modal transform, then move the mouse:

| Key | Action |
|-----|--------|
| `G` | Grab/Move |
| `R` | Rotate |
| `S` | Scale |
| `X/Y/Z` | Constrain to axis (after G/R/S) |
| `Escape` | Cancel |
| `Enter` | Confirm |

Type a number after pressing G/R/S to input a precise value.

## Parenting

Drag an entity onto another in the Hierarchy to make it a child. Children inherit their parent's transform — moving the parent moves all children.

**Common uses:**
- Attach a weapon to a character's hand
- Group building parts together
- Create pivot points for rotation
- Organize scenes with empty parent entities

## Multi-Selection

| Input | Action |
|-------|--------|
| `Ctrl+Click` | Toggle entity in selection |
| `Shift+Click` | Add to selection |
| `Escape` | Deselect all |

## Scene Operations

| Shortcut | Action |
|----------|--------|
| `Ctrl+N` | New scene |
| `Ctrl+O` | Open scene |
| `Ctrl+S` | Save scene |
| `Ctrl+Shift+S` | Save scene as... |
| `Ctrl+D` | Duplicate selected |
| `Delete` | Delete selected |
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |

## Scene File Format

Scenes are saved as `.ron` (Rusty Object Notation) files in your project's `scenes/` folder. They contain:

- Entity hierarchy with parent-child relationships
- All component data (transforms, physics, materials, scripts)
- Asset references as relative paths

The startup scene is defined in `project.toml`:

```toml
[scene]
main = "scenes/main.ron"
```
