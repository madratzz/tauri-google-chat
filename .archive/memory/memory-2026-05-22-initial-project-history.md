# Archived Memory

Archived date: 2026-05-22
Source: `.agents/memory.md` active snapshot

## Stable Facts at Archive Time

- Local project path: `<LOCAL_PROJECT_PATH>`.
- Tauri 2/Rust wrapper for Google Chat; product name `Google Chat Desktop`; crate `google-chat-desktop`; bundle identifier `com.madratzz.google-chat-desktop`.
- The repository used the official npm registry through `.npmrc`.
- The primary source was `src-tauri/src/lib.rs`; bundle configuration was `src-tauri/tauri.conf.json`.

## Recorded Preferences and Conventions

- Keep links inside Tauri rather than sending them to the system browser.
- Use the Dashboard Icons/selfh.st Google Chat icon (CC BY 4.0 attribution in the README).
- Maintain AI-agent context, memory, learnings, logs, and archives.
- Keep active files under `.agents/` and old material under `.archive/`.
- Use concise imperative commit messages.

## Technical Facts Worth Retaining

- Google Chat needs the Safari-like user agent for reliable sign-in.
- macOS launcher icons require `icon.icns` and bundle configuration; PNG alone is insufficient.
- Runtime macOS Dock icon swapping uses AppKit calls in addition to Tauri window icons.
- Do not copy credentials or authentication data to agent files.

## Redactions

- A historical local filesystem path was replaced with `<LOCAL_PROJECT_PATH>`.
