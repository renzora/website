# Material API Reference

Complete reference for the material system — properties, scripting, and node graph.

## PBR material properties

Every material supports these properties, editable in the Inspector or via scripts:

| Property | Type | Range | Default | Description |
|----------|------|-------|---------|-------------|
| **Base Color** | Color (RGBA) | 0–255 per channel | White | Surface albedo |
| **Base Color Texture** | Texture | — | None | Overrides base color per-pixel |
| **Metallic** | Float | 0.0–1.0 | 0.0 | 0 = dielectric, 1 = metal |
| **Metallic Texture** | Texture | — | None | Per-pixel metallic values |
| **Roughness** | Float | 0.0–1.0 | 0.5 | 0 = mirror, 1 = matte |
| **Roughness Texture** | Texture | — | None | Per-pixel roughness |
| **Normal Map** | Texture | — | None | Surface detail via normals |
| **Normal Scale** | Float | 0.0–2.0 | 1.0 | Normal map intensity |
| **Emissive Color** | Color (RGB) | 0–255 | Black | Self-illumination color |
| **Emissive Intensity** | Float | 0.0–100.0 | 0.0 | Glow brightness |
| **Emissive Texture** | Texture | — | None | Per-pixel emission |
| **Ambient Occlusion** | Texture | — | None | Cavity darkening |
| **AO Strength** | Float | 0.0–1.0 | 1.0 | AO effect intensity |
| **Opacity** | Float | 0.0–1.0 | 1.0 | 0 = invisible, 1 = opaque |
| **Opacity Texture** | Texture | — | None | Per-pixel opacity |
| **Alpha Cutoff** | Float | 0.0–1.0 | 0.5 | Discard below threshold (foliage) |
| **Double Sided** | Bool | — | false | Render both faces |
| **Unlit** | Bool | — | false | Skip lighting (UI, sky) |

## Scripting API

### set_material_color(entity, r, g, b, a)

Set the base color of an entity's material.

```rhai
// Solid red
set_material_color("my_entity", 255, 0, 0, 255);

// Semi-transparent blue
set_material_color("my_entity", 0, 100, 255, 128);

// Animate color
fn on_update() {
    let r = (sin(elapsed * 2.0) * 0.5 + 0.5) * 255.0;
    set_material_color("pulsing_light", r, 0.0, 0.0, 255.0);
}
```

**Parameters**: entity (string) — entity name. r, g, b, a (int) — color channels, 0–255.

### set_material_property(entity, property, value)

Set a float material property.

```rhai
set_material_property("chrome_ball", "metallic", 1.0);
set_material_property("chrome_ball", "roughness", 0.1);
set_material_property("glass_pane", "opacity", 0.3);
set_material_property("leaf", "alpha_cutoff", 0.4);
```

**Valid property names**: `metallic`, `roughness`, `opacity`, `alpha_cutoff`, `emissive_intensity`, `normal_scale`, `ao_strength`.

### set_material_emissive(entity, r, g, b, intensity)

Set the emissive (glow) color and intensity.

```rhai
// Neon blue glow
set_material_emissive("neon_sign", 0, 150, 255, 5.0);

// Lava glow
set_material_emissive("lava_rock", 255, 100, 0, 3.0);

// Turn off glow
set_material_emissive("lamp", 0, 0, 0, 0.0);
```

**Parameters**: entity (string), r, g, b (int, 0–255), intensity (float, typically 0–100).

Intensity values above 1.0 create bloom when the Bloom post-process effect is active.

### swap_material(entity, path)

Replace an entity's entire material with one loaded from file.

```rhai
// Change material based on game state
if health < 20 {
    swap_material("player_model", "materials/damaged.mat");
} else {
    swap_material("player_model", "materials/default.mat");
}
```

**Parameters**: entity (string), path (string) — path to `.mat` file relative to `assets/`.

## Material node graph reference

### Input nodes

| Node | Output type | Description |
|------|-------------|-------------|
| **UV** | Vec2 | Mesh UV coordinates |
| **World Position** | Vec3 | Fragment's world-space position |
| **World Normal** | Vec3 | Fragment's world-space normal |
| **View Direction** | Vec3 | Camera → fragment direction |
| **Time** | Float | Time in seconds (for animation) |
| **Camera Distance** | Float | Distance from camera to fragment |
| **Vertex Color** | Vec4 | Per-vertex color (if mesh has vertex colors) |

