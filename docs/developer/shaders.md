# WGSL Shaders

Write custom shaders for materials and post-processing.

## WGSL basics

Renzora uses **WGSL** (WebGPU Shading Language) — the standard shader language for wgpu/WebGPU.

```wgsl
// Types
var x: f32 = 1.0;
var v: vec3<f32> = vec3<f32>(1.0, 2.0, 3.0);
var m: mat4x4<f32>;

// Functions
fn my_func(a: f32, b: f32) -> f32 {
    return a + b;
}

// Swizzling
var color: vec4<f32> = vec4<f32>(1.0, 0.5, 0.0, 1.0);
var rgb: vec3<f32> = color.rgb;
var rr: vec2<f32> = color.xx;
```

## Shader structure

### Vertex shader

```wgsl
#import bevy_pbr::mesh_functions
#import bevy_pbr::mesh_bindings

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let model = mesh_functions::get_world_from_local(in.instance_index);
    out.world_position = (model * vec4<f32>(in.position, 1.0)).xyz;
    out.world_normal = (model * vec4<f32>(in.normal, 0.0)).xyz;
    out.clip_position = view.clip_from_world * vec4<f32>(out.world_position, 1.0);
    out.uv = in.uv;
    return out;
}
```

### Fragment shader

```wgsl
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = vec3<f32>(0.8, 0.2, 0.1);
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 0.5));
    let ndotl = max(dot(normalize(in.world_normal), light_dir), 0.0);
    let color = base_color * (ndotl * 0.8 + 0.2); // diffuse + ambient
    return vec4<f32>(color, 1.0);
}
```

## Uniform bindings

### View uniforms (group 0)

Available in all shaders:

```wgsl
struct View {
    clip_from_world: mat4x4<f32>,      // view-projection matrix
    world_from_clip: mat4x4<f32>,      // inverse
    world_position: vec3<f32>,          // camera position
    viewport: vec4<f32>,               // x, y, width, height
}
@group(0) @binding(0) var<uniform> view: View;
```

### Mesh uniforms (group 1)

Per-mesh data:

```wgsl
struct Mesh {
    world_from_local: mat4x4<f32>,     // model matrix
    inverse_transpose: mat4x4<f32>,     // for normal transformation
}
```

### Global uniforms

```wgsl
#import renzora::common_uniforms

// Available:
// globals.time       - elapsed seconds
// globals.delta_time - frame delta
// globals.frame      - frame counter
```

## Built-in imports

```wgsl
#import bevy_pbr::mesh_functions       // model matrix helpers
#import bevy_pbr::mesh_bindings        // mesh uniform access
#import bevy_pbr::utils                // PI, saturate, etc.
#import renzora::common_uniforms       // time, delta, frame
#import renzora::noise                 // perlin, simplex, worley, fbm
#import renzora::math                  // remap, smooth_union, rotate2d
```

## Post-process shader pattern

```wgsl
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;
@group(0) @binding(2) var depth_texture: texture_depth_2d;

struct MyParams {
    intensity: f32,
    color: vec3<f32>,
}
@group(1) @binding(0) var<uniform> params: MyParams;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(screen_texture, screen_sampler, in.uv);
    let depth = textureSample(depth_texture, screen_sampler, in.uv);

    // Your effect here
    let result = mix(color.rgb, params.color, params.intensity);
    return vec4<f32>(result, 1.0);
}
```

## Example: hologram shader

```wgsl
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let scan_line = step(0.5, fract(in.world_position.y * 50.0 + globals.time * 2.0));
    let fresnel = pow(1.0 - max(dot(normalize(in.world_normal), normalize(view.world_position - in.world_position)), 0.0), 2.0);

    let holo_color = vec3<f32>(0.0, 0.8, 1.0);
    let intensity = (scan_line * 0.3 + 0.7) * (fresnel * 0.5 + 0.5);

    return vec4<f32>(holo_color * intensity, 0.7);
}
```

## Example: water surface

```wgsl
#import renzora::noise

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let t = globals.time * 0.5;
    let uv = in.uv * 10.0;

    // Layered noise for waves
    let wave1 = noise::perlin(uv + vec2<f32>(t, 0.0)) * 0.5;
    let wave2 = noise::perlin(uv * 2.0 + vec2<f32>(0.0, t * 0.7)) * 0.25;
    let waves = wave1 + wave2;

    // Fresnel for reflection blend
    let view_dir = normalize(view.world_position - in.world_position);
    let fresnel = pow(1.0 - max(dot(in.world_normal, view_dir), 0.0), 4.0);

    let deep = vec3<f32>(0.0, 0.1, 0.3);
    let shallow = vec3<f32>(0.0, 0.4, 0.5);
    let color = mix(deep, shallow, waves + 0.5);
    let final_color = mix(color, vec3<f32>(0.8, 0.9, 1.0), fresnel);

    return vec4<f32>(final_color, 0.85);
}
```

## Hot reloading

Shaders in the `assets/shaders/` directory hot-reload in the editor:

1. Edit a `.wgsl` file and save
2. The engine detects the change and recompiles
3. The viewport updates immediately

Compilation errors appear in the console with line numbers.

## Debugging

- **Validation errors** — wgpu validates shaders at compile time. Check the console for detailed error messages.
- **Visual debugging** — output intermediate values as colors: `return vec4<f32>(my_value, my_value, my_value, 1.0);`
- **RenderDoc** — attach RenderDoc to capture and inspect individual frames, shader inputs/outputs, and GPU state
