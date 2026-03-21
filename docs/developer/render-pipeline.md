# Render Pipeline

How Renzora renders each frame.

## Overview

Renzora uses Bevy's **wgpu**-based renderer with a **deferred rendering** pipeline. The GPU work for each frame:

```
PrePass → GBuffer → Lighting → PostProcess → UI → Present
```

## Render stages

### 1. PrePass

- Depth-only pass for all opaque geometry
- Generates the depth buffer used by SSAO, depth-of-field, etc.
- Motion vectors for TAA and motion blur

### 2. GBuffer

- Renders all opaque meshes into multiple render targets:
  - **Albedo** (RGB) + **Metallic** (A)
  - **Normal** (RGB) + **Roughness** (A)
  - **Emissive** (RGB) + **AO** (A)
  - **Depth** (R32F)
- Materials compile to fragment shaders that output to these targets

### 3. Lighting

- **Directional lights** — sun/moon, cascaded shadow maps (4 cascades)
- **Point lights** — clustered forward rendering for up to 256 lights
- **Spot lights** — with shadow maps
- **Ambient** — image-based lighting (IBL) from environment maps
- **SSAO** — screen-space ambient occlusion applied here
- **SSR** — screen-space reflections composited with environment reflections

### 4. Post-processing

Effects applied sequentially to the lit image. See [Post-Processing Effects](/docs/developer/post-processing).

### 5. UI overlay

- Editor UI (egui) rendered on top
- Game UI (bevy_ui) rendered in-world or as overlay
- Debug visualizations (physics colliders, navmesh, gizmos)

## Render features

| Feature | Implementation |
|---------|---------------|
| **PBR** | Metallic-roughness workflow, Cook-Torrance BRDF |
| **HDR** | 16-bit float render targets, tone mapping |
| **Shadows** | Cascaded shadow maps (directional), cube maps (point), 2D maps (spot) |
| **SSAO** | Ground-truth AO with temporal filtering |
| **SSR** | Hi-Z traced reflections with fallback to environment map |
| **Bloom** | Dual-kawase blur with threshold |
| **Anti-aliasing** | FXAA, TAA, SMAA |
| **GPU culling** | Frustum and occlusion culling on the GPU |

## Material compilation

The Material Editor's node graph compiles to WGSL:

1. Topological sort of the node graph
2. Generate variable declarations for each node output
3. Inline each node's `ShaderSnippet`
4. Wrap in a standard PBR fragment function
5. Compile and cache the shader

Recompilation only happens when the graph changes. All entities sharing a material graph share one shader.

## Instanced rendering

Entities with identical mesh + material are **automatically instanced**:

- Per-instance data (transform, color overrides) sent via storage buffers
- Thousands of identical objects in a single draw call
- The editor shows instance counts in **Window → Render Stats**

## Extending the render graph

Add custom render passes:

```rust
use bevy::render::render_graph::{RenderGraph, Node, NodeRunError, RenderGraphContext};

pub struct MyRenderNode;

impl Node for MyRenderNode {
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        // Custom GPU work here
        Ok(())
    }
}
```

Insert into the graph:
```rust
let mut graph = render_app.world.resource_mut::<RenderGraph>();
graph.add_node("my_pass", MyRenderNode);
graph.add_node_edge("main_pass", "my_pass");
```

## Performance profiling

### Editor tools

- **Window → Render Stats** — FPS, draw calls, triangles, GPU time per pass
- **Window → GPU Profiler** — per-pass timing breakdown
- **View → Wireframe** — visualize triangle density
- **View → Overdraw** — visualize how many times each pixel is drawn

### Key metrics

| Metric | Target (60 FPS) | Description |
|--------|-----------------|-------------|
| Frame time | < 16.6 ms | Total CPU + GPU time |
| Draw calls | < 2000 | Fewer = better (use instancing) |
| Triangle count | < 5M | Per frame (with LOD) |
| Texture memory | < 2 GB | VRAM usage |

### Optimization strategies

- **LOD** — automatic level-of-detail for meshes based on camera distance
- **Frustum culling** — skip objects outside the camera view (automatic)
- **Occlusion culling** — skip objects behind other objects (GPU-based)
- **Instancing** — batch identical meshes (automatic)
- **Texture streaming** — load mip levels on demand
- **Shadow distance** — limit shadow rendering distance
