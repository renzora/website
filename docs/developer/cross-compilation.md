# Cross-Compilation

Build Renzora for a different platform than your development machine.

## Supported combinations

| Host | Targets |
|------|---------|
| **Windows** | Windows (native), Linux (via WSL or cross) |
| **Linux** | Linux (native), Windows (mingw), Android (NDK) |
| **macOS** | macOS (native), iOS (native), Windows (cross), Linux (cross), Android (NDK) |

iOS builds **require macOS**. All other targets can be built from any host.

## Rust cross-compilation setup

### 1. Add the target

```bash
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-linux-android
rustup target add aarch64-apple-ios
```

### 2. Install a linker

Each target needs a compatible linker. Configure in `~/.cargo/config.toml`:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "x86_64-linux-gnu-gcc"

[target.aarch64-linux-android]
linker = "/path/to/ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android33-clang"
```

### 3. Build

```bash
cargo build --target x86_64-unknown-linux-gnu --release
```

## Using `cross`

The [cross](https://github.com/cross-rs/cross) tool uses Docker containers with pre-configured toolchains:

```bash
cargo install cross

# Build for Linux from any host
cross build --target x86_64-unknown-linux-gnu --release

# Build for ARM Linux
cross build --target aarch64-unknown-linux-gnu --release
```

`cross` handles linkers, system libraries, and sysroots automatically.

## Platform-specific setup

### Android NDK

```bash
# Install via Android Studio or sdkmanager
sdkmanager "ndk;25.2.9519653"

export ANDROID_NDK_ROOT=$HOME/Android/Sdk/ndk/25.2.9519653

# Add targets
rustup target add aarch64-linux-android armv7-linux-androideabi

# Build
cargo build --target aarch64-linux-android --release
```

### iOS (macOS only)

```bash
# Install Xcode command line tools
xcode-select --install

# Add targets
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim  # for simulator

# Build
cargo build --target aarch64-apple-ios --release
```

### Linux → Windows (mingw)

```bash
# Ubuntu
sudo apt install gcc-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu

cargo build --target x86_64-pc-windows-gnu --release
```

### macOS Universal Binary

Build both architectures and combine:

```bash
cargo build --target x86_64-apple-darwin --release
cargo build --target aarch64-apple-darwin --release

lipo -create \
    target/x86_64-apple-darwin/release/renzora_runtime \
    target/aarch64-apple-darwin/release/renzora_runtime \
    -output renzora_runtime_universal
```

## CI/CD cross-compilation

GitHub Actions matrix for all platforms:

```yaml
strategy:
  matrix:
    include:
      - { os: ubuntu-latest, target: x86_64-unknown-linux-gnu }
      - { os: ubuntu-latest, target: aarch64-unknown-linux-gnu, use_cross: true }
      - { os: windows-latest, target: x86_64-pc-windows-msvc }
      - { os: macos-latest, target: aarch64-apple-darwin }
      - { os: macos-latest, target: x86_64-apple-darwin }
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `linker not found` | Install the cross-linker or use `cross` |
| `cannot find -lssl` | Install target's OpenSSL dev package or use `vendored` feature |
| `failed to run custom build` | Build scripts may need host tools — check `build.rs` dependencies |
| `undefined reference to...` | Wrong linker or missing system library for the target |
| Android: `cannot find crt*.o` | NDK path or API level mismatch — check `ANDROID_NDK_ROOT` |
| iOS: `code signing` | Sign with `codesign` after building |
