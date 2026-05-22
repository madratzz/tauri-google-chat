# Active Logs

Last updated: 2026-05-22

## Current Session

### 2026-05-22 14:41

Hardened Google Workspace popup handling after the user reported Marketplace app install flows crashing or doing nothing.

Files touched:

- `src-tauri/src/lib.rs`
- `.agents/learnings.md`
- `.agents/logs.md`

Decisions made:

- Added a global atomic counter to make child popup window labels unique.
- Replaced popup window `.expect(...)` panics with graceful `NewWindowResponse::Deny` on build failure.
- Kept popup windows inside the app through Tauri-managed child windows.

Issues found:

- URL-derived child window labels could collide for repeated Marketplace/OAuth/install popups.
- A label collision or build failure inside `.expect(...)` could crash the app.

Next steps:

- Manually retest Google Workspace Marketplace `Install` and `Add to Space` flows.
- Consider a full in-app overlay browser shell with toolbar controls if the user wants links shown inside the main window instead of child windows.

### 2026-05-22 14:34

Fixed Google Chat opening Google Tasks, Drive, Contacts, and similar side-panel services in separate blank windows on its own.

Files touched:

- `src-tauri/src/lib.rs`
- `.agents/context.md`
- `.agents/learnings.md`
- `.agents/logs.md`

Decisions made:

- Removed broad `on_navigation` interception for Workspace hosts.
- Kept internal app windows only for explicit `window.open` popup requests.
- Changed popup child windows to use `about:blank` with `NewWindowResponse::Create`, matching Tauri's recommended new-window flow.

Issues found:

- Google Chat embeds side-panel/plugin services such as Tasks through navigations that are not user-requested external link opens.
- Intercepting those navigations caused unwanted app windows and could leave embedded panels stuck loading.

Next steps:

- Manually verify clicking real Docs/Sheets/Gmail links opens an in-app child window, while side-panel Tasks/Contacts/Drive remain embedded.

### 2026-05-22 14:25

Set up the agent context system requested by the user.

Files touched:

- `.agents/README.md`
- `.agents/INDEX.md`
- `.agents/context.md`
- `.agents/memory.md`
- `.agents/learnings.md`
- `.agents/logs.md`
- `.agents/agents/default-agent.md`
- `.archive/README.md`
- `.archive/INDEX.md`
- `.archive/logs/INDEX.md`
- `.archive/memory/INDEX.md`
- `.archive/learnings/INDEX.md`
- `.archive/context/INDEX.md`
- `.archive/agents/INDEX.md`

Decisions made:

- Created active context files with concrete project details instead of placeholder-only content.
- Created empty archive indexes because no dated archive files exist yet.
- Avoided storing secrets or credentials.

Issues found:

- Initial directory creation needed elevated filesystem permission for dot-directories in this environment.

Next steps:

- Commit the agent-context setup if the user wants it versioned.
- Update `.agents/logs.md` after future meaningful work.

## Recent Previous Sessions

### 2026-05-22

Project was bootstrapped as a Tauri/Rust Google Chat desktop app. Work included npm registry override, Rust installation, app build/test setup, native Edit menu for paste support, Safari-like user agent for Google Chat, Google Chat icon assets, internal Workspace link windows, runtime icon variant menu, git initialization, and several commits.

## Archive Summary

No archived logs exist yet.

## Archive Pointers

- [Archived Logs Index](../.archive/logs/INDEX.md)
