# Core Concepts

The fundamental building blocks of Renzora — entities, components, scenes, and scripts.

## Entities

An entity is a thing in your game world. A character, a light, a tree, a camera — they're all entities. By themselves, entities are empty containers. They only gain behavior through components.

## Components

Components are data attached to entities. A **Transform** component gives an entity a position, rotation, and scale. A **Mesh** component gives it a 3D shape. A **Rigid Body** gives it physics.

You build game objects by combining components. A player character might have:

- Transform — position in the world
- Mesh — the 3D model
- Material — the visual appearance
- Rigid Body — physics simulation
- Collider — collision detection
- Script — game logic (movement, health, etc.)

## Scenes

A scene is a collection of entities saved as a `.ron` file. Your game can have multiple scenes — a main menu, a gameplay level, a settings screen.

The startup scene is defined in `project.toml` and loads automatically when the game runs.

## Scripts

Scripts add custom behavior to entities. Renzora supports three scripting approaches:

- **Rhai** — a lightweight scripting language designed for Rust. Great for gameplay logic.
- **Lua** — the industry-standard game scripting language. Familiar to most game developers.
- **Blueprints** — visual node graphs for logic. No coding required.

Scripts run two key functions: `on_ready()` when the entity spawns, and `on_update()` every frame.

## Materials

Materials define how surfaces look. Renzora uses a node-based material editor — you connect texture nodes, math nodes, and shader properties in a visual graph. Materials are saved as `.material` files.

## The game loop

Every frame, the engine:

1. Processes input (keyboard, mouse, gamepad)
2. Runs scripts (`on_update` on every active script)
3. Steps the physics simulation
4. Updates transforms and animations
5. Renders the frame

You don't need to manage this loop yourself — just write your script logic and the engine handles the rest.

## What's next?

Dive into the [Editor Guide](/docs/editor/scenes) to learn the tools, or start [Scripting](/docs/scripting/overview) to add game logic.
