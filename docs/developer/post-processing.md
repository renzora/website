# Post-Processing Effects

Add screen-space effects to the rendering pipeline.

## PostProcessEffect trait

Every post-processing effect implements:

```rust
pub trait PostProcessEffect: Component + ExtractComponent {
    /// Path to the WGSL fragment shader
    fn fragment_shader() -> ShaderRef;
}
```

The engine handles the render pipeline integration. You write the component (data) and the shader (logic).

## Built-in effects

Renzora includes 40+ effects. Each is a component — add it to a camera entity to enable:

### Color & Tone
| Effect | Key properties |
|--------|---------------|
| `ToneMapping` | mode (ACES, Reinhard, Filmic), exposure |
| `ColorGrading` | temperature, tint, saturation, contrast, brightness |
| `ColorCorrection` | shadows/midtones/highlights color, lift/gamma/gain |
| `Posterize` | levels (2–256) |
| `Sepia` | intensity |
| `Invert` | intensity |

### Blur & Focus
| Effect | Key properties |
|--------|---------------|
| `DepthOfField` | focal_distance, focal_range, bokeh_intensity |
| `MotionBlur` | intensity, samples |
| `GaussianBlur` | radius, sigma |
| `TiltShift` | focus_position, blur_size |

### Lighting
| Effect | Key properties |
|--------|---------------|
| `Bloom` | threshold, intensity, radius |
| `SSAO` | radius, bias, intensity, samples |
| `SSR` | max_steps, thickness, intensity |
| `VolumetricFog` | density, scattering, absorption |
| `LensFlare` | threshold, intensity, ghost_count |
| `GodRays` | intensity, decay, density |

### Stylistic
| Effect | Key properties |
|--------|---------------|
| `Vignette` | intensity, smoothness, color |
| `ChromaticAberration` | intensity, offset |
| `FilmGrain` | intensity, size |
| `Scanlines` | count, intensity, speed |
| `CRT` | curvature, scanline_intensity, vignette |
| `Pixelate` | pixel_size |
| `Outline` | thickness, color, depth_threshold, normal_threshold |
| `Dithering` | pattern (Bayer, BlueNoise), bit_depth |

### Anti-aliasing
| Effect | Key properties |
|--------|---------------|
| `FXAA` | quality (Low, Medium, High) |
| `TAA` | blend_factor, jitter_intensity |
| `SMAA` | quality |

### Fog
| Effect | Key properties |
|--------|---------------|
| `DistanceFog` | color, start, end, density |
| `HeightFog` | color, base_height, density, falloff |

## Creating a custom effect

### 1. Define the component

```rust
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use renzora_rendering::PostProcessEffect;

#[derive(Component, Clone, ExtractComponent)]
pub struct UnderwaterEffect {
    pub distortion: f32,
    pub speed: f32,
    pub tint: Vec3,
    pub tint_strength: f32,
}

impl Default for UnderwaterEffect {
    fn default() -> Self {
        Self {
            distortion: 0.02,
            speed: 1.0,
            tint: Vec3::new(0.0, 0.3, 0.5),
            tint_strength: 0.3,
        }
    }
}

impl PostProcessEffect for UnderwaterEffect {
    fn fragment_shader() -> ShaderRef {
        "shaders/underwater.wgsl".into()
    }
}
```

### 2. Write the shader

Create `assets/shaders/underwater.wgsl`:

```wgsl
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import renzora::common_uniforms

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct UnderwaterUniforms {
    distortion: f32,
    speed: f32,
    tint: vec3<f32>,
    tint_strength: f32,
}

@group(1) @binding(0) var<uniform> params: UnderwaterUniforms;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;

    // Wavy distortion
    let wave_x = sin(uv.y * 20.0 + globals.time * params.speed) * params.distortion;
    let wave_y = cos(uv.x * 15.0 + globals.time * params.speed * 0.8) * params.distortion;
    uv += vec2<f32>(wave_x, wave_y);

    // Sample screen
    var color = textureSample(screen_texture, texture_sampler, uv);

    // Apply tint
    color = mix(color, vec4<f32>(params.tint, 1.0), params.tint_strength);

    return color;
}
```

### 3. Register

```rust
app.add_post_process::<UnderwaterEffect>();
```

### 4. Use it

Add the component to a camera entity:

```rust
commands.spawn((
    Camera3d::default(),
    UnderwaterEffect {
        distortion: 0.03,
        tint: Vec3::new(0.0, 0.4, 0.6),
        ..default()
    },
));
```

Or from a script:
```rhai
fn on_ready() {
    // Effects can be toggled at runtime
    add_effect("camera", "underwater");
    set_effect_property("camera", "underwater", "distortion", 0.02);
}
```

## Shader uniforms

Component fields are automatically extracted to GPU uniforms. The mapping:

| Rust type | WGSL type |
|-----------|-----------|
| `f32` | `f32` |
| `Vec2` | `vec2<f32>` |
| `Vec3` | `vec3<f32>` |
| `Vec4` | `vec4<f32>` |
| `u32` | `u32` |
| `i32` | `i32` |

The uniform struct layout must match the component fields **in order**.

## Render order

Effects execute in a fixed order:

1. SSAO
2. SSR
3. Lighting/GodRays
4. Custom effects (in registration order)
5. Bloom
6. Tone mapping
7. Color grading
8. Anti-aliasing (FXAA/TAA/SMAA)
9. Vignette, film grain, CRT (overlays)

Override with priority:
```rust
app.add_post_process_with_priority::<MyEffect>(500); // higher = later
```

## Performance

- **Inactive effects have zero overhead** — only components present on a camera execute
- Profile with **Window → Render Stats** in the editor (shows per-effect GPU time)
- Combine multiple lightweight effects into one shader for fewer render passes
- Half-resolution rendering for expensive effects (SSAO, SSR): set `half_res: true`
