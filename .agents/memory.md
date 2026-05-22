# Active Memory

Last updated: 2026-05-22

## Stable Project Facts

- Project root: `/Users/madratzz/Documents/Codex/Tauri-Based-Google-Chat`.
- The app is a Tauri 2 / Rust desktop wrapper for Google Chat.
- Main product name: `Google Chat Desktop`.
- Rust crate name: `google-chat-desktop`.
- Bundle identifier: `com.madratzz.google-chat-desktop`.
- The app currently builds a macOS `.app` bundle by default.
- Git is initialized on branch `main`.
- The repo uses local `.npmrc` with `registry=https://registry.npmjs.org/`.
- Rust was installed via Homebrew in this environment.

## User Preferences

- User wants git initialized and changes committed during development.
- User prefers the official npm registry for this project.
- User wanted the Dashboard Icons/selfh.st Google Chat icon.
- User wanted Google Workspace links to open inside the app.
- User asked for persistent AI-agent context, logs, memory, learnings, and archives.

## Naming Conventions

- Archive filenames use `DD-MM-YY` plus optional lowercase hyphenated slug.
- Active agent files live under `.agents/`.
- Older context material lives under `.archive/`.
- Commit messages so far are concise imperative summaries.

## Important Entities

- Google Chat URL: `https://chat.google.com/`.
- Workspace link hosts handled in app include Docs, Drive, Gmail, Calendar, Meet, Contacts, Keep, Tasks, and Jamboard.
- Icon source: Dashboard Icons page backed by `selfh.st/icons`, CC BY 4.0.
- Main source file: `src-tauri/src/lib.rs`.
- Bundle config: `src-tauri/tauri.conf.json`.

## Do Not Forget

- Do not copy secrets or credentials into `.agents/` or `.archive/`.
- macOS `.app` icon requires `icon.icns` and `CFBundleIconFile`; PNG alone is not enough.
- Google Chat rejected the default embedded browser, so the Safari-like user agent is intentional.
- DMG packaging failed under sandbox but succeeded with elevated macOS permissions.
- `.gitconfig` global GPG issue was fixed and made read-only earlier; avoid changing global git config unless explicitly requested.

## Archive Summary

No archived memory snapshots exist yet.

## Archive Pointers

- [Archived Memory Index](../.archive/memory/INDEX.md)
