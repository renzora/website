# Custom Material Nodes

Extend the material editor's node graph with custom nodes.

## Material system architecture

The material editor uses a **node graph** that compiles to WGSL shaders:

1. User connects nodes in the Material Editor
2. The compiler walks the graph, collecting shader snippets
3. Snippets are composed into a complete WGSL fragment shader
4. The shader is compiled and cached

This means material nodes are **compile-time** — they produce shader code, not runtime logic.

## The MaterialNode trait

```rust
use renzora_materials::{MaterialNode, NodePort, PortType, ShaderSnippet};

pub trait MaterialNode: Send + Sync {
    /// Display name in the material node library
    fn name(&self) -> &'static str;

    /// Category for organization
    fn category(&self) -> &'static str;

    /// Input ports
    fn inputs(&self) -> Vec<NodePort>;

    /// Output ports
    fn outputs(&self) -> Vec<NodePort>;

    /// Generate WGSL shader code for this node
    fn generate_shader(&self) -> ShaderSnippet;

    /// Optional: tooltip description
    fn description(&self) -> &'static str { "" }
}
```

## Port types

| PortType | WGSL type | Description |
|----------|-----------|-------------|
| `PortType::Float` | `f32` | Scalar value |
| `PortType::Vec2` | `vec2<f32>` | 2D vector (UV coordinates) |
| `PortType::Vec3` | `vec3<f32>` | 3D vector (color, normal, position) |
| `PortType::Vec4` | `vec4<f32>` | 4D vector (color with alpha) |
| `PortType::Texture2D` | `texture_2d<f32>` | Texture reference |
| `PortType::Sampler` | `sampler` | Texture sampler |

## Built-in material nodes

### Input
| Node | Outputs | Description |
|------|---------|-------------|
| **UV** | Vec2 | Mesh UV coordinates |
| **World Position** | Vec3 | Fragment world position |
| **World Normal** | Vec3 | Fragment world normal |
| **View Direction** | Vec3 | Camera-to-fragment direction |
| **Time** | Float | Current time (for animation) |
| **Camera Distance** | Float | Distance from camera |

### Constants
| Node | Outputs | Description |
|------|---------|-------------|
| **Float** | Float | Configurable float constant |
| **Color** | Vec3/Vec4 | Color picker constant |
| **Vec2/Vec3/Vec4** | Vec2/3/4 | Vector constants |

### Texture
| Node | Outputs | Description |
|------|---------|-------------|
| **Texture Sample** | Vec4, Float (R/G/B/A) | Sample a texture at UV |
| **Normal Map** | Vec3 | Unpack and transform normal map |
| **Triplanar** | Vec4 | Project texture from 3 axes |

### Math
| Node | Description |
|------|-------------|
| **Add** | A + B |
| **Subtract** | A - B |
| **Multiply** | A * B |
| **Divide** | A / B |
| **Power** | A ^ B |
| **Lerp** | mix(A, B, T) |
| **Clamp** | clamp(value, min, max) |
| **Abs** | abs(value) |
| **Min / Max** | min/max of two values |
| **Sin / Cos** | Trigonometry |
| **Smoothstep** | smoothstep(edge0, edge1, x) |
| **Step** | step(edge, x) |
| **Dot** | dot product of two vectors |
| **Cross** | cross product of two Vec3s |
| **Normalize** | normalize vector to unit length |
| **Length** | vector magnitude |
| **Distance** | distance between two points |
| **Remap** | remap value from one range to another |
| **OneMinus** | 1.0 - x |
| **Saturate** | clamp(x, 0.0, 1.0) |
| **Fract** | fractional part |

### Effects
| Node | Description |
|------|-------------|
| **Fresnel** | View-angle effect (rim lighting, glass) |
| **Parallax Occlusion** | Fake depth from height map |
| **Noise (Perlin)** | Procedural Perlin noise |
| **Noise (Simplex)** | Procedural Simplex noise |
| **Noise (Worley)** | Procedural cellular/Worley noise |
| **FBM** | Fractal Brownian Motion (layered noise) |
| **Voronoi** | Voronoi cell pattern |
| **UV Transform** | Scale, offset, rotate UVs |

### Output
| Port | Type | Description |
|------|------|-------------|
| **Base Color** | Vec3 | Surface albedo |
| **Metallic** | Float | 0 = dielectric, 1 = metal |
| **Roughness** | Float | 0 = smooth, 1 = rough |
| **Normal** | Vec3 | Surface normal (tangent space) |
| **Emissive** | Vec3 | Self-illumination color |
| **Ambient Occlusion** | Float | Cavity darkening |
| **Opacity** | Float | 0 = transparent, 1 = opaque |

