# Archived Project Context

Archived date: 2026-05-22
Source: `.agents/context.md` active snapshot

## Project Summary

The project was a Rust/Tauri desktop wrapper for Google Chat. It opened Google Chat in a native webview, supplied app menus, bundled Google Chat icon assets, and managed Google Workspace popups in-app.

## Goals and Architecture at Archive Time

- Keep the macOS wrapper stable while retaining Windows and Linux potential.
- `package.json` supplied `dev`, `build`, `test`, and `tauri` scripts.
- `src-tauri/Cargo.toml` defined the Rust crate and Tauri dependencies.
- `src-tauri/src/lib.rs` built the main webview, menus, icon switching, and child webviews.
- `src-tauri/tauri.conf.json` defined bundle metadata and icon assets.
- `src-tauri/icons/` contained app and color/dark/white Google Chat icons.

## Important Decisions

- Construct the main Tauri window in Rust so navigation and new-window handlers can be attached.
- Use a Safari-like user agent because the default embedded browser was rejected by Google after sign-in.
- Keep Workspace popups inside app-managed webviews.
- Use a color Google Chat icon for the bundled macOS app and provide color, dark, and white runtime icon choices.
- Treat DMG packaging as environment-permission-sensitive.

## Open Questions at Archive Time

- Whether to add platform-specific user agents and bundle targets for Windows and Linux.
- Whether child windows needed their own navigation controls.
- Whether same-tab Workspace navigation needed different handling.
- Whether icon choice should persist between launches.
- Whether DMG should be enabled by default or built as an elevated explicit step.

## Redactions

- A historical local filesystem path was replaced with `<LOCAL_PROJECT_PATH>` rather than copied.
