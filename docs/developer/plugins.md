# Building Plugins

Extend Renzora Engine with custom functionality.

## What is a Plugin?

A plugin is a Bevy `Plugin` that adds new components, systems, or editor panels to the engine. Plugins are Rust crates that live in the `crates/` directory.

## Creating a Plugin

### 1. Create the crate

```bash
cargo new crates/core/renzora_myplugin --lib
```

### 2. Add dependencies

```toml
# crates/core/renzora_myplugin/Cargo.toml
[package]
name = "renzora_myplugin"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = { version = "0.18", default-features = false }
```

### 3. Implement the plugin

```rust
use bevy::prelude::*;

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyState>()
           .add_systems(Update, my_system);
    }
}

#[derive(Resource, Default)]
pub struct MyState {
    pub counter: u32,
}

fn my_system(mut state: ResMut<MyState>) {
    state.counter += 1;
}
```

### 4. Register in the runtime

Add to `renzora_runtime` or `renzora_editor` as a dependency, then call `app.add_plugins(MyPlugin)`.

## Adding an Editor Panel

```rust
use bevy::prelude::*;
use bevy_egui::egui;
use renzora_editor::EditorPanel;

pub struct MyPanel;

impl EditorPanel for MyPanel {
    fn name(&self) -> &'static str {
        "My Panel"
    }

    fn show(&mut self, ctx: &egui::Context, is_open: &mut bool, world: &World) {
        egui::Window::new("My Panel")
            .open(is_open)
            .show(ctx, |ui| {
                ui.label("Hello from my plugin!");
            });
    }
}
```

Register it:
```rust
app.register_panel::<MyPanel>();
```

## Adding Script Functions

Extend the scripting API using the `ScriptExtension` trait:

```rust
use renzora_scripting::{ScriptExtension, ExtensionData};

pub struct MyScriptExtension;

impl ScriptExtension for MyScriptExtension {
    fn register_rhai_functions(&self, engine: &mut rhai::Engine) {
        engine.register_fn("my_function", |x: f64| -> f64 {
            x * 2.0
        });
    }

    fn populate_context(&self, world: &World, entity: Entity, data: &mut ExtensionData) {
        // Inject per-entity data into scripts
    }
}
```

Now scripts can call `my_function(5.0)` and get `10.0`.

## Adding Custom Components

Make components serializable so they survive scene save/load:

```rust
#[derive(Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
```

Register the type:
```rust
app.register_type::<Health>();
```

## Adding Inspector UI

Implement a custom inspector for your component:

```rust
use renzora_editor::{InspectorEntry, InspectorRegistry};

fn register_inspector(mut registry: ResMut<InspectorRegistry>) {
    registry.register::<Health>(|ui, health| {
        ui.label("Health");
        ui.add(egui::Slider::new(&mut health.current, 0.0..=health.max));
    });
}
```

## Adding Post-Processing Effects

Use the `#[renzora_macros::post_process]` derive macro:

```rust
#[renzora_macros::post_process]
#[derive(Component)]
pub struct MyEffect {
    pub intensity: f32,
}
```

Write a WGSL fragment shader and the engine handles the rest.
