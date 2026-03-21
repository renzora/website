# Export: iOS & tvOS

Build your game for iPhone, iPad, and Apple TV.

## Prerequisites

- **macOS** — iOS builds require a Mac
- **Xcode 15+** — includes iOS SDK and simulator
- **Apple Developer Account** ($99/year) — required for device testing and App Store
- **Rust iOS targets**: `rustup target add aarch64-apple-ios aarch64-apple-ios-sim`

## Export settings

Open **Project → Export → iOS** and configure:

| Setting | Default | Description |
|---------|---------|-------------|
| **Bundle ID** | com.studio.game | Unique app identifier |
| **App Name** | Project name | Display name on device |
| **Deployment Target** | 15.0 | Oldest supported iOS version |
| **Device Family** | Universal | iPhone, iPad, or Universal |
| **Orientation** | Landscape | Landscape, Portrait, or All |
| **Icon** | Renzora default | App icon (1024×1024 source) |
| **Launch Screen** | Default | Storyboard or static image |

### tvOS settings

| Setting | Default | Description |
|---------|---------|-------------|
| **Enable tvOS** | false | Build for Apple TV |
| **tvOS Deployment Target** | 15.0 | Oldest supported tvOS |
| **Top Shelf Image** | none | Wide banner for tvOS home screen |

## Provisioning and certificates

### Development (testing)

1. In Xcode: **Settings → Accounts → Manage Certificates → +** → Apple Development
2. Register test devices in Apple Developer Portal (or use automatic signing)
3. Create a development provisioning profile

### Distribution (App Store)

1. Create an "Apple Distribution" certificate
2. Create an App Store provisioning profile
3. Assign both in export settings

## Building

1. Click **Export** — generates an Xcode project
2. Open the `.xcodeproj` in Xcode
3. Select your device or simulator
4. **Product → Build** (Cmd+B) or **Product → Archive** (for App Store)

### Simulator vs device

| Target | Description |
|--------|-------------|
| **Simulator** | Runs on Mac, x86_64/arm64-sim, no provisioning needed, no GPU features |
| **Device** | Runs on real hardware, requires signing, full GPU support |

## Touch input

Touch is mapped automatically:

- **Tap** → `mouse_button_left`
- **Touch position** → `mouse_x`, `mouse_y`
- **Multi-touch** → `touch_count`, `touch_x(index)`, `touch_y(index)`

Use the UI system for on-screen controls (virtual joystick, buttons).

## tvOS specifics

### Controller requirement

Apple TV requires **game controller support** (Siri Remote or MFi controller). Touch is not available.

- Siri Remote trackpad → `gamepad_left_x/y`
- Siri Remote buttons → `gamepad_south` (select), `gamepad_east` (menu)
- MFi controllers → full gamepad support

### UI guidelines

- Focus-based navigation (no touch)
- Larger UI elements (viewed from 3+ meters)
- Top Shelf image: 1920×720 or 2320×720

## App Store submission

1. **Archive** in Xcode (Product → Archive)
2. **Distribute App** → App Store Connect
3. Fill out App Store listing (screenshots, description, rating)
4. Submit for review

### Required screenshots

| Device | Size |
|--------|------|
| iPhone 6.7" | 1290×2796 |
| iPhone 6.5" | 1242×2688 |
| iPad 12.9" | 2048×2732 |
| Apple TV | 1920×1080 |

## Performance considerations

- iPhones have excellent GPUs — most effects run well
- Target **60 FPS** on iPhone (120 FPS on ProMotion devices is a bonus)
- Use **Metal** (default) — Renzora uses wgpu which targets Metal on Apple platforms
- Watch **thermal throttling** — sustained load causes the device to reduce GPU clock
- Test on the oldest device you support

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "Untrusted Developer" | On device: Settings → General → VPN & Device Management → Trust |
| Signing errors | Check provisioning profile matches Bundle ID and device |
| Simulator crash | Use arm64-sim target on Apple Silicon Macs |
| Black screen | Check Metal support on deployment target iOS version |
