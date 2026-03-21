# Terrain

Create and sculpt landscapes with the built-in terrain system.

## Creating terrain

1. **Scene menu → Add → Terrain**
2. Set initial size in the dialog (e.g., 512×512 meters, 256 resolution)
3. The terrain appears in the viewport with a flat plane

## Terrain properties

| Property | Default | Description |
|----------|---------|-------------|
| **Size** | 512 m | Width and length of the terrain |
| **Resolution** | 256 | Heightmap resolution (vertices per side) |
| **Height Scale** | 100 m | Maximum terrain height |
| **Chunk Size** | 64 | Subdivision for LOD and culling |
| **LOD Levels** | 4 | Number of detail levels (higher = more distant detail reduction) |

## Sculpting tools

Select the terrain entity, then open **Window → Terrain Editor** or use the toolbar.

### Brush modes

| Tool | Shortcut | Description |
|------|----------|-------------|
| **Raise** | B | Push terrain up under the brush |
| **Lower** | Shift+B | Push terrain down |
| **Smooth** | S | Average heights under the brush |
| **Flatten** | F | Level terrain to a target height |
| **Erode** | E | Simulate natural erosion patterns |

### Brush settings

- **Size** — brush radius in meters (1–200)
- **Strength** — how fast the brush affects terrain (0.01–1.0)
- **Falloff** — how the brush fades from center to edge (Sharp, Linear, Smooth, Sphere)

Hold **Ctrl** while painting to invert the current tool (raise becomes lower, etc.).

## Texture painting

The terrain supports up to **4 splat layers** blended by weight.

### Adding texture layers

1. In the Terrain Editor, switch to **Paint** tab
2. Click **+** to add a layer
3. Assign textures: albedo, normal map, roughness (optional)
4. Set tiling scale (how many times the texture repeats)

### Painting

Select a layer and paint on the terrain. The brush blends between layers based on weight. Tips:

- Paint base layer first (e.g., grass), then detail layers on top (dirt paths, rock)
- Use a low-strength brush for natural blending
- Hold **Shift** to erase the selected layer (reveals layer below)

## Foliage

Paint grass and small props directly on the terrain.

1. In the Terrain Editor, switch to **Foliage** tab
2. Add a foliage type (mesh + material)
3. Configure: density, min/max scale, random rotation, alignment to surface normal
4. Paint on the terrain — foliage instances are scattered within the brush

Foliage uses **GPU instancing** for performance. Thousands of grass blades render efficiently.

## Terrain colliders

Physics colliders are generated automatically from the heightmap. The collider updates when you sculpt.

- Collider type: **Heightfield** (efficient for terrain)
- Friction and restitution are set on the terrain's physics material
- Characters and rigid bodies interact with terrain immediately — no setup needed

## Scripting terrain

```rhai
fn on_update() {
    // Get terrain height at a world position
    let h = terrain_get_height(position_x, position_z);

    // Snap entity to terrain surface
    position_y = terrain_get_height(position_x, position_z) + 1.0;
}
```

| Function | Description |
|----------|-------------|
| `terrain_get_height(x, z)` | Returns terrain height at world X/Z |
| `terrain_set_height(x, z, h)` | Sets terrain height at world X/Z (runtime sculpting) |

## Heightmap import/export

- **Import**: Terrain Editor → File → Import Heightmap (16-bit PNG or RAW)
- **Export**: Terrain Editor → File → Export Heightmap
- Resolution must match the terrain's resolution setting

## Performance tips

- **Lower resolution** for distant or less detailed terrain
- **Reduce foliage density** for better frame rates on lower-end hardware
- **LOD** handles most optimization automatically — distant terrain uses fewer triangles
- **Chunk culling** skips rendering terrain chunks outside the camera frustum
