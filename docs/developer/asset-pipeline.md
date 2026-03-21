# Asset Pipeline

How Renzora loads, processes, and distributes game assets.

## Bevy's asset system

Bevy provides an async asset loading system. Assets are loaded by file path and tracked with handles.

```rust
let mesh: Handle<Mesh> = asset_server.load("models/player.glb#Mesh0");
let texture: Handle<Image> = asset_server.load("textures/brick.png");
let sound: Handle<AudioSource> = asset_server.load("sounds/explosion.ogg");
```

## Asset types in Renzora

| Type | Rust type | File formats |
|------|-----------|-------------|
| Meshes | `Handle<Mesh>` | `.glb`, `.gltf`, `.fbx`, `.obj` |
| Textures | `Handle<Image>` | `.png`, `.jpg`, `.hdr`, `.ktx2`, `.tga`, `.bmp` |
| Materials | `Handle<StandardMaterial>` | `.mat` (Renzora format) |
| Audio | `Handle<AudioSource>` | `.ogg`, `.wav`, `.mp3`, `.flac` |
| Scenes | `Handle<DynamicScene>` | `.ron` (Rusty Object Notation) |
| Scripts | `Handle<ScriptAsset>` | `.rhai`, `.lua` |
| Blueprints | `Handle<BlueprintAsset>` | `.blueprint` |
| Shaders | `Handle<Shader>` | `.wgsl` |
| Fonts | `Handle<Font>` | `.ttf`, `.otf` |

## Asset handles

```rust
// Strong handle — keeps the asset loaded
let handle: Handle<Image> = asset_server.load("textures/brick.png");

// Weak handle — doesn't prevent unloading
let weak: Handle<Image> = handle.clone_weak();

// Check load state
match asset_server.get_load_state(&handle) {
    LoadState::Loading => { /* still loading */ },
    LoadState::Loaded => { /* ready to use */ },
    LoadState::Failed => { /* error */ },
    _ => {},
}
```

## Hot reloading

In the editor, assets reload automatically when files change on disk:

1. Edit a texture in Photoshop → save
2. The editor detects the file change
3. Asset is reloaded in place — all references update instantly

Hot reload works for textures, materials, scripts, shaders, and scenes. Mesh hot reload requires re-import.

Enable/disable in editor settings: **Edit → Preferences → Asset Hot Reload**.

## Custom asset loaders

Add support for new file formats:

```rust
use bevy::asset::{AssetLoader, LoadContext, BoxedFuture};

#[derive(Asset, TypePath, Debug)]
pub struct DialogueTree {
    pub nodes: Vec<DialogueNode>,
}

#[derive(Default)]
pub struct DialogueLoader;

impl AssetLoader for DialogueLoader {
    type Asset = DialogueTree;
    type Settings = ();
    type Error = anyhow::Error;

    fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl Future<Output = Result<Self::Asset, Self::Error>> {
        async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let tree: DialogueTree = serde_json::from_slice(&bytes)?;
            Ok(tree)
        }
    }

    fn extensions(&self) -> &[&str] {
        &["dialogue", "dlg"]
    }
}
```

Register:
```rust
app.init_asset::<DialogueTree>()
   .init_asset_loader::<DialogueLoader>();
```

Now `asset_server.load("dialogue/intro.dlg")` works.

## Asset processing

For formats that need conversion (e.g., FBX → internal mesh format), Bevy supports asset processors:

```rust
app.init_asset_processor::<MyProcessor>();
```

Processors run at import time, converting source assets to engine-optimized formats. Processed assets are cached in `.bevy_processed/`.

## Asset metadata

Each asset can have a `.meta` file with import settings:

```
textures/brick.png.meta
```

```ron
(
    format: "Png",
    sampler: (
        filter: Linear,
        wrap: Repeat,
    ),
    generate_mipmaps: true,
)
```

The editor generates `.meta` files automatically. Edit them to change import settings.

## rpak format

For distribution, assets are packed into `.rpak` archives:

```bash
renzora pack --input assets/ --output game.rpak
```

rpak files contain:
- Compressed asset data (LZ4 or Zstd)
- An index for fast lookups
- Optional AES-256 encryption

See [Asset Packing](/docs/developer/asset-packing) for details.

## Asset resolution priority

When loading an asset by path, the engine checks in order:

1. **Absolute path** — pass-through
2. **rpak archive** — packed builds
3. **Project `assets/` directory** — development
4. **Exe-adjacent `assets/` directory** — installed games
5. **CWD `assets/` directory** — fallback

This allows hot-reload in the editor while supporting packed distribution.

## Memory management

- Assets are reference-counted via handles
- When no strong handles remain, the asset is queued for unloading
- `asset_server.load()` returns the same handle if the asset is already loaded
- Large assets (textures, meshes) are stored on the GPU — RAM is freed after upload
- Monitor memory usage: **Window → Asset Inspector** in the editor
