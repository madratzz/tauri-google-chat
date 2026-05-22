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
- On macOS, `window.set_icon` does not change the Dock/taskbar icon or App Launcher icon. To dynamically swap the application Dock icon at runtime, you must interface directly with the AppKit API using the `objc` and `cocoa` crates to load `NSData` (calling `dataWithBytes:length:` on the `NSData` class object itself, not via `nil` as that returns `nil`), initialize an `NSImage` via `initWithData:`, and set it using `[NSApp setApplicationIconImage:image]`. The static application icon shown in the App Launcher (Launchpad) and Finder is defined by the bundle's `icon.icns` file on disk, which can be generated using `npm run tauri icon <source_image.png>`.
- DMG packaging uses macOS tools such as `hdiutil` and AppleScript; sandboxed execution can fail without showing the inner error.
- The user does not want to use the system browser at all; all links must stay inside the application.
- The user prefers a "peek" picture-in-picture approach: clicking a link should open an overlay panel centered over the main chat, occupying 85% of its width/height, not navigate the main webview or open a separate OS window.
- Creating a separate `WebviewWindow` (even with `always_on_top`) still appears as a distinct OS window with its own Dock entry and title bar. The user rejected this.
- For true in-window PiP, use Tauri v2's multi-webview support: `Window::add_child(WebviewBuilder, position, size)` to create a child webview overlay inside the main window. Requires the `unstable` feature flag in `Cargo.toml`.
- Inject a floating toolbar into the peek webview via `eval()` after `on_page_load`. Toolbar buttons navigate to sentinel URLs (e.g., `https://peek-action.tauri.internal/expand`) intercepted by `on_navigation`.
- `on_page_load` with `PageLoadEvent::Finished` re-injects the toolbar after each in-peek navigation.
- To "Pop Out" a peek: get the URL, close the child webview, then create a full `WebviewWindow`.
- Listen for `WindowEvent::Resized` on the main window and call `webview.set_size()` and `webview.set_position()` to keep the peek overlay sized to 85% and perfectly centered.
- `inner_size()` returns `PhysicalSize`; convert to logical using `scale_factor()` for layout and size positioning.

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
