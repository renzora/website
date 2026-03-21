# Building Export Templates

Create platform-specific runtime binaries for game distribution.

## What export templates are

An export template is a **pre-built runtime binary** for a target platform. When a user exports their game, the engine:

1. Takes the export template (runtime binary)
2. Packs the project's assets into `.rpak`
3. Bundles them together as the final distributable

Templates don't include the editor — just the game runtime.

## Template structure

```
templates/
├── windows-x86_64/
│   ├── renzora_runtime.exe
│   └── template.toml
├── linux-x86_64/
│   ├── renzora_runtime
│   └── template.toml
├── macos-universal/
│   ├── renzora_runtime
│   └── template.toml
├── android-arm64/
│   ├── librenzora_runtime.so
│   └── template.toml
└── ios-arm64/
    ├── librenzora_runtime.a
    └── template.toml
```

### template.toml

```toml
[template]
platform = "windows"
arch = "x86_64"
engine_version = "0.1.0"
profile = "release"
features = ["audio", "physics", "networking"]
```

## Building from source

### Debug template

```bash
cargo build --package renzora_runtime --target x86_64-pc-windows-msvc
```

### Release template

```bash
cargo build --package renzora_runtime --target x86_64-pc-windows-msvc --release
```

### Release with LTO (smallest binary)

```bash
CARGO_PROFILE_RELEASE_LTO=true cargo build --package renzora_runtime --target x86_64-pc-windows-msvc --release
```

## Platform targets

| Platform | Target triple | Notes |
|----------|--------------|-------|
| Windows x64 | `x86_64-pc-windows-msvc` | Requires MSVC build tools |
| Linux x64 | `x86_64-unknown-linux-gnu` | Requires system libs |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | Cross-compile with `cross` |
| macOS Intel | `x86_64-apple-darwin` | macOS only |
| macOS Apple Silicon | `aarch64-apple-darwin` | macOS only |
| macOS Universal | Both above, `lipo` merged | macOS only |
| Android ARM64 | `aarch64-linux-android` | Requires NDK |
| iOS | `aarch64-apple-ios` | macOS + Xcode only |

## Feature flags

Control what's included in the template:

```bash
cargo build --package renzora_runtime --release --no-default-features --features "audio,physics"
```

| Feature | Default | Description |
|---------|---------|-------------|
| `audio` | yes | Audio playback and spatial sound |
| `physics` | yes | Avian3D physics simulation |
| `networking` | yes | Multiplayer client/server |
| `scripting_rhai` | yes | Rhai scripting |
| `scripting_lua` | yes | Lua scripting |
| `blueprints` | yes | Visual blueprint execution |

Disabling unused features reduces binary size.

## Template versioning

Templates must match the engine version. The editor checks `template.toml` and warns if there's a mismatch.

Naming convention: `renzora_runtime_0.1.0_windows_x86_64.zip`

## CI/CD builds

GitHub Actions workflow for building templates:

```yaml
jobs:
  build-templates:
    strategy:
      matrix:
        include:
          - os: windows-latest
            target: x86_64-pc-windows-msvc
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: aarch64-apple-darwin
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - run: cargo build --package renzora_runtime --release --target ${{ matrix.target }}
      - uses: actions/upload-artifact@v4
        with:
          name: template-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/renzora_runtime*
```

Templates are attached to GitHub releases and downloaded by the editor.
