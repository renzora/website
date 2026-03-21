# Theming System

Customize the editor's visual appearance.

## EditorTheme resource

The editor's look is controlled by the `EditorTheme` resource:

```rust
#[derive(Resource)]
pub struct EditorTheme {
    // Backgrounds
    pub bg_primary: Color32,        // Main background
    pub bg_secondary: Color32,      // Panel backgrounds
    pub bg_tertiary: Color32,       // Input fields, insets

    // Accent
    pub accent: Color32,            // Primary action color
    pub accent_hover: Color32,      // Hovered accent
    pub accent_muted: Color32,      // Subtle accent (borders, indicators)

    // Text
    pub text_primary: Color32,      // Main text
    pub text_secondary: Color32,    // Labels, hints
    pub text_muted: Color32,        // Disabled text

    // Borders
    pub border: Color32,            // Panel and input borders
    pub border_focused: Color32,    // Focused input border

    // Status
    pub success: Color32,           // Green — success states
    pub warning: Color32,           // Yellow — warnings
    pub error: Color32,             // Red — errors, destructive actions

    // Spacing
    pub spacing: f32,               // Default item spacing
    pub rounding: f32,              // Corner radius for panels/buttons
    pub padding: f32,               // Inner padding

    // Fonts
    pub font_size: f32,             // Base font size
    pub font_mono: FontId,          // Monospace font (code, values)
}
```

## Built-in themes

| Theme | Description |
|-------|-------------|
| **Dark** (default) | Dark backgrounds, light text, indigo accent |
| **Light** | Light backgrounds, dark text, blue accent |
| **High Contrast** | Maximum contrast for accessibility, bold borders |

Switch themes: **Edit → Preferences → Theme**

## Creating a custom theme

### From code

```rust
pub fn cyberpunk_theme() -> EditorTheme {
    EditorTheme {
        bg_primary: Color32::from_rgb(10, 5, 20),
        bg_secondary: Color32::from_rgb(20, 10, 35),
        bg_tertiary: Color32::from_rgb(30, 15, 50),

        accent: Color32::from_rgb(0, 255, 200),    // neon cyan
        accent_hover: Color32::from_rgb(0, 200, 160),
        accent_muted: Color32::from_rgb(0, 100, 80),

        text_primary: Color32::from_rgb(220, 220, 255),
        text_secondary: Color32::from_rgb(150, 150, 200),
        text_muted: Color32::from_rgb(80, 80, 120),

        border: Color32::from_rgb(50, 30, 80),
        border_focused: Color32::from_rgb(0, 255, 200),

        success: Color32::from_rgb(0, 255, 100),
        warning: Color32::from_rgb(255, 200, 0),
        error: Color32::from_rgb(255, 50, 80),

        spacing: 4.0,
        rounding: 6.0,
        padding: 8.0,
        font_size: 14.0,
        font_mono: FontId::monospace(13.0),
    }
}
```

Register:
```rust
app.register_theme("Cyberpunk", cyberpunk_theme());
```

### From a TOML file

Create `editor_themes/my_theme.toml`:

```toml
[colors]
bg_primary = "#0a0514"
bg_secondary = "#140a23"
accent = "#00ffc8"
text_primary = "#dcdcff"
border = "#321e50"
success = "#00ff64"
warning = "#ffc800"
error = "#ff3250"

[layout]
spacing = 4.0
rounding = 6.0
padding = 8.0
font_size = 14.0
```

The editor scans `editor_themes/` and adds them to the theme selector.

## Applying themes to custom panels

Use the theme resource in your panel's `show()`:

```rust
fn show(&mut self, ctx: &egui::Context, is_open: &mut bool, world: &World) {
    let theme = world.resource::<EditorTheme>();

    let frame = egui::Frame::none()
        .fill(theme.bg_secondary)
        .stroke(egui::Stroke::new(1.0, theme.border))
        .rounding(theme.rounding)
        .inner_margin(theme.padding);

    egui::Window::new("My Panel")
        .frame(frame)
        .open(is_open)
        .show(ctx, |ui| {
            // Buttons match theme
            let btn = egui::Button::new("Action").fill(theme.accent);
            if ui.add(btn).clicked() { /* ... */ }

            // Status colors
            ui.colored_label(theme.error, "Critical!");
            ui.colored_label(theme.success, "All good");
        });
}
```

## Runtime theme switching

Switch themes at runtime from any system:

```rust
fn toggle_theme(
    keys: Res<ButtonInput<KeyCode>>,
    mut theme_manager: ResMut<ThemeManager>,
) {
    if keys.just_pressed(KeyCode::F12) {
        theme_manager.cycle_next(); // Dark → Light → High Contrast → ...
    }
}
```

Or set a specific theme:
```rust
theme_manager.set_theme("Cyberpunk");
```

## Color reference

Default dark theme values:

| Token | Hex | Usage |
|-------|-----|-------|
| `bg_primary` | `#0a0a0b` | Main editor background |
| `bg_secondary` | `#111113` | Panel backgrounds |
| `bg_tertiary` | `#1a1a1e` | Input fields |
| `accent` | `#6366f1` | Buttons, active tabs, selections |
| `text_primary` | `#fafafa` | Main text |
| `text_secondary` | `#a1a1aa` | Labels |
| `text_muted` | `#52525b` | Disabled text |
| `border` | `#27272a` | Panel borders |
| `success` | `#22c55e` | Success states |
| `warning` | `#eab308` | Warnings |
| `error` | `#ef4444` | Errors |
