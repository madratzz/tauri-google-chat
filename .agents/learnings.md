# Active Learnings

Last updated: 2026-05-22

## Recent Learnings

- Google Chat may show an unsupported-browser page in a Tauri webview unless a supported user agent is set.
- A macOS `.app` will not show a Finder/Dock icon from `icon.png` alone; it needs generated `.icns` assets and bundle config pointing to them.
- Tauri `on_new_window` can create app-managed child windows for Workspace links.
- Tauri child windows on macOS should use `.window_features(features)` to share the opener webview configuration where possible.
- `on_navigation` callbacks must capture a cloneable `AppHandle`, not a borrowed `App`, because the callback must be `Send + 'static`.
- Tauri runtime icon switching requires the `image-png` feature and `Image::from_bytes`.
- DMG packaging uses macOS tools such as `hdiutil` and AppleScript; sandboxed execution can fail without showing the inner error.

## Patterns

- Use `cargo check`, `npm test`, and `npm run build` after Rust/Tauri changes.
- Keep `src-tauri/target/`, `src-tauri/gen/`, and generated mobile icon folders out of git.
- Prefer creating windows in Rust when navigation/new-window behavior needs customization.

## Mistakes to Avoid

- Do not rely on `tauri.conf.json` window declarations when custom window event hooks are required.
- Do not remove the Safari-like user agent without retesting Google sign-in and Google Chat support.
- Do not treat DMG failure as an app compile failure; check whether macOS packaging permissions are involved.
- Do not store sensitive account or auth material in agent logs.

## Archive Summary

No archived learnings exist yet.

## Archive Pointers

- [Archived Learnings Index](../.archive/learnings/INDEX.md)
