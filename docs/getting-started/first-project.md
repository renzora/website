# Your First Project

Create a new project, add some objects to your scene, and hit play.

## Creating a project

When you launch Renzora, you'll see a project browser. Click **New Project**, give it a name, and choose a location on disk.

This creates a project folder with the following structure:

```
my-game/
├── project.toml      # project settings
├── scenes/
│   └── main.ron      # your startup scene
├── scripts/          # Lua/Rhai scripts
├── textures/         # images & sprites
├── audio/            # sound effects & music
└── materials/        # material graph files
```

## The project.toml file

This is your project's configuration:

```toml
[project]
name = "my-game"
version = "0.1.0"

[window]
resolution = [1280, 720]
fullscreen = false
title = "My Game"

[scene]
main = "scenes/main.ron"
```

## Adding objects to the scene

The editor opens with an empty scene. Let's add something:

1. Click the **+** button in the Hierarchy panel (left side)
2. Choose **3D → Cube** from the menu
3. A cube appears in the viewport. Use the gizmo to move it around.
4. Add a light: **+ → Light → Directional Light**

## Running your game

Press `F5` (or click the play button in the title bar) to enter play mode. The viewport switches to the game camera and your scripts start running.

Press `F5` again to stop and return to the editor.

> **Tip:** Use `Shift+F5` to run scripts without switching to the game camera. This is useful for testing script logic while still having editor controls.

## Saving

Press `Ctrl+S` to save your scene. Scenes are stored as `.ron` files in the `scenes/` folder.

## What's next?

Learn the editor interface in the [Editor Overview](/docs/getting-started/editor-overview), or jump straight to [Scripting](/docs/scripting/overview) to add game logic.
