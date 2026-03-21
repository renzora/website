# Export: Linux

Build your game for Linux desktop.

## Prerequisites

- **GCC** or **Clang** toolchain
- Development libraries: `libx11-dev`, `libasound2-dev`, `libudev-dev`, `libxkbcommon-dev`, `libwayland-dev`

On Ubuntu/Debian:
```bash
sudo apt install build-essential libx11-dev libasound2-dev libudev-dev libxkbcommon-dev libwayland-dev
```

## Export settings

Open **Project → Export → Linux** and configure:

| Setting | Default | Description |
|---------|---------|-------------|
| **Binary Name** | Project name | Output executable name |
| **Architecture** | x86_64 | Target CPU (x86_64 or aarch64) |
| **Resolution** | 1280×720 | Default window size |
| **Fullscreen** | false | Launch in fullscreen |
| **Wayland** | true | Enable Wayland support (X11 always included) |

## Building

1. Configure settings
2. Click **Export**
3. Choose an output directory

## Output files

```
my_game/
├── my_game              # Executable (no extension)
└── game.rpak            # Packed game assets
```

Mark as executable: `chmod +x my_game`

## Packaging

### AppImage

Portable, runs on most distros without installation:

```bash
# Use linuxdeploy to package
linuxdeploy --appdir AppDir --executable my_game --output appimage
```

### Flatpak

For sandboxed distribution via Flathub:

1. Create a Flatpak manifest (`com.yourstudio.mygame.yml`)
2. Build with `flatpak-builder`
3. Submit to Flathub

### Steam (Proton not needed)

Native Linux builds run directly. Upload to Steam via `steamcmd` with a Linux depot.

## ARM64 builds

For Raspberry Pi, ARM laptops, and ARM servers:

1. Set **Architecture** to `aarch64` in export settings
2. Cross-compile from x86_64 (requires `aarch64-linux-gnu-gcc`) or build natively on ARM hardware

## Troubleshooting

| Issue | Solution |
|-------|----------|
| No audio | Install ALSA or PulseAudio libraries |
| Wayland crash | Set env `WINIT_UNIX_BACKEND=x11` to force X11 |
| GPU not detected | Install Vulkan drivers (`mesa-vulkan-drivers` or proprietary) |
| Permission denied | Run `chmod +x my_game` |
