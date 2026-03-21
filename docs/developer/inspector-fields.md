# Custom Inspector Fields

Define how your components appear in the Inspector panel.

## How the Inspector works

When you select an entity, the Inspector shows all its components with editable fields. The `InspectorRegistry` maps component types to rendering functions.

Built-in types have default renderers:

| Rust type | Inspector widget |
|-----------|-----------------|
| `f32`, `f64` | Drag value / slider |
| `i32`, `u32`, etc. | Integer drag value |
| `bool` | Checkbox |
| `String` | Text input |
| `Vec3` | XYZ drag values (colored) |
| `Vec2` | XY drag values |
| `Color` | Color picker with hex input |
| `Entity` | Entity picker dropdown |
| `Handle<Image>` | Texture preview + file picker |
| `Handle<Mesh>` | Mesh name + file picker |
| Enums | Dropdown selector |

## Registering custom inspectors

For your own components, register a UI function:

```rust
use renzora_editor::{InspectorRegistry, InspectorEntry};

fn register_my_inspectors(mut registry: ResMut<InspectorRegistry>) {
    registry.register::<Health>(|ui, health| {
        ui.label("Health");
        ui.add(egui::Slider::new(&mut health.current, 0.0..=health.max)
            .text("Current"));
        ui.add(egui::DragValue::new(&mut health.max)
            .range(1.0..=10000.0)
            .prefix("Max: "));
    });
}
```

Add the system to your plugin:
```rust
app.add_systems(Startup, register_my_inspectors);
```

## Enum rendering

Enums with `#[derive(Reflect)]` render as dropdowns automatically:

```rust
#[derive(Component, Reflect, Default, Clone, PartialEq)]
#[reflect(Component)]
pub enum Team {
    #[default]
    Neutral,
    Red,
    Blue,
    Green,
}
```

For custom enum UI:

```rust
registry.register::<Team>(|ui, team| {
    egui::ComboBox::from_label("Team")
        .selected_text(format!("{:?}", team))
        .show_ui(ui, |ui| {
            ui.selectable_value(team, Team::Neutral, "Neutral");
            ui.selectable_value(team, Team::Red, "🔴 Red");
            ui.selectable_value(team, Team::Blue, "🔵 Blue");
            ui.selectable_value(team, Team::Green, "🟢 Green");
        });
});
```

## Nested struct rendering

For components with nested structs, render each field:

```rust
#[derive(Component)]
pub struct PhysicsConfig {
    pub gravity_scale: f32,
    pub damping: DampingConfig,
    pub constraints: AxisConstraints,
}

pub struct DampingConfig {
    pub linear: f32,
    pub angular: f32,
}

registry.register::<PhysicsConfig>(|ui, config| {
    ui.add(egui::Slider::new(&mut config.gravity_scale, 0.0..=10.0)
        .text("Gravity Scale"));

    egui::CollapsingHeader::new("Damping").show(ui, |ui| {
        ui.add(egui::Slider::new(&mut config.damping.linear, 0.0..=10.0)
            .text("Linear"));
        ui.add(egui::Slider::new(&mut config.damping.angular, 0.0..=10.0)
            .text("Angular"));
    });

    egui::CollapsingHeader::new("Constraints").show(ui, |ui| {
        ui.checkbox(&mut config.constraints.lock_x, "Lock X");
        ui.checkbox(&mut config.constraints.lock_y, "Lock Y");
        ui.checkbox(&mut config.constraints.lock_z, "Lock Z");
    });
});
```

## Read-only fields

Display information without allowing edits:

```rust
registry.register::<NetworkIdentity>(|ui, net_id| {
    ui.label("Network Identity");
    ui.horizontal(|ui| {
        ui.label("Net ID:");
        ui.label(format!("{}", net_id.id));  // no &mut, no editor
    });
    ui.horizontal(|ui| {
        ui.label("Owner:");
        ui.label(format!("{:?}", net_id.owner));
    });
});
```

## Validation

Add visual feedback for invalid values:

```rust
registry.register::<SpawnConfig>(|ui, config| {
    ui.add(egui::DragValue::new(&mut config.spawn_rate).range(0.1..=100.0));

    if config.max_entities == 0 {
        ui.colored_label(egui::Color32::RED, "⚠ max_entities cannot be 0");
    }
    ui.add(egui::DragValue::new(&mut config.max_entities).range(0..=10000));

    if config.spawn_rate > 50.0 {
        ui.colored_label(egui::Color32::YELLOW, "High spawn rate may affect performance");
    }
});
```

## Example: RPG stats inspector

```rust
#[derive(Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct RpgStats {
    pub level: u32,
    pub experience: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub vitality: u32,
    pub points_available: u32,
}

registry.register::<RpgStats>(|ui, stats| {
    ui.horizontal(|ui| {
        ui.label(format!("Level {} — {} XP", stats.level, stats.experience));
    });
    ui.separator();

    let xp_to_next = stats.level * 100;
    ui.add(egui::ProgressBar::new(stats.experience as f32 / xp_to_next as f32)
        .text(format!("{}/{} XP", stats.experience, xp_to_next)));

    ui.separator();
    ui.label(format!("Points available: {}", stats.points_available));

    let stats_list = [
        ("Strength", &mut stats.strength),
        ("Dexterity", &mut stats.dexterity),
        ("Intelligence", &mut stats.intelligence),
        ("Vitality", &mut stats.vitality),
    ];

    for (name, value) in stats_list {
        ui.horizontal(|ui| {
            ui.label(format!("{}: {}", name, *value));
            if stats.points_available > 0 && ui.small_button("+").clicked() {
                *value += 1;
                stats.points_available -= 1;
            }
        });
    }
});
```
