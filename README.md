# Google Chat Desktop

A small Rust/Tauri desktop wrapper for Google Chat.

The app opens `https://chat.google.com/` in a native desktop webview.

## Features

- **Tray and background mode** — closing the window hides it to the system tray; click the tray icon to toggle it, or use the tray menu to quit. If no tray is available (some Linux desktops), closing quits as before.
- **Unread badge and notifications** — the unread count from the page title drives the dock/taskbar badge and tray tooltip, and a native notification is shown when it rises while the window is unfocused.
- **Link handling** — Google hosts (`*.google.com`, `*.googleusercontent.com`, sign-in hosts) stay in the app; new-window Google links open in the in-window peek overlay (Esc closes, *Pop Out* opens a separate window). All other links open in your default browser.
- **Remembers your window** — size, position, monitor and maximized state are restored on launch. Minimum size is 600×400 so the app works in split-screen and on small laptops.
- **Zoom** — View → Zoom In / Zoom Out / Actual Size (`Ctrl/Cmd` + `=`, `-`, `0`), persisted between launches.
- **App icon** — View → App Icon switches between color, dark, and white variants; the choice is persisted.
- **Offline banner** — shows a Retry banner when the connection drops and reloads automatically when it returns.
- **Single instance** — launching the app again focuses the existing window.

Preferences are stored in `settings.json` in the platform app-config directory.

## Installation

**Latest release: [v1.2.29904303](https://github.com/madratzz/tauri-google-chat/releases/tag/v1.2.29904303)** (2026-11-09) — fixes Google sign-in on macOS. 1.2 added the tray, unread badge and notifications, external-link handling, window-state memory, zoom, an offline banner and smaller, faster release builds. See the [changelog](CHANGELOG.md) for details.

| Platform | Download |
|---|---|
| Windows (x64) | [`setup.exe`](https://github.com/madratzz/tauri-google-chat/releases/download/v1.2.29904303/Google.Chat.Desktop_1.2.29904303_x64-setup.exe) · [`.msi`](https://github.com/madratzz/tauri-google-chat/releases/download/v1.2.29904303/Google.Chat.Desktop_1.2.29904303_x64_en-US.msi) |
| macOS (Apple Silicon) | [`.dmg`](https://github.com/madratzz/tauri-google-chat/releases/download/v1.2.29904303/Google.Chat.Desktop_1.2.29904303_aarch64.dmg) |
| Debian / Ubuntu | [`.deb`](https://github.com/madratzz/tauri-google-chat/releases/download/v1.2.29904303/Google.Chat.Desktop_1.2.29904303_amd64.deb) |
| Fedora / RPM | [`.rpm`](https://github.com/madratzz/tauri-google-chat/releases/download/v1.2.29904303/Google.Chat.Desktop-1.2.29904303-1.x86_64.rpm) |
| Arch Linux | [`.pkg.tar.zst`](https://github.com/madratzz/tauri-google-chat/releases/download/v1.2.29904303/google-chat-desktop-1.2.29904303-1-x86_64.pkg.tar.zst) · [`PKGBUILD`](https://github.com/madratzz/tauri-google-chat/releases/download/v1.2.29904303/PKGBUILD) |
| Any Linux | [`.AppImage`](https://github.com/madratzz/tauri-google-chat/releases/download/v1.2.29904303/Google.Chat.Desktop_1.2.29904303_amd64.AppImage) |

Older versions are on the [Releases page](https://github.com/madratzz/tauri-google-chat/releases).

### Windows

Run the `.exe` installer or the `.msi` package — no additional dependencies needed.

### macOS

Open the `.dmg`, drag the app to Applications, then right-click → Open the first time to bypass Gatekeeper.

### Linux

**Arch Linux**

```sh
# Install the pre-built package directly
sudo pacman -U google-chat-desktop-*-x86_64.pkg.tar.zst

# Or install via the release PKGBUILD (repackages the released .deb)
curl -O https://github.com/madratzz/tauri-google-chat/releases/latest/download/PKGBUILD
makepkg -si
```

**Debian / Ubuntu**

```sh
sudo apt install ./Google.Chat.Desktop_*_amd64.deb
```

**Fedora / RPM-based**

```sh
sudo rpm -i Google.Chat.Desktop-*.x86_64.rpm
```

**AppImage (any distro)**

```sh
chmod +x Google.Chat.Desktop_*_amd64.AppImage
./Google.Chat.Desktop_*_amd64.AppImage
```

---

## Requirements

- Node.js 18+
- npm
- Rust and Cargo
- Tauri platform prerequisites for your OS

For macOS, Tauri generally needs Xcode Command Line Tools:

```sh
xcode-select --install
```

Install Rust with:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Run

```sh
npm install
npm run dev
```

## Test

```sh
npm test
```

CI (`.github/workflows/ci.yml`) runs `cargo fmt --check`, `clippy -D warnings`, and the unit tests on Linux, macOS and Windows for every pull request.

## Build

```sh
npm run build
```

The bundled app will be created under `src-tauri/target/release/bundle`. Release builds use LTO, a single codegen unit, size optimisation and stripped symbols.

## Versioning

Versions are `X.Y.Z` where `Z = floor(unix_seconds / 60)` (UTC minute). Windows MSI cannot hold a patch above 65,535, so the WiX version is `X.Y.floor(Z / 65536).(Z % 65536)` — configured in `src-tauri/tauri.conf.json` → `bundle.windows.wix.version`.

## Releasing

`.github/workflows/release.yml` runs on a pushed `vX.Y.Z` tag, or manually (Actions → *Publish Release* → Run workflow; it takes the version from `src-tauri/Cargo.toml`). Either way it:

1. creates (or reuses) the draft release for that version;
2. builds macOS, Windows and Linux bundles and uploads them;
3. builds the Arch `.pkg.tar.zst` and a binary `PKGBUILD` from the Linux `.deb` (`.github/scripts/arch-package.sh`) and uploads them;
4. publishes the release as latest once every platform succeeded (untick *publish* on a manual run to keep it as a draft).

Re-running a failed run is safe: the release is reused and Arch assets are replaced.

The repository `PKGBUILD` builds from source and is for local use; the one attached to each release installs the prebuilt binary.

## Notes

Google Chat authentication happens inside the Tauri webview. If Google blocks sign-in because it does not trust the embedded browser on your platform, use Google Chat in your system browser instead.

## Icon Attribution

Google Chat icon artwork from [Google's 2026 Chat SVG](https://www.gstatic.com/dynamite/images/product/chat_2026_220dp.svg). The square source used for desktop bundles preserves the supplied artwork's proportions on a transparent canvas.
