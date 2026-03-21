# Export: Android

Build your game for Android phones and tablets.

## Prerequisites

- **Android SDK** (API 28+) — install via [Android Studio](https://developer.android.com/studio) or `sdkmanager`
- **Android NDK** (r25+) — for native Rust compilation
- **JDK 17** — for APK/AAB building
- **Rust Android targets**: `rustup target add aarch64-linux-android armv7-linux-androideabi`

Set environment variables:

```bash
export ANDROID_SDK_ROOT=~/Android/Sdk
export ANDROID_NDK_ROOT=~/Android/Sdk/ndk/25.2.9519653
```

## Export settings

Open **Project → Export → Android** and configure:

| Setting | Default | Description |
|---------|---------|-------------|
| **Package Name** | com.studio.game | Unique app identifier |
| **App Name** | Project name | Display name on device |
| **Min SDK** | 28 (Android 9) | Oldest supported Android version |
| **Target SDK** | 34 (Android 14) | Latest tested version |
| **Version Code** | 1 | Integer, increment each release |
| **Version Name** | 1.0.0 | User-facing version string |
| **Orientation** | Landscape | Landscape, Portrait, or Auto |
| **Icon** | Renzora default | Adaptive icon (foreground + background layers) |

### Permissions

Enable only what your game needs:

| Permission | When needed |
|------------|-------------|
| **Internet** | Multiplayer games |
| **Vibrate** | Haptic feedback |
| **External Storage** | Save files to shared storage |
| **Microphone** | Voice chat |

## Keystore setup

Android requires all APKs/AABs to be signed.

### Debug keystore (development)

Auto-generated. Fine for testing, not for publishing.

### Release keystore

```bash
keytool -genkey -v -keystore release.keystore -alias mykey -keyalg RSA -keysize 2048 -validity 10000
```

**Keep this file safe** — you cannot update your app without the same keystore.

Set in export settings: **Keystore Path**, **Keystore Password**, **Key Alias**, **Key Password**.

## Building

### APK (for testing)

1. Click **Export → Build APK**
2. Install on device: `adb install my_game.apk`

### AAB (for Google Play)

1. Click **Export → Build AAB**
2. Upload to Google Play Console

AAB is required for Google Play. It produces optimized APKs for each device configuration.

## Testing

### On device

```bash
adb install -r my_game.apk
adb logcat | grep renzora     # view logs
```

### On emulator

Use Android Studio's emulator. Enable **GPU acceleration** for acceptable performance.

## Touch input

Touch maps to mouse input automatically:

- **Single touch** → `mouse_x`, `mouse_y`, `mouse_button_left`
- **Multi-touch** — use `touch_count` and `touch_x(index)`, `touch_y(index)`

Add on-screen controls with the UI system (virtual joystick, action buttons).

## Performance considerations

- **Target 30 FPS** minimum on mid-range devices
- **Reduce draw calls** — use material batching and texture atlases
- **Lower resolution textures** — create `textures_mobile/` variants
- **Disable expensive post-processing** — SSAO, SSR, high-quality bloom
- **Test on real devices** — emulators don't reflect actual GPU performance

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Build fails: NDK not found | Set `ANDROID_NDK_ROOT` environment variable |
| Black screen on device | Check Vulkan support — some older devices need OpenGL ES fallback |
| APK too large (>150MB) | Compress textures, use AAB for split delivery |
| Touch not responding | Ensure UI elements have proper touch target sizes (48dp minimum) |
