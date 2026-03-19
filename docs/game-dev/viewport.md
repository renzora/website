# Viewport & Camera

Navigate the 3D scene and control the editor camera.

## Navigation

| Input | Action |
|-------|--------|
| **Right-click + WASD/QE** | Fly through the scene (W forward, S back, A left, D right, E up, Q down) |
| **Shift** (while flying) | Move faster |
| **Alt + Left-click drag** | Orbit around the focus point |
| **Middle-click drag** | Pan the camera |
| **Scroll wheel** | Zoom in/out |
| **F** | Focus camera on selected entity |

The camera speed scales with distance — close to an object it moves slowly, far away it moves quickly.

## View Presets

Quick camera angle shortcuts using the numpad:

| Key | View |
|-----|------|
| `Numpad 1` | Front |
| `Numpad 3` | Right |
| `Numpad 7` | Top |
| `Ctrl+Numpad 1` | Back |
| `Ctrl+Numpad 3` | Left |
| `Ctrl+Numpad 7` | Bottom |
| `Numpad 5` | Toggle perspective / orthographic |

## Display Options

| Key | Toggle |
|-----|--------|
| `Z` | Wireframe mode |
| `Shift+Z` | Lighting on/off |
| `H` | Grid on/off |

## 2D Mode

The viewport automatically switches to a 2D orthographic view when you select a 2D entity (sprite, tilemap, UI canvas). In 2D mode, the camera is locked to a top-down view and scroll zooms in/out.

## Gizmos

The transform gizmo appears when an entity is selected:
- **Colored arrows** — drag to move along one axis
- **Colored planes** — drag to move on a plane
- **Circles** (rotate mode) — drag to rotate around an axis
- **Handles** (scale mode) — drag to scale along an axis

The gizmo automatically sizes itself based on camera distance so it's always usable.

## Selection

- **Left-click** — select entity under cursor
- **Ctrl+click** — toggle selection
- **Shift+click** — add to selection
- **Click empty space** — deselect all

Selected entities show an outline highlight in the viewport.
