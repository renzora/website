# Export: Windows

Build your game for Windows desktop.

## Prerequisites

- **Visual Studio Build Tools** (or full Visual Studio) with the "Desktop development with C++" workload
- **Windows SDK** (installed with Build Tools)

## Export settings

Open **Project → Export → Windows** and configure:

| Setting | Default | Description |
|---------|---------|-------------|
| **App Name** | Project name | Displayed name and exe filename |
| **Version** | 1.0.0 | Shown in file properties |
| **Icon** | Renzora default | `.ico` file for the executable |
| **Resolution** | 1280×720 | Default window size |
| **Fullscreen** | false | Launch in fullscreen mode |
| **VSync** | true | Sync to monitor refresh rate |
| **Console** | false | Show a debug console window |

## Building

1. Configure settings above
2. Click **Export**
3. Choose an output directory
4. Wait for the build to complete

### Build profiles

| Profile | Description |
|---------|-------------|
| **Debug** | Fast compile, includes debug info, larger binary |
| **Release** | Optimized, smaller binary, slower compile |
| **Release + LTO** | Maximum optimization, smallest binary, slowest compile |

## Output files

The export produces:

```
my_game/
├── my_game.exe          # Main executable
├── game.rpak            # Packed game assets
└── my_game.pdb          # Debug symbols (Debug builds only)
```

## Distribution

### Direct download

Zip the output folder and distribute. No installer needed — the game is a single folder.

### Steam

1. Use the Steamworks SDK to create a depot
2. Point the depot at the export folder
3. Upload via `steamcmd`

### itch.io

1. Zip the export folder
2. Upload to your itch.io project page
3. Mark as "Windows" and "Executable"

## Code signing (optional)

Sign the exe to avoid "Unknown publisher" warnings:

1. Obtain a code signing certificate (DigiCert, Sectigo, etc.)
2. Use `signtool.exe` from the Windows SDK:
   ```
   signtool sign /f cert.pfx /p password /tr http://timestamp.digicert.com /td sha256 my_game.exe
   ```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Missing DLLs | Ensure Visual C++ Redistributable is installed or bundle `vcruntime140.dll` |
| Black screen on launch | Update GPU drivers, check minimum OpenGL/Vulkan support |
| Antivirus blocks exe | Code sign the executable, or submit to AV vendor for whitelisting |
| Slow first launch | Normal — shaders compile on first run. Subsequent launches are faster |
