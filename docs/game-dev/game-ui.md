# Game UI

Control in-game UI elements from your scripts.

## Overview

Renzora has a built-in game UI system separate from the editor UI. You create UI in the editor's **UI workspace**, then control it from scripts at runtime.

The UI system uses **bevy_ui** (not egui) and supports 19+ widget types.

## Widget Types

### Layout
- **Panel** — flat background container
- **Layout Container** — flex layout for arranging children
- **Virtual Scroll** — scrollable area

### Basic
- **Label** — text display
- **Button** — clickable button
- **Image** — texture display
- **Progress Bar** — horizontal fill bar

### Input
- **Slider** — drag to set a value
- **Checkbox** — on/off toggle
- **Toggle** — switch control
- **Radio Button** — exclusive selection
- **Text Input** — editable text field
- **Dropdown Menu** — pick from a list

### Display
- **Tooltip** — hover information
- **Spinner** — loading indicator
- **Tab Bar** — tabbed navigation

### Overlay
- **Modal** — popup dialog
- **Draggable Window** — movable panel

### HUD
- **Crosshair**, **Ammo Counter**, **Compass**, **Health Bar**
- **Minimap**, **Inventory Grid**, **Dialog Box**
- **Notification Feed**, **Radial Menu**

## Controlling UI from Scripts

All UI functions use the entity name (set in the Hierarchy) to target widgets.

### Show / Hide

```rhai
ui_show("health_bar");
ui_hide("main_menu");
ui_toggle("inventory");
```

### Set Text

```rhai
ui_set_text("score_label", "Score: " + score);
ui_set_text("player_name", "Hero");
```

### Progress Bars

```rhai
// Value from 0.0 to 1.0
ui_set_progress("loading_bar", 0.75);
ui_set_progress("xp_bar", current_xp / max_xp);
```

### Health Bars

```rhai
// (widget_name, current_health, max_health)
ui_set_health("player_hp", 80, 100);
```

### Sliders and Toggles

```rhai
ui_set_slider("volume_slider", 0.5);
ui_set_checkbox("vsync_toggle", true);
ui_set_toggle("mute_button", false);
```

### Colors

```rhai
// RGBA values (0-255)
ui_set_color("damage_flash", 255, 0, 0, 128);
ui_set_color("heal_glow", 0, 255, 100, 200);
```

### Images

```rhai
ui_set_image("weapon_icon", "textures/sword.png");
```

### Themes

Switch the entire UI theme at runtime:

```rhai
ui_set_theme("dark");           // default
ui_set_theme("light");          // light theme
ui_set_theme("high_contrast");  // accessible
```

## Example: Game HUD

```rhai
let health = 100;
let max_health = 100;
let score = 0;

fn on_ready() {
    ui_show("hud");
    ui_hide("game_over");
}

fn on_update() {
    // Update HUD elements
    ui_set_health("health_bar", health, max_health);
    ui_set_text("health_text", health + " / " + max_health);
    ui_set_text("score_text", "Score: " + score);

    // Show damage flash on hit
    if collisions_entered > 0 {
        health -= 10;
        ui_show("damage_flash");
    }

    // Game over
    if health <= 0 {
        ui_hide("hud");
        ui_show("game_over");
        ui_set_text("final_score", "Final Score: " + score);
    }
}
```
