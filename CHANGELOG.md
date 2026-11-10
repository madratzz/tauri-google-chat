# Changelog

## 1.3.29905200 — 2026-11-10T12:00:43Z

Patch release. UTC-minute identifier `29905200`; Windows MSI version `1.3.456.20784`.

- Fix: clicking Download in Chat's file viewer got stuck. With no download handler, WKWebView cancels every download. All webviews now save downloads to the Downloads folder (unique name, sanitised) and show a notification on completion.

## 1.3.29905158 — 2026-11-10T11:18:44Z

Patch release. UTC-minute identifier `29905158`; Windows MSI version `1.3.456.20742`.

- Fix: dragging files into Google Chat (attachments) did nothing. Tauri's native file-drop handler intercepted the drop before the page saw it; it is now disabled on the main, peek and pop-out webviews.

## 1.3.29904943 — 2026-11-10T07:43:45Z

Minor release (backward-compatible feature; minor `2` → `3`). UTC-minute identifier `29904943`; Windows MSI version `1.3.456.20527`.

- New: View → Title Bar → Match System / Light / Dark. The native title bar follows the OS appearance and cannot see Google Chat's in-page theme, so users of Chat's dark theme on a light system can now make it dark. The choice is saved in `settings.json`.

## 1.2.29904922 — 2026-11-10T07:22:32Z

Patch release. UTC-minute identifier `29904922`; Windows MSI version `1.2.456.20506`.

- Fix: sign-in on macOS. When signed out, `chat.google.com` redirects to the Google Workspace marketing page, whose **Sign in** button does nothing in an embedded webview (diagnostic logs showed no navigation or new-window request on click). The app now detects that page and opens Google's sign-in for Chat directly, at most once a minute to avoid redirect loops.

## 1.2.29904373 — 2026-11-09T22:13:05Z

Patch release. UTC-minute identifier `29904373`; Windows MSI version `1.2.456.19957`.

- Fix (macOS sign-in, second attempt): Google sign-in links that open a new window now load in the main window instead of the peek overlay, and the peek overlay is created outside the new-window callback (creating a webview inside it is unsafe on macOS).
- Diagnostics: the app logs new-window requests, page loads and links sent to the browser to stderr (run the app binary from Terminal to see them), and right-click → Inspect Element is enabled.

## 1.2.29904303 — 2026-11-09T21:03:24Z

Patch release (minor stays `2`). UTC-minute identifier `29904303`; Windows MSI version `1.2.456.19887`.

- Fix: pressing **Sign in** did nothing on macOS. The 1.2 link allowlist ran on every navigation, and on macOS (WKWebView) that includes iframes and `javascript:` links, so Google sign-in was cancelled. The link policy now applies only to new-window requests; in-page navigation is never filtered.
- Country Google domains (e.g. `google.co.uk`, `accounts.google.com.pk`) are treated as in-app hosts.

## 1.2.29902849 — 2026-11-08T20:49:24Z

Minor release (backward-compatible features; minor `1` → `2`). UTC-minute identifier `29902849`; Windows MSI version `1.2.456.18433`.

- Speed: release profile with LTO, `codegen-units = 1`, `opt-level = "s"`, `panic = "abort"`, stripped binaries. Peek toolbar is injected once as an initialization script (no repeated `eval`, no `backdrop-filter`), and peek repositioning is debounced during resizes.
- Security: navigation allowlist on every webview — non-Google links open in the system browser instead of taking over the app window.
- Responsive UI: minimum window size lowered to 600×400 (child windows 480×360); window size/position persisted via `tauri-plugin-window-state`; peek overlay clamps to a usable minimum and follows DPI changes.
- UX: system tray with hide-on-close, unread badge + tray tooltip + native notifications, persisted zoom (`Ctrl/Cmd` `=`/`-`/`0`), persisted icon choice, offline Retry banner, Esc closes peek, single-instance focus, proper macOS app and Window menus, Back/Forward on `Alt+Left/Right` outside macOS.
- Robustness: removed panicking `unwrap()`s on the peek path, peek URL state is cleared on close/expand, the WebView2 close workaround is Windows-only, short window labels.
- Build/CI: new PR CI (fmt, clippy, tests on 3 OSes); unit tests for URL policy, titles, unread parsing, peek geometry and zoom; `workflow_dispatch` releases no longer use the branch name as a version; macOS dock icon code migrated from deprecated `cocoa`/`objc` to `objc2`; Tauri CLI pinned; `rust-version` raised to 1.80.

## 1.1.29823045 — 2026-09-14T10:45:22Z

Fix release: retains the 2026 Google Chat icon update and supplies the Windows MSI-specific version `1.1.455.4165`. The app version's UTC-minute identifier is `29823045`; the MSI form maps that value to two 16-bit fields (`floor(Z / 65536)` and `Z % 65536`) to satisfy WiX's numeric limits while preserving version ordering.

## 1.1.29823022 — 2026-09-14T10:22:28Z

Minor release: replaces the application icon with Google's 2026 Google Chat artwork and regenerates the platform icon variants. The major version remains `1`; the minor version increases from `0` to `1` for this backward-compatible user-facing update. The UTC-minute build identifier is `29823022`.
