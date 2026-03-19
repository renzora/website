# Editor Overview

A tour of the Renzora editor interface and its main panels.

## Layout

The editor uses a dockable panel system. You can drag panels, split them, and arrange them however you like. The default layout has:

- **Title bar** — file menu, workspace tabs, play controls, settings, and sign-in
- **Viewport** (center) — your 3D/2D scene view
- **Hierarchy** (left) — tree of all entities in the scene
- **Inspector** (right) — properties of the selected entity
- **Asset browser** (bottom) — browse and manage project files
- **Console** (bottom) — logs and script output

## Workspaces

The title bar has workspace tabs at the top. Each workspace is a different panel arrangement optimized for a task:

- **Scene** — default layout for level editing
- **Materials** — material graph editor with preview
- **Animation** — timeline and keyframe editor
- **Audio** — audio mixer (DAW-style)
- **UI** — game UI canvas editor
- **Network** — multiplayer configuration

You can create custom workspaces and save your own layouts.

## Key panels

### Viewport

The main scene view. Navigate with WASD to fly, Alt+left-click to orbit, and scroll to zoom. It automatically switches between 3D and 2D mode based on the selected entity.

### Hierarchy

Shows every entity in the scene as a tree. Drag entities to reparent them. Right-click for options like duplicate, delete, and rename. Use the + button to add new entities.

### Inspector

Shows all components on the selected entity: transform, mesh, material, physics, scripts, and more. Click "Add Component" to attach new functionality.

### Asset Browser

A file browser for your project. Drag textures onto materials, scripts onto entities, and scenes into the hierarchy. Supports thumbnails for images and models.

## What's next?

Learn about [Core Concepts](/docs/getting-started/concepts) like entities, components, and scenes.