## Creating a custom node

### Example: Dissolve effect

```rust
pub struct DissolveNode;

impl MaterialNode for DissolveNode {
    fn name(&self) -> &'static str { "Dissolve" }
    fn category(&self) -> &'static str { "Effects" }
    fn description(&self) -> &'static str {
        "Dissolve effect using noise and a threshold"
    }

    fn inputs(&self) -> Vec<NodePort> {
        vec![
            NodePort::new("Threshold", PortType::Float).default(0.5),
            NodePort::new("Edge Width", PortType::Float).default(0.05),
            NodePort::new("Edge Color", PortType::Vec3).default_color(1.0, 0.5, 0.0),
            NodePort::new("Noise", PortType::Float),
        ]
    }

    fn outputs(&self) -> Vec<NodePort> {
        vec![
            NodePort::new("Alpha", PortType::Float),
            NodePort::new("Emission", PortType::Vec3),
        ]
    }

    fn generate_shader(&self) -> ShaderSnippet {
        ShaderSnippet::new("
            let dissolve_edge = smoothstep(
                {Threshold} - {Edge Width},
                {Threshold},
                {Noise}
            );
            let {Alpha} = step({Threshold}, {Noise});
            let {Emission} = {Edge Color} * (1.0 - dissolve_edge) * step({Noise}, {Threshold} + {Edge Width});
        ")
    }
}
```

### ShaderSnippet syntax

- `{InputName}` — replaced with the connected input's variable name
- `{OutputName}` — replaced with the generated output variable name
- The compiler handles variable naming, type casting, and ordering

### Example: Rim Light

```rust
pub struct RimLightNode;

impl MaterialNode for RimLightNode {
    fn name(&self) -> &'static str { "Rim Light" }
    fn category(&self) -> &'static str { "Effects" }

    fn inputs(&self) -> Vec<NodePort> {
        vec![
            NodePort::new("Color", PortType::Vec3).default_color(1.0, 1.0, 1.0),
            NodePort::new("Power", PortType::Float).default(3.0),
            NodePort::new("Intensity", PortType::Float).default(1.0),
        ]
    }

    fn outputs(&self) -> Vec<NodePort> {
        vec![NodePort::new("Result", PortType::Vec3)]
    }

    fn generate_shader(&self) -> ShaderSnippet {
        ShaderSnippet::new("
            let rim_dot = 1.0 - max(dot(normalize(view_direction), normalize(world_normal)), 0.0);
            let rim_factor = pow(rim_dot, {Power}) * {Intensity};
            let {Result} = {Color} * rim_factor;
        ")
        .requires_input("view_direction")
        .requires_input("world_normal")
    }
}
```

## Registering nodes

```rust
impl Plugin for MyMaterialNodesPlugin {
    fn build(&self, app: &mut App) {
        app.register_material_node::<DissolveNode>()
           .register_material_node::<RimLightNode>();
    }
}
```

## ShaderSnippet API

| Method | Description |
|--------|-------------|
| `ShaderSnippet::new(code)` | Create from WGSL code string |
| `.requires_input(name)` | Declare dependency on a built-in input (UV, world_normal, etc.) |
| `.requires_function(name, code)` | Include a helper function in the final shader |
| `.uniform(name, type)` | Add a uniform variable (exposed in Inspector) |

## Node categories

| Category | For |
|----------|-----|
| `"Input"` | UV, position, normal, time |
| `"Constants"` | Float, color, vector |
| `"Texture"` | Sampling, normal maps, triplanar |
| `"Math"` | Arithmetic, trigonometry, interpolation |
| `"Effects"` | Dissolve, fresnel, rim light, parallax, noise |
| `"Utility"` | UV transform, remap, pack/unpack |

## Debugging

- **Preview nodes** — right-click any node → Preview to see its output as a grayscale/color image
- **Shader output** — Material Editor → View → Generated Shader to see the compiled WGSL
- **Validation errors** — type mismatches and missing connections show as red borders
- **Hot reload** — shader changes take effect immediately in the viewport

## Performance

- Each unique material graph produces one shader. Entities sharing the same graph share the shader
- Minimize texture samples — each sample is a GPU memory read
- Prefer math nodes over texture lookups for procedural patterns
- The compiler optimizes dead code — unconnected nodes have zero cost
