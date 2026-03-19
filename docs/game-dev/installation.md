# Installation

Get Renzora Engine running on your machine in a few minutes.

## System Requirements

- **OS:** Windows 10+, macOS 12+, or Ubuntu 22.04+
- **GPU:** Any GPU with Vulkan, Metal, or DX12 support
- **RAM:** 4 GB minimum, 8 GB recommended
- **Disk:** ~500 MB for the editor

## Download

Head to the [download page](/download) and grab the installer for your platform.

### Windows

Download the `.exe` installer and run it. The editor will be added to your Start menu. Alternatively, download the portable `.zip` and extract it anywhere.

### macOS

Download the `.dmg`, open it, and drag Renzora to your Applications folder.

> On first launch, you may need to right-click and choose Open, then confirm in the security dialog.

### Linux

Download the `.AppImage`, make it executable, and run it:

```bash
chmod +x Renzora-r1-alpha4.AppImage
./Renzora-r1-alpha4.AppImage
```

Debian/Ubuntu users can also use the `.deb` package.

### Build from source

If you prefer to compile from source, you'll need Rust 1.85+ and Git:

```bash
git clone https://github.com/renzora/engine.git
cd engine
cargo run --release
```

## What's next?

Now that you have the engine installed, [create your first project](/docs/getting-started/first-project).
