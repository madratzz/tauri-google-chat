# Archived Learnings

Archived date: 2026-05-22
Source: `.agents/learnings.md` active snapshot

## Tauri and Google Chat

- Google Chat can show an unsupported-browser page in Tauri unless a supported user agent is configured.
- Build the window in Rust when navigation or popup behavior requires custom hooks.
- Do not broadly intercept Workspace-host navigation: Google Chat uses internal frames and panels for services such as Tasks, Contacts, and Drive, and broad interception can create blank windows or break those panels.
- For `on_new_window`, creating a child webview from `about:blank` and returning `NewWindowResponse::Create` follows the expected Tauri flow.
- Window labels must be unique; repeated Marketplace/OAuth/install flows can collide when labels are URL-derived.
- Avoid `expect` in new-window handlers; gracefully deny a popup build failure instead of crashing the app.

## Peek Overlay

- A separate `WebviewWindow`, even always-on-top, is still an OS-level window; use Tauri v2 multi-webview `Window::add_child` for a true in-window overlay.
- The overlay needs Tauri's `unstable` feature.
- Inject its toolbar after `PageLoadEvent::Finished`; sentinel URLs intercepted by `on_navigation` can trigger Pop Out and Close actions.
- Reinject after each in-overlay navigation.
- Store the real URL before navigation cancellation on Windows/WebView2, because the callback is asynchronous and the webview may already expose the sentinel URL.
- Window operations from Windows/WebView2 navigation callbacks should be spawned asynchronously to avoid deadlocks.
- Use logical dimensions (after scale-factor conversion) and respond to `WindowEvent::Resized` to keep the overlay centered and at 85% of the main window.

## Icons and Packaging

- Runtime icon switching requires Tauri's `image-png` feature and `Image::from_bytes`.
- On macOS, `window.set_icon` does not update the Dock or launcher icon. Updating the Dock requires AppKit (`NSData`, `NSImage`, and `setApplicationIconImage:`); the launcher remains controlled by bundled `icon.icns`.
- DMG packaging depends on macOS tools and may fail when the environment lacks required disk-image/Finder automation permissions.

## Verification Pattern

- Run `cargo check`, `npm test`, and `npm run build` after Rust/Tauri changes when the environment supports them.
- Do not track generated `node_modules/`, `src-tauri/target/`, `src-tauri/gen/`, or generated mobile-icon output.
