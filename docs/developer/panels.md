# Building Editor Panels

Add custom panels to the Renzora editor.

## The EditorPanel trait

Every editor panel implements this trait:

```rust
pub trait EditorPanel: Send + Sync {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool, world: &World);
}
```

- `name()` — displayed in the Window menu and panel tab
- `show()` — called every frame when the panel is visible
- `world` — read-only access to the ECS world

## Creating a panel

```rust
use bevy::prelude::*;
use bevy_egui::egui;
use renzora_editor::EditorPanel;

pub struct DebugStatsPanel {
    show_fps: bool,
    show_entities: bool,
}

impl Default for DebugStatsPanel {
    fn default() -> Self {
        Self { show_fps: true, show_entities: true }
    }
}

impl EditorPanel for DebugStatsPanel {
    fn name(&self) -> &'static str {
        "Debug Stats"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool, world: &World) {
        egui::Window::new("Debug Stats")
            .open(is_open)
            .show(ctx, |ui| {
                ui.checkbox(&mut self.show_fps, "Show FPS");
                ui.checkbox(&mut self.show_entities, "Show Entity Count");

                ui.separator();

                if self.show_fps {
                    let time = world.resource::<Time>();
                    ui.label(format!("FPS: {:.0}", 1.0 / time.delta_secs()));
                }

                if self.show_entities {
                    let count = world.entities().len();
                    ui.label(format!("Entities: {}", count));
                }
            });
    }
}
```

## Registering panels

In your plugin's `build()`:

```rust
app.register_panel::<DebugStatsPanel>();
```

The panel appears in **Window → Debug Stats**. Users can dock it anywhere in the editor layout.

## Accessing world data

The `world` parameter gives read-only access:

```rust
// Read resources
let state = world.resource::<EditorState>();
let selected = state.selected_entity;

// Query entities
let mut query = world.query::<(&Name, &Transform)>();
for (name, transform) in query.iter(world) {
    // ...
}

// Get a specific entity's components
if let Some(entity) = selected {
    if let Some(health) = world.get::<Health>(entity) {
        ui.label(format!("Health: {}", health.current));
    }
}
```

## Modifying the world

Panels can't mutate the world directly (it's a shared `&World`). Use `EditorCommands`:

```rust
impl EditorPanel for MyPanel {
    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool, world: &World) {
        let mut cmds = world.resource::<EditorCommands>().clone();

        egui::Window::new("My Panel").open(is_open).show(ctx, |ui| {
            if ui.button("Spawn Cube").clicked() {
                cmds.push(|world: &mut World| {
                    world.spawn(PbrBundle {
                        mesh: world.resource::<DefaultMeshes>().cube.clone(),
                        ..default()
                    });
                });
            }

            if ui.button("Delete Selected").clicked() {
                if let Some(entity) = world.resource::<EditorState>().selected_entity {
                    cmds.push(move |world: &mut World| {
                        world.despawn(entity);
                    });
                }
            }
        });
    }
}
```

`EditorCommands` are executed at the end of the frame, avoiding borrow conflicts.

## Panel state persistence

Panel state (position, size, open/closed) is saved automatically by the docking system. Your panel's fields are **not** persisted by default.

To persist custom state, implement `Serialize`/`Deserialize` on your panel and register it with the persistence system:

```rust
#[derive(Serialize, Deserialize)]
pub struct MyPanel {
    pub filter: String,
    pub sort_ascending: bool,
}

app.register_panel_with_persistence::<MyPanel>();
```

## Example: entity list panel

```rust
pub struct EntityListPanel {
    filter: String,
}

impl EditorPanel for EntityListPanel {
    fn name(&self) -> &'static str { "Entity List" }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool, world: &World) {
        egui::Window::new("Entity List").open(is_open).show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.filter);
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut query = world.query::<(Entity, &Name)>();
                for (entity, name) in query.iter(world) {
                    let name_str = name.as_str();
                    if !self.filter.is_empty() && !name_str.contains(&self.filter) {
                        continue;
                    }
                    let selected = world.resource::<EditorState>().selected_entity == Some(entity);
                    if ui.selectable_label(selected, name_str).clicked() {
                        let mut cmds = world.resource::<EditorCommands>().clone();
                        cmds.push(move |world: &mut World| {
                            world.resource_mut::<EditorState>().selected_entity = Some(entity);
                        });
                    }
                }
            });
        });
    }
}
```
