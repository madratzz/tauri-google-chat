# Google Chat Desktop

A small Rust/Tauri desktop wrapper for Google Chat.

The app opens `https://chat.google.com/` in a native desktop webview and adds a simple native menu with reload, back, forward, and quit actions.

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
