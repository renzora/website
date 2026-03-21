# Asset Packing (rpak)

Pack game assets into compressed archives for distribution.

## What is rpak?

`rpak` (Renzora Pack) is the engine's asset archive format. It bundles all game assets into a single file for distribution, replacing the loose `assets/` directory.

## rpak structure

```
┌─────────────────────────────────┐
│ Header (magic, version, flags)  │
├─────────────────────────────────┤
│ Index (path → offset, size)     │
├─────────────────────────────────┤
│ Asset 1 (compressed data)       │
│ Asset 2 (compressed data)       │
│ ...                             │
│ Asset N (compressed data)       │
└─────────────────────────────────┘
```

- **Header**: magic bytes (`RPAK`), format version, compression type, encryption flag
- **Index**: maps asset paths to byte offsets and sizes within the file
- **Data**: individually compressed asset blobs

## Packing assets

### Basic usage

```bash
renzora pack --input assets/ --output game.rpak
```

### With options

```bash
renzora pack \
    --input assets/ \
    --output game.rpak \
    --compression zstd \
    --compression-level 6 \
    --exclude "*.psd" \
    --exclude "*.blend"
```

## Compression options

| Algorithm | Speed | Ratio | Best for |
|-----------|-------|-------|----------|
| **LZ4** (default) | Very fast | Good | Games with streaming (fast decompression) |
| **Zstd** | Fast | Better | Final distribution (smaller files) |
| **None** | Instant | 1:1 | Pre-compressed assets (OGG, PNG) |

```bash
renzora pack --compression lz4     # fast decompression
renzora pack --compression zstd    # smaller file size
renzora pack --compression none    # no compression
```

## Streaming support

rpak supports on-demand asset loading. The runtime reads the index on startup, then loads individual assets as requested:

```rust
// The AssetServer transparently reads from rpak
let texture = asset_server.load("textures/brick.png");
// → Reads from game.rpak if present, falls back to filesystem
```

No code changes needed — the asset server checks rpak first.

## Encryption

Optional AES-256 encryption for sensitive assets:

```bash
renzora pack \
    --input assets/ \
    --output game.rpak \
    --encrypt \
    --key-file secret.key
```

Generate a key:
```bash
renzora keygen --output secret.key
```

The runtime needs the key to load encrypted archives:
```bash
./my_game --asset-key secret.key
```

Or embed the key at compile time (less secure but more convenient):
```rust
app.insert_resource(AssetEncryptionKey::from_bytes(include_bytes!("../secret.key")));
```

## CLI reference

```
renzora pack [OPTIONS]

OPTIONS:
    --input <DIR>              Source assets directory
    --output <FILE>            Output rpak file path
    --compression <TYPE>       lz4 (default), zstd, none
    --compression-level <N>    1-22 for zstd, 1-12 for lz4 (default: 6)
    --encrypt                  Enable AES-256 encryption
    --key-file <FILE>          Encryption key file
    --exclude <PATTERN>        Glob pattern to exclude (repeatable)
    --include <PATTERN>        Only include matching files (repeatable)
    --strip-metadata           Remove .meta files from the archive
    --verbose                  Print each file as it's packed
    --dry-run                  Show what would be packed without writing

renzora unpack [OPTIONS]
    --input <FILE>             rpak file to extract
    --output <DIR>             Extraction directory
    --key-file <FILE>          Decryption key (if encrypted)

renzora pack-info <FILE>
    Show archive metadata: file count, total size, compression ratio

renzora keygen --output <FILE>
    Generate a random AES-256 encryption key
```

## Integration with export

The export pipeline calls `renzora pack` automatically:

1. **Export → Windows/Linux/macOS** collects assets
2. Runs `renzora pack` with project settings
3. Places `game.rpak` alongside the runtime binary

Configure packing in **Project → Settings → Export**:

| Setting | Default | Description |
|---------|---------|-------------|
| Compression | LZ4 | Algorithm for asset compression |
| Exclude patterns | `*.psd, *.blend, *.xcf` | Source files to skip |
| Strip metadata | true | Remove `.meta` files |
| Encrypt | false | Enable encryption |

## Custom asset types

Custom asset loaders work transparently with rpak. The `AssetServer` reads from rpak using the same path the loader registered:

```rust
// This loader works with both filesystem and rpak
impl AssetLoader for DialogueLoader {
    fn extensions(&self) -> &[&str] { &["dialogue"] }
    // ... load() reads bytes from whatever source the AssetServer provides
}
```

No special rpak integration code needed in your loader.
