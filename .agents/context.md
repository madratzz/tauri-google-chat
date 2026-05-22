# Active Project Context

Last updated: 2026-05-22

## Project Summary

This project is a Rust/Tauri desktop wrapper for Google Chat. It opens `https://chat.google.com/` in a native webview, adds app menus, supports macOS app bundling, includes Google Chat icon assets, and handles Google Workspace links inside app-managed windows.

## Current Goals

- Keep the Google Chat desktop wrapper stable and usable on macOS.
- Preserve cross-platform potential for Windows and Linux with Tauri.
- Keep app context, memory, logs, and learnings available for future AI agents.

## Current Architecture / Structure

- `package.json` defines Tauri CLI scripts: `dev`, `build`, `test`, and `tauri`.
- `.npmrc` pins this repo to the official npm registry.
- `src-tauri/Cargo.toml` defines the Rust crate and Tauri dependency.
- `src-tauri/src/lib.rs` creates the main Google Chat webview, native menus, icon switching, and internal child windows for explicit Workspace popup links.
- `src-tauri/tauri.conf.json` defines bundle metadata, app category, bundle target, and icon assets.
- `src-tauri/icons/` contains desktop icon assets plus color, dark, and white Google Chat variants.
- `.agents/` contains active AI-agent context.
- `.archive/` contains dated archived AI-agent context indexes and future archive files.

## Important Decisions

- The main Tauri window is created in Rust, not solely from `tauri.conf.json`, so new-window and navigation handlers can be attached.
- Google Chat uses a Safari-like user agent because Google rejected the default embedded browser after sign-in.
- Explicit Google Workspace popup links open inside app-managed child webview windows rather than the system browser.
- The default bundled macOS icon is the color Google Chat icon; runtime menu options can switch active window icon to color, dark, or white.
- DMG packaging can work, but it requires elevated macOS disk-image/Finder automation permissions in this environment.
- The default `npm run build` target builds a macOS `.app` bundle.

## Active Constraints

- Do not store secrets, API keys, tokens, passwords, private keys, or credentials in agent files.
- Keep generated folders ignored: `node_modules/`, `src-tauri/target/`, `src-tauri/gen/`, mobile icon output folders.
- Use official npm registry for this project.
- Prefer source/config changes over generated build output.
- Preserve useful history in `.archive/` before major context restructures.
- Maintain reverse chronological logs and archive indexes.

## Current Open Questions

- Should Windows/Linux platform-specific user agents and bundle targets be added?
- Should internal child windows get their own navigation menu controls?
- Should same-tab Workspace navigations be handled differently, or left inside the main webview?
- Should app icon selection persist between launches?
- Should the DMG target be re-enabled by default or kept as an explicit elevated build step?

## Archive Summary

No archived context files exist yet. Current active files capture the initial project bootstrap and agent-context setup.

## Archive Pointers

- [Archived Context Index](../.archive/context/INDEX.md)