### Constant nodes

| Node | Output type | Description |
|------|-------------|-------------|
| **Float** | Float | Configurable constant (editable in Inspector) |
| **Color** | Vec3 or Vec4 | Color picker constant |
| **Vec2** | Vec2 | Two-component vector |
| **Vec3** | Vec3 | Three-component vector |
| **Vec4** | Vec4 | Four-component vector |

### Texture nodes

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Texture Sample** | UV (Vec2) | Color (Vec4), R, G, B, A | Sample 2D texture |
| **Normal Map** | UV (Vec2), Strength (Float) | Normal (Vec3) | Unpack and transform normal |
| **Triplanar** | Blend (Float) | Color (Vec4) | Project from 3 axes (no UV seams) |
| **Parallax** | UV, Height Map, Depth, Steps | UV (Vec2) | Parallax occlusion mapping |

### Math nodes

| Node | Inputs | Output |
|------|--------|--------|
| **Add** | A, B | A + B |
| **Subtract** | A, B | A - B |
| **Multiply** | A, B | A × B |
| **Divide** | A, B | A / B |
| **Power** | Base, Exp | Base ^ Exp |
| **Lerp** | A, B, T | mix(A, B, T) |
| **Clamp** | Value, Min, Max | clamp |
| **Smoothstep** | Edge0, Edge1, X | Hermite interpolation |
| **Step** | Edge, X | 0 or 1 |
| **Abs** | Value | |Value| |
| **Floor / Ceil / Round** | Value | Rounded value |
| **Fract** | Value | Fractional part |
| **Sin / Cos / Tan** | Angle | Trig result |
| **Dot** | A (Vec), B (Vec) | Scalar |
| **Cross** | A (Vec3), B (Vec3) | Vec3 |
| **Normalize** | Vec | Unit vector |
| **Length** | Vec | Scalar magnitude |
| **Distance** | A, B | Scalar distance |
| **Remap** | Value, InMin, InMax, OutMin, OutMax | Remapped value |
| **One Minus** | Value | 1.0 - Value |
| **Saturate** | Value | clamp(0, 1) |
| **Min / Max** | A, B | Smaller / larger |

### Effect nodes

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Fresnel** | Power, Scale | Float | View-angle effect (rim lighting) |
| **Noise (Perlin)** | UV, Scale, Octaves | Float | Perlin noise |
| **Noise (Simplex)** | UV, Scale | Float | Simplex noise |
| **Noise (Worley)** | UV, Scale | Float, Cell ID | Cellular noise |
| **FBM** | UV, Scale, Octaves, Lacunarity, Gain | Float | Fractal Brownian motion |
| **Voronoi** | UV, Scale | Distance, Cell Color | Voronoi cells |
| **UV Transform** | UV, Scale (Vec2), Offset (Vec2), Rotation (Float) | UV (Vec2) | Transform UVs |

### Output ports

Connect nodes to these final output ports:

| Port | Type | Description |
|------|------|-------------|
| **Base Color** | Vec3 | Surface albedo |
| **Metallic** | Float | Metal vs dielectric (0–1) |
| **Roughness** | Float | Surface smoothness (0–1) |
| **Normal** | Vec3 | Tangent-space normal |
| **Emissive** | Vec3 | Self-illumination color × intensity |
| **Ambient Occlusion** | Float | Cavity darkening (0–1) |
| **Opacity** | Float | Transparency (0–1) |

## Common material recipes

### Glowing pulse

```
Time → Sin → Remap(0-1) → Multiply(Emissive Color) → Emissive
```

### Dissolve

```
UV → Noise(Perlin) → Step(Threshold) → Opacity
UV → Noise(Perlin) → Smoothstep(Threshold, Threshold+EdgeWidth) → OneMinus → Multiply(EdgeColor) → Emissive
```

### Scrolling texture

```
UV → Add(Time × ScrollSpeed) → TextureSample → BaseColor
```

### Wet surface

```
Fresnel(Power:3) → Lerp(DryRoughness, 0.1, Fresnel) → Roughness
BaseColor → Multiply(0.7) → BaseColor  (darken when wet)
```

### Triplanar terrain

```
WorldPosition → Triplanar(TopTexture, SideTexture) → BaseColor
WorldNormal.Y → Smoothstep(0.5, 0.8) → Blend weight for top vs side
```
