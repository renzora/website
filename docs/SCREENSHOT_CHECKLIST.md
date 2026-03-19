# Screenshot Checklist

Screenshots needed for the documentation. Save as PNG in `assets/images/docs/`.

## Editor Overview
- [ ] `editor-full.png` — Full editor with a scene loaded (hierarchy, viewport, inspector visible)
- [ ] `editor-workspaces.png` — Title bar showing workspace tabs
- [ ] `editor-hierarchy.png` — Hierarchy panel with nested entities
- [ ] `editor-inspector.png` — Inspector showing Transform + Physics + Script components
- [ ] `editor-add-entity.png` — The "+" button menu showing entity types

## Viewport
- [ ] `viewport-gizmo-translate.png` — Translation gizmo on a selected cube
- [ ] `viewport-gizmo-rotate.png` — Rotation gizmo
- [ ] `viewport-gizmo-scale.png` — Scale gizmo
- [ ] `viewport-wireframe.png` — Wireframe mode enabled
- [ ] `viewport-2d-mode.png` — Viewport in 2D mode with sprites

## Materials
- [ ] `material-editor.png` — Material graph editor with nodes connected
- [ ] `material-preview.png` — Material preview on a sphere
- [ ] `material-nodes.png` — Node palette showing available nodes

## Terrain
- [ ] `terrain-sculpt.png` — Terrain with sculpting brush active
- [ ] `terrain-paint.png` — Terrain with paint layers visible
- [ ] `terrain-inspector.png` — Terrain settings in inspector

## Animation
- [ ] `animation-timeline.png` — Animation timeline with keyframes
- [ ] `animation-state-machine.png` — State machine graph (if visible)

## Blueprint / Visual Scripting
- [ ] `blueprint-editor.png` — Blueprint graph with connected nodes
- [ ] `blueprint-node-palette.png` — Node category list

## Audio
- [ ] `mixer-panel.png` — Audio mixer with channel strips and VU meters
- [ ] `audio-inspector.png` — AudioPlayer component in inspector

## Script Editor
- [ ] `code-editor.png` — Rhai script in the code editor with syntax highlighting
- [ ] `script-props.png` — Script properties displayed in inspector

## Physics
- [ ] `physics-colliders.png` — Scene with visible collider wireframes
- [ ] `physics-inspector.png` — Physics body + collision shape in inspector

## Export
- [ ] `export-dialog.png` — Export overlay showing platform selection

## Game UI
- [ ] `game-ui-canvas.png` — UI canvas editor with widgets placed
- [ ] `game-ui-preview.png` — Game running with HUD visible

## Networking
- [ ] `network-editor.png` — Network status panel

## Project
- [ ] `project-browser.png` — Project browser / launcher screen
- [ ] `asset-browser.png` — Asset browser with thumbnails

## How to Capture
1. Open the engine: `cargo run --release`
2. Load a demo scene or create one with various entity types
3. Use Windows Snipping Tool (`Win+Shift+S`) or `Alt+PrintScreen`
4. Save to `F:\website\assets\images\docs\` with the filename above
5. Crop to show just the relevant area (no window chrome)
6. Aim for ~1200px wide for full-panel shots, ~600px for close-ups
