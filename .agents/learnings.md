# Active Learnings

Last updated: 2026-09-14

## Recent Learnings

- WiX limits MSI version fields to 255.255.65535.65535. The UTC-minute app patch number therefore requires `bundle.windows.wix.version`; use `1.1.floor(Z / 65536).(Z % 65536)` for this app's `1.1.Z` releases.
- Google Chat needs the Safari-like user agent in the embedded webview; retest sign-in before changing it.
- Do not broadly intercept Workspace-host navigation: Chat's embedded panels can break or spawn blank windows.
- The true in-window peek UI uses Tauri v2 multi-webview support (`Window::add_child`) and the `unstable` feature.
- The Windows/WebView2 navigation callback is asynchronous; retain the original peek URL and spawn window operations from the callback.
- macOS Dock icon updates require AppKit in addition to Tauri's `window.set_icon`; bundled launcher icons still come from `icon.icns`.

## Patterns

- Configure Tauri windows in Rust when navigation or new-window behavior needs customization.
- Run proportionate Rust/Tauri verification after implementation changes.
- Keep generated folders out of version control.

## Mistakes to Avoid

- Do not use `expect` for child popup construction; deny a failed popup rather than crash the application.
- Do not treat a DMG packaging failure as a compile failure without checking macOS packaging permissions.
- Do not record authentication or other sensitive material in agent files.

## Archive Summary

Detailed May 2026 Tauri, WebView2, icon, and packaging findings were archived during the September 2026 refresh.

## Archive Pointers

- [Archived Learnings Index](../.archive/learnings/INDEX.md)
- [2026-05-22 Tauri webview history](../.archive/learnings/learnings-2026-05-22-tauri-webview-history.md)

- 2026-11-09: On macOS, wry/Tauri `on_navigation` is called for subframe and `javascript:` navigations too. Never use it as a host allowlist on Google pages; apply link policy in `on_new_window` instead.

- 2026-11-10: Signed-out `chat.google.com` redirects to `workspace.google.com/products/chat/`, whose Sign in button is inert in an embedded WKWebView. Send signed-out users to `accounts.google.com/ServiceLogin?service=dynamite&continue=…` instead.

- 2026-11-10: Tauri's default drag-drop handler swallows OS file drops; remote web apps that accept dropped files need `disable_drag_drop_handler()` on every webview.

- 2026-11-10: wry cancels downloads when no download handler is set; every webview needs `on_download` that assigns an absolute destination.

- 2026-11-12: macOS camera/microphone in the webview need `NSCameraUsageDescription`/`NSMicrophoneUsageDescription` in `src-tauri/Info.plist`; wry already grants WebKit media-capture requests by default.

- 2026-11-12: Tauri signs macOS builds with the hardened runtime even for ad-hoc (`-`) identities; mic/camera also need `com.apple.security.device.audio-input` / `device.camera` in `bundle.macOS.entitlements`, or access is denied with no prompt.
