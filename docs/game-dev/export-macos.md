# Export: macOS

Build your game for macOS.

## Prerequisites

- **Xcode Command Line Tools**: `xcode-select --install`
- macOS 12+ recommended for building

## Export settings

Open **Project → Export → macOS** and configure:

| Setting | Default | Description |
|---------|---------|-------------|
| **App Name** | Project name | Name shown in Finder and Dock |
| **Bundle Identifier** | com.studio.game | Unique app ID (reverse domain) |
| **Icon** | Renzora default | `.icns` file for the app icon |
| **Minimum macOS** | 12.0 | Oldest supported macOS version |
| **Architecture** | Universal | Intel + Apple Silicon combined binary |
| **Resolution** | 1280×720 | Default window size |
| **Fullscreen** | false | Launch in fullscreen |
| **Retina** | true | Support high-DPI displays |

## Building

1. Configure settings
2. Click **Export**
3. Choose an output directory

### Architecture options

| Option | Description |
|--------|-------------|
| **Universal** | Fat binary for both Intel and Apple Silicon. Larger but runs everywhere. |
| **x86_64** | Intel Macs only |
| **aarch64** | Apple Silicon only (M1/M2/M3/M4) |

## Output

```
My Game.app/
├── Contents/
│   ├── Info.plist
│   ├── MacOS/
│   │   └── my_game          # Executable
│   └── Resources/
│       ├── game.rpak         # Packed assets
│       └── AppIcon.icns      # App icon
```

## Code signing and notarization

Required for distribution outside the Mac App Store. Without it, Gatekeeper blocks the app.

### Signing

```bash
codesign --deep --force --sign "Developer ID Application: Your Name (TEAMID)" "My Game.app"
```

### Notarization

```bash
# Create a zip for notarization
ditto -c -k --keepParent "My Game.app" MyGame.zip

# Submit to Apple
xcrun notarytool submit MyGame.zip --apple-id you@email.com --team-id TEAMID --password app-specific-password --wait

# Staple the ticket
xcrun stapler staple "My Game.app"
```

### DMG packaging

Create a `.dmg` for distribution:

```bash
hdiutil create -volname "My Game" -srcfolder "My Game.app" -ov MyGame.dmg
```

## Mac App Store

1. Sign with "Apple Distribution" certificate (not "Developer ID")
2. Create an App Store Connect listing
3. Upload via `xcrun altool` or Transporter
4. Sandbox entitlements are required

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "App is damaged" | Notarize the app, or users run `xattr -cr "My Game.app"` |
| GPU errors on Intel Mac | Ensure Metal is supported (macOS 10.14+) |
| Crash on Apple Silicon | Build Universal binary or aarch64-specific |
| Slow first launch | Shader compilation on first run — normal |
