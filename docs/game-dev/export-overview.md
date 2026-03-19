# Export Overview

Build your game for multiple platforms from a single project.

## Supported Platforms

| Platform | Format | Notes |
|----------|--------|-------|
| **Windows** (x64) | `.exe` | Most common target |
| **macOS** (Intel + Apple Silicon) | `.app` | Universal binary |
| **Linux** (x64) | ELF binary | AppImage recommended |
| **Android** (ARM64 + x86_64) | `.apk` | Requires signing certificate |
| **iOS** (ARM64) | `.ipa` | Requires Apple developer account |
| **iPadOS** (ARM64) | `.ipa` | Same as iOS |
| **Apple TV** (tvOS) | `.ipa` | Requires Apple developer account |
| **Web** (WASM) | `.wasm` + `.js` | Runs in browsers |

## How to Export

1. Open your project in the editor
2. Go to **File → Export** (or click the export button in the title bar)
3. Select your target platform
4. Configure export settings:
   - Window title and icon
   - Resolution and fullscreen defaults
   - Include/exclude assets
5. Click **Export**

## What Happens During Export

1. **Asset packing** — all project assets are packed into a `.rpak` archive
2. **Template bundling** — a pre-built runtime binary for the target platform is combined with your assets
3. **Configuration** — your `project.toml` settings are embedded

## Export Templates

Each platform requires a **pre-compiled runtime template**. These are downloaded automatically on first export and cached for future builds.

Templates are available on the [download page](/download) or can be built from source (see [Building Export Templates](/docs/developer/export-templates)).

## Packaging Modes

### Standalone
Assets are copied to an `assets/` folder alongside the executable. Good for development and testing.

### RPK Archive
Assets are packed into a single `.rpak` file. Smaller distribution, harder to tamper with. Recommended for releases.

## Multiplayer Exports

If your project uses networking, the export includes an optional **server binary** alongside the game client. Deploy the server to any Linux host for dedicated multiplayer.

## Platform-Specific Notes

### Android
- Requires a signing certificate (`.keystore` file)
- Both ARM64 and x86_64 architectures supported
- Fire TV exports use the same APK format

### iOS / tvOS
- Requires Apple Developer Program membership
- Builds produce an `.ipa` for installation via Xcode or TestFlight

### Web
- Produces a WASM bundle with HTML wrapper
- Some features may be limited (no native file system, no UDP networking)
- Use WebSocket transport for multiplayer
