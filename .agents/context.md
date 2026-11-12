# Active Project Context

Last updated: 2026-11-08

## Project Summary

Google Chat Desktop is a Rust/Tauri 2 desktop wrapper for `https://chat.google.com/`. It uses a native webview, native menus, Google Chat icon variants, and an in-window peek overlay for new-window content.

## Current Goals

- Keep the desktop wrapper stable across its supported Tauri platforms.
- Preserve the in-app navigation and peek-overlay experience.
- Maintain concise active AI context with immutable dated history.

## Current Architecture / Structure

- `package.json` supplies `dev`, `build`, `test`, and `tauri` commands.
- `src-tauri/Cargo.toml` defines the Rust crate and its Tauri dependencies.
- `src-tauri/src/lib.rs` creates the main webview, menus, tray, unread badge/notifications, navigation policy, and the in-window peek overlay.
- `src-tauri/src/util.rs` holds pure, unit-tested helpers (URL allowlist, titles, unread parsing, peek geometry, zoom).
- `src-tauri/src/settings.rs` persists icon and zoom preferences to `settings.json` in the app config dir.
- `.github/workflows/ci.yml` runs fmt/clippy/tests on PRs; `release.yml` builds all platforms (Arch repackaged from the .deb by `.github/scripts/arch-package.sh`) and publishes, from a tag or a manual run.
- `src-tauri/Entitlements.plist` grants mic/camera under the hardened runtime.
- `src-tauri/Info.plist` adds macOS camera/microphone usage strings (merged into the bundle).
- `src-tauri/tauri.conf.json` defines bundle metadata, targets, and icon assets.
- `.agents/` holds concise active context; `.archive/` holds immutable dated snapshots.

## Important Decisions

- The main window is configured in Rust so it can have custom webview and new-window handlers.
- The Safari-like user agent is intentional and should be retested before changing it.
- Google-hosted content (including new-window links) remains in Tauri webviews; non-Google hosts open in the system browser (2026-11-08).
- Closing the main window hides it to the tray only when a tray icon was created; otherwise it quits.
- Windows MSI versions use a deterministic WiX-compatible representation of the app's UTC-minute version when the app patch number exceeds WiX limits.
- Historic context is retained in the dated archives listed below; the active files remain concise.

## Active Constraints

- Follow root `AGENTS.md`, including the `development` → feature branch → pull-request workflow.
- Do not store secrets or sensitive personal/internal data in context files.
- Keep generated folders ignored and prefer source/configuration changes.
- Maintain active logs and archive indexes in reverse chronological order.

## Current Open Questions

- Which platform-specific validation should run before the next release? Unverified on real hardware for 1.2: macOS `objc2` dock-icon code, unread-title format, notification permission prompts, tray on GNOME.
- The repository has no `development` branch; AGENTS.md workflow assumes one.

## Archive Summary

The May 2026 project bootstrap and original active context were archived as a verified dated snapshot during the September 2026 context refresh.

## Archive Pointers

- [Archived Context Index](../.archive/context/INDEX.md)
- [2026-05-22 initial project history](../.archive/context/context-2026-05-22-initial-project-history.md)
