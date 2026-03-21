# Custom Widgets

Build reusable UI components for editor panels.

## egui widget basics

Renzora's editor uses [egui](https://github.com/emilk/egui). Widgets are added to a `Ui`:

```rust
ui.label("Hello");
ui.button("Click me");
ui.text_edit_singleline(&mut my_string);
ui.add(egui::Slider::new(&mut value, 0.0..=100.0));
ui.checkbox(&mut enabled, "Enable feature");
```

## The Widget trait

Create reusable widgets by implementing `egui::Widget`:

```rust
pub struct Vec3Editor<'a> {
    value: &'a mut Vec3,
    label: &'a str,
}

impl<'a> Vec3Editor<'a> {
    pub fn new(value: &'a mut Vec3, label: &'a str) -> Self {
        Self { value, label }
    }
}

impl<'a> egui::Widget for Vec3Editor<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label(self.label);
            ui.colored_label(egui::Color32::RED, "X");
            ui.add(egui::DragValue::new(&mut self.value.x).speed(0.1));
            ui.colored_label(egui::Color32::GREEN, "Y");
            ui.add(egui::DragValue::new(&mut self.value.y).speed(0.1));
            ui.colored_label(egui::Color32::LIGHT_BLUE, "Z");
            ui.add(egui::DragValue::new(&mut self.value.z).speed(0.1));
        }).response
    }
}
```

Use it:
```rust
ui.add(Vec3Editor::new(&mut position, "Position"));
```

## Common patterns

### Collapsing sections

```rust
egui::CollapsingHeader::new("Advanced Settings")
    .default_open(false)
    .show(ui, |ui| {
        ui.add(egui::Slider::new(&mut config.quality, 0..=3));
        ui.checkbox(&mut config.debug_draw, "Debug Draw");
    });
```

### Scrollable areas

```rust
egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
    for item in &items {
        ui.label(item.name);
    }
});
```

### Drag values

```rust
ui.add(egui::DragValue::new(&mut mass)
    .speed(0.1)
    .range(0.0..=1000.0)
    .suffix(" kg"));
```

### Color picker

```rust
let mut color = [1.0, 0.5, 0.2, 1.0]; // RGBA float
ui.color_edit_button_rgba_unmultiplied(&mut color);
```

## Renzora's built-in widgets

These are available from `renzora_editor::widgets`:

| Widget | Description |
|--------|-------------|
| `EntityPicker` | Dropdown to select an entity from the scene |
| `AssetBrowser` | File picker filtered by asset type |
| `ColorPicker` | HDR-aware color picker with hex input |
| `CurvePlot` | Editable spline curve (for animation, easing) |
| `Vec3Editor` | XYZ drag-value editor with colored labels |
| `QuaternionEditor` | Euler angle editor backed by quaternion |
| `TransformGizmo` | 3D move/rotate/scale gizmo (viewport) |
| `SearchBar` | Text input with debounced search callback |
| `TagList` | Add/remove tags with autocomplete |
| `PropertyGrid` | Two-column label + value layout |

### Usage

```rust
use renzora_editor::widgets::*;

// Entity picker
let mut target: Option<Entity> = None;
ui.add(EntityPicker::new(&mut target, world));

// Asset browser (filtered to textures)
let mut texture_path = String::new();
ui.add(AssetBrowser::new(&mut texture_path, AssetType::Texture));

// Curve editor
ui.add(CurvePlot::new(&mut my_curve).size(egui::vec2(200.0, 100.0)));
```

## Styling widgets

Match the editor theme using `EditorTheme`:

```rust
let theme = world.resource::<EditorTheme>();

ui.visuals_mut().override_text_color = Some(theme.text_primary);
ui.style_mut().spacing.item_spacing = egui::vec2(theme.spacing, theme.spacing);

// Use theme colors
let button = egui::Button::new("Action")
    .fill(theme.accent)
    .stroke(egui::Stroke::new(1.0, theme.border));
if ui.add(button).clicked() { ... }
```

## Example: property editor

```rust
pub struct HealthEditor<'a> {
    health: &'a mut Health,
}

impl<'a> egui::Widget for HealthEditor<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.group(|ui| {
            ui.label("Health");
            ui.horizontal(|ui| {
                ui.label("Current:");
                ui.add(egui::Slider::new(&mut self.health.current, 0.0..=self.health.max));
            });
            ui.horizontal(|ui| {
                ui.label("Max:");
                ui.add(egui::DragValue::new(&mut self.health.max).range(1.0..=10000.0));
            });

            // Visual health bar
            let fraction = self.health.current / self.health.max;
            let color = if fraction > 0.5 {
                egui::Color32::GREEN
            } else if fraction > 0.25 {
                egui::Color32::YELLOW
            } else {
                egui::Color32::RED
            };
            let bar = egui::ProgressBar::new(fraction).fill(color);
            ui.add(bar);
        }).response
    }
}
```
