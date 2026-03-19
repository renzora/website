# Input Handling

Read keyboard, mouse, and gamepad input in your scripts.

## Keyboard

The `input_x` and `input_y` globals give you axis values from WASD/arrow keys:

```rhai
fn on_update() {
    let speed = 5.0;
    position_x += input_x * speed * delta;  // A/D or Left/Right
    position_z += input_y * speed * delta;  // W/S or Up/Down
}
```

Values range from -1 to 1. Pressing D gives `input_x = 1`, pressing A gives `input_x = -1`. No key pressed = 0.

## Mouse

```rhai
fn on_update() {
    // Mouse position (screen coordinates)
    let mx = mouse_x;
    let my = mouse_y;

    // Mouse movement since last frame (great for camera look)
    let look_speed = 2.0;
    rotation_y += mouse_delta_x * look_speed * delta;
    rotation_x -= mouse_delta_y * look_speed * delta;

    // Mouse buttons (true while held)
    if mouse_button_left {
        print("Shooting!");
    }
    if mouse_button_right {
        print("Aiming");
    }
    if mouse_button_middle {
        print("Middle click");
    }
}
```

## Gamepad

```rhai
fn on_update() {
    // Left stick — movement
    position_x += gamepad_left_x * 5.0 * delta;
    position_z += gamepad_left_y * 5.0 * delta;

    // Right stick — camera look
    rotation_y += gamepad_right_x * 2.0 * delta;

    // Face buttons (A/B/X/Y on Xbox, Cross/Circle/Square/Triangle on PS)
    if gamepad_south {   // A / Cross
        print("Jump!");
    }
    if gamepad_east {    // B / Circle
        print("Dodge!");
    }
    if gamepad_north {   // Y / Triangle
        print("Special!");
    }
    if gamepad_west {    // X / Square
        print("Interact!");
    }

    // Triggers (0.0 to 1.0, analog)
    if gamepad_right_trigger > 0.5 {
        print("Firing!");
    }
    if gamepad_left_trigger > 0.5 {
        print("Aiming!");
    }
}
```

## Checking Specific Keys

For keys beyond WASD, use the key check functions:

```rhai
fn on_update() {
    if is_key_pressed("Space") {
        print("Space held");
    }
    if is_key_just_pressed("E") {
        print("E pressed this frame");
    }
    if is_key_just_released("Escape") {
        print("Escape released");
    }
}
```

## Example: FPS Controller

```rhai
fn props() {
    #{
        speed: #{ default: 5.0, min: 1.0, max: 20.0 },
        look_speed: #{ default: 2.0, min: 0.5, max: 5.0 },
        jump_force: #{ default: 8.0 }
    }
}

fn on_update() {
    // Movement
    position_x += input_x * speed * delta;
    position_z += input_y * speed * delta;

    // Mouse look
    rotation_y += mouse_delta_x * look_speed * delta;

    // Jump (if on ground)
    if is_key_just_pressed("Space") {
        apply_impulse(0.0, jump_force, 0.0);
    }

    // Sprint
    if is_key_pressed("ShiftLeft") {
        position_x += input_x * speed * 2.0 * delta;
        position_z += input_y * speed * 2.0 * delta;
    }
}
```
