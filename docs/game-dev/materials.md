# Material Editor

Create and edit PBR materials to control how surfaces look.

## Overview

Renzora uses a **Physically Based Rendering (PBR)** material model. Every mesh needs a material to define its appearance — color, reflectivity, roughness, and more.

## Creating a material

1. Right-click in the Project panel → **New → Material**
2. Name it (e.g., `stone_wall.mat`)
3. Double-click to open the **Material Editor**

Or assign directly: select an entity → Inspector → Material → **New Material**.

## Material properties

| Property | Type | Range | Description |
|----------|------|-------|-------------|
| **Base Color** | Color + Texture | RGBA | The surface color (albedo) |
| **Metallic** | Float + Texture | 0.0–1.0 | 0 = dielectric (plastic, wood), 1 = metal |
| **Roughness** | Float + Texture | 0.0–1.0 | 0 = mirror-smooth, 1 = completely rough |
| **Normal Map** | Texture | — | Adds surface detail without extra geometry |
| **Emissive** | Color + Float | RGBA + intensity | Self-illumination (glowing surfaces) |
| **Ambient Occlusion** | Texture | 0.0–1.0 | Darkens crevices and corners |
| **Opacity** | Float + Texture | 0.0–1.0 | 0 = fully transparent, 1 = fully opaque |
| **Alpha Cutoff** | Float | 0.0–1.0 | Discard pixels below threshold (foliage, fences) |

## Texture slots

Each property that supports textures has a slot. Click the slot to browse for a texture file, or drag an image from the Project panel.

Supported formats: **PNG**, **JPEG**, **TGA**, **BMP**, **HDR**, **KTX2**.

Textures override the flat value. For example, a Roughness texture overrides the Roughness slider per-pixel.

## Material instances

Create a variant of an existing material without duplicating it:

1. Right-click a material → **Create Instance**
2. Override only the properties you want to change

Instances share the parent's shader. Change the parent and all instances update — except for overridden properties.

## The Material Node Graph

For advanced materials, switch to **Node Graph** mode in the Material Editor. Connect nodes to procedurally generate textures and effects:

- **Texture Sample** — sample a texture file
- **Color / Float / Vec2 / Vec3** — constant values
- **Math nodes** — Add, Multiply, Lerp, Clamp, Power, Abs, Sin, Cos
- **Noise** — Perlin, Simplex, Worley, FBM
- **UV Transform** — scale, offset, rotate UVs
- **Fresnel** — view-angle-dependent effect (rim lighting, glass)
- **Normal Map** — unpack and blend normal maps
- **Parallax** — fake depth using height maps
- **Triplanar** — project textures from 3 axes (no UV seams)
- **Time** — animate materials over time

The node graph compiles to a WGSL shader automatically.

## Scripting materials

Change material properties at runtime from scripts:

```rhai
fn on_update() {
    // Set base color (RGBA, 0-255)
    set_material_color("my_entity", 255, 0, 0, 255);

    // Set individual PBR properties
    set_material_property("my_entity", "metallic", 0.8);
    set_material_property("my_entity", "roughness", 0.2);
    set_material_property("my_entity", "opacity", 0.5);

    // Set emissive glow
    set_material_emissive("my_entity", 0, 100, 255, 3.0);

    // Swap to a completely different material
    swap_material("my_entity", "materials/gold.mat");
}
```

### Material API reference

| Function | Description |
|----------|-------------|
| `set_material_color(entity, r, g, b, a)` | Set base color (0–255 per channel) |
| `set_material_property(entity, name, value)` | Set a float property by name |
| `set_material_emissive(entity, r, g, b, intensity)` | Set emissive color and brightness |
| `swap_material(entity, path)` | Replace the entire material from file |

Valid property names for `set_material_property`: `metallic`, `roughness`, `opacity`, `alpha_cutoff`, `emissive_intensity`.

## Tips

- **Use texture atlases** for many small objects sharing similar materials — reduces draw calls
- **Keep roughness above 0.05** — perfectly smooth surfaces can cause visual artifacts
- **Metallic should be 0 or 1** for most real-world materials. Values in between are rarely physically correct
- **Normal map format** — use OpenGL-style (green channel = up). The engine auto-converts DirectX-style if detected
- **Material batching** — entities sharing the same material are batched into fewer draw calls. Reuse materials when possible
