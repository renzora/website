# Rhai Scripting

Write game logic with Rhai — a lightweight, Rust-native scripting language.

## Your first script

Create a file called `scripts/player.rhai`:

```rhai
fn on_ready() {
    print("Player spawned!");
}

fn on_update() {
    let speed = 5.0;
    position_x += input_x * speed * delta;
    position_z += input_y * speed * delta;
}
```

Attach it to an entity in the Inspector. When you hit play, the entity moves with WASD.

## Script properties

Expose variables to the Inspector so designers can tweak values without editing code:

```rhai
fn props() {
    #{
        speed: #{ default: 5.0, min: 0.0, max: 100.0 },
        jump_force: #{ default: 10.0, min: 0.0, max: 50.0 },
        can_fly: #{ default: false }
    }
}

fn on_update() {
    position_x += input_x * speed * delta;
}
```

The `speed`, `jump_force`, and `can_fly` properties appear as editable fields in the Inspector with appropriate widgets (sliders, checkboxes).

## Working with collisions

```rhai
fn on_update() {
    if collisions_entered > 0 {
        print("Hit something!");
    }

    if active_collisions > 0 {
        print("Still touching");
    }
}
```

## Entity hierarchy

```rhai
fn on_ready() {
    print(self_entity_name);
    print("Children: " + children_count);
    print("Parent: " + parent_entity_id);
}
```

## Rhai syntax basics

Rhai is similar to Rust and JavaScript:

```rhai
// Variables
let x = 42;
let name = "hello";

// Functions
fn add(a, b) {
    a + b
}

// Control flow
if x > 10 {
    print("big");
} else {
    print("small");
}

// Loops
for i in 0..10 {
    print(i);
}

while x > 0 {
    x -= 1;
}
```
