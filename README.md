# Google Chat Desktop

A small Rust/Tauri desktop wrapper for Google Chat.

The app opens `https://chat.google.com/` in a native desktop webview and adds a simple native menu with reload, back, forward, and quit actions.

The Icon menu can switch the active window icon between the color, dark, and white Google Chat variants.

## Installation

Download the latest release from the [Releases page](https://github.com/madratzz/tauri-google-chat/releases/latest).

### Windows

Run the `.exe` installer or the `.msi` package — no additional dependencies needed.

### macOS

Open the `.dmg`, drag the app to Applications, then right-click → Open the first time to bypass Gatekeeper.

### Linux

**Arch Linux**

```sh
# Install the pre-built package directly
sudo pacman -U google-chat-desktop-*-x86_64.pkg.tar.zst

# Or build from source using the PKGBUILD
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

## Build

```sh
npm run build
```

The bundled app will be created under `src-tauri/target/release/bundle`.

## Notes

Google Chat authentication happens inside the Tauri webview. If Google blocks sign-in because it does not trust the embedded browser on your platform, use Google Chat in your system browser instead.

## Icon Attribution

Google Chat icon from [selfh.st/icons](https://github.com/selfhst/icons), surfaced by [Dashboard Icons](https://dashboardicons.com/icons/external/google-chat), licensed under CC BY 4.0.
