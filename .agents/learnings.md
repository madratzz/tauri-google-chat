# Active Learnings

Last updated: 2026-09-14

## Recent Learnings

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
