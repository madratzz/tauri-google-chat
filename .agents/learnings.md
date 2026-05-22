# Active Learnings

Last updated: 2026-05-22

## Recent Learnings

- Google Chat may show an unsupported-browser page in a Tauri webview unless a supported user agent is set.
- A macOS `.app` will not show a Finder/Dock icon from `icon.png` alone; it needs generated `.icns` assets and bundle config pointing to them.
- Tauri `on_new_window` can create app-managed child windows for Workspace links.
- Tauri child windows on macOS should use `.window_features(features)` to share the opener webview configuration where possible.
- `on_navigation` callbacks must capture a cloneable `AppHandle`, not a borrowed `App`, because the callback must be `Send + 'static`.
- Avoid using broad `on_navigation` interception for Google Workspace hosts in Google Chat; Chat uses embedded side panels/frames for Tasks, Contacts, Drive, and similar services, and intercepting those navigations can spawn blank windows or break the embedded panel.
- For Tauri `on_new_window`, prefer creating the child webview with `about:blank` and returning `NewWindowResponse::Create`; the runtime attaches the requested popup URL to that webview.
- Tauri window labels must be unique. URL-derived labels can collide when Google Marketplace/OAuth/install flows open the same popup more than once, causing `.build().expect(...)` crashes.
- Tauri runtime icon switching requires the `image-png` feature and `Image::from_bytes`.
- DMG packaging uses macOS tools such as `hdiutil` and AppleScript; sandboxed execution can fail without showing the inner error.
- The user does not want to use the system browser at all; all links must navigate inside the application webview.
- To open links in the same window when they request a new window (e.g. `target="_blank"`), call `.navigate(url)` on the active window inside `on_new_window` and return `NewWindowResponse::Deny`.
- Provide an expand/pop-out feature (e.g., `Cmd+E`) so the user can pop the current page out into a separate Tauri child window and return the original window to its base state.

## Patterns

- Use `cargo check`, `npm test`, and `npm run build` after Rust/Tauri changes.
- Keep `src-tauri/target/`, `src-tauri/gen/`, and generated mobile icon folders out of git.
- Prefer creating windows in Rust when navigation/new-window behavior needs customization.

## Mistakes to Avoid

- Do not rely on `tauri.conf.json` window declarations when custom window event hooks are required.
- Do not treat all `docs.google.com`, `drive.google.com`, `tasks.google.com`, or similar navigations as user-clicked external links; some are internal Google Chat sidebar/plugin frames.
- Do not `expect` child popup window creation in new-window handlers; return `NewWindowResponse::Deny` on build failure so popup problems do not crash the whole app.
- Do not remove the Safari-like user agent without retesting Google sign-in and Google Chat support.
- Do not treat DMG failure as an app compile failure; check whether macOS packaging permissions are involved.
- Do not store sensitive account or auth material in agent logs.
- Do not route link clicks to the default system browser; keep all navigations within Tauri webviews.

## Archive Summary

No archived learnings exist yet.

## Archive Pointers

- [Archived Learnings Index](../.archive/learnings/INDEX.md)
