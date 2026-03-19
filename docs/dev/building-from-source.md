# Building from Source

Clone the engine repo and build your own custom version of Renzora Engine.

## Prerequisites

- **Rust** 1.85+ ([install](https://rustup.rs))
- **Git**
- **C/C++ toolchain** — MSVC on Windows, Clang on macOS, GCC on Linux
- **CMake** (for some native dependencies)

### Platform-specific

**Windows:**
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with "Desktop development with C++"

**macOS:**
```bash
xcode-select --install
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt install build-essential cmake pkg-config libasound2-dev libudev-dev libxkbcommon-dev libwayland-dev
```

## Clone and build

```bash
git clone https://github.com/renzora/engine.git
cd engine
cargo run --release
```

The first build takes several minutes. Subsequent builds are incremental and much faster.

## Project structure

```
engine/
├── Cargo.toml              # workspace root
├── src/
│   ├── editor.rs           # editor binary entry point
│   └── runtime.rs          # runtime library entry point
├── crates/
│   ├── core/               # engine core (runtime, assets, scripting, networking)
│   ├── editor/             # editor panels and tools
│   ├── ui/                 # UI framework (docking, widgets, theme)
│   └── postprocessing/     # post-processing effects
├── assets/                 # engine default assets
└── _legacy_src/            # legacy code (reference only, not compiled)
```

## Running the editor

```bash
cargo run --release --bin renzora
```

## Running just the runtime (no editor)

```bash
cargo run --release --bin renzora-runtime
```

## Build export templates

Export templates are pre-compiled runtime binaries for each target platform:

```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS
cargo build --release --target aarch64-apple-darwin

# Android (requires Android NDK)
cargo ndk -t arm64-v8a build --release

# Web (requires wasm-pack)
cargo build --release --target wasm32-unknown-unknown
```

## What's next?

- [Architecture](/docs/dev/architecture) — understand how the engine is structured
- [Building Plugins](/docs/dev/plugins) — extend the engine with custom functionality
- [Contributing](/docs/dev/contributing) — submit changes to the main repo
