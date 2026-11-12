# Active Logs

Last updated: 2026-11-09

## Current Session

### 2026-11-12T08:37:01Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Release 1.3.29907877: mic still unavailable after 1.3.29907752. Inspected the released `.app`: Info.plist usage strings present, but the binary's CodeDirectory flags are `0x10002` (hardened runtime + ad-hoc) with no entitlements blob. Added `src-tauri/Entitlements.plist` (audio-input, camera) wired via `bundle.macOS.entitlements`.

### 2026-11-12T06:32:44Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Release 1.3.29907752: user reported the Chat recorder spinning. wry grants WKWebView media-capture requests when no permission handler is set, so the block was macOS TCC: added `src-tauri/Info.plist` with camera and microphone usage strings.

### 2026-11-10T12:00:43Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Release 1.3.29905200: user reported Download in the file viewer hangs. wry cancels downloads when no handler is registered (`wkwebview/navigation.rs`). Added `on_download` on all webviews: save to `download_dir()` via `util::download_file_name` + `util::unique_path` (tested), notify on finish.

### 2026-11-10T11:18:44Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Release 1.3.29905158: user reported drag-and-drop file attachments not working. Added `disable_drag_drop_handler()` to all webview builders so HTML5 drops reach Google Chat.

### 2026-11-10T07:43:45Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Release 1.3.29904943: user confirmed sign-in works on macOS with 1.2.29904922 and asked why the title bar is white (macOS light appearance vs. Chat dark theme). Added a persisted View → Title Bar setting (system/light/dark) applied with `Window::set_theme`.

### 2026-11-10T07:22:32Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Release 1.2.29904922: user's `[gcd]` log on an M1 Mac showed signed-out `chat.google.com` lands on `workspace.google.com/products/chat/` and clicking Sign in emits nothing. Main-window `on_page_load` now redirects that landing page to `accounts.google.com/ServiceLogin?service=dynamite&continue=https://chat.google.com/` (rate-limited to once per minute).

### 2026-11-09T22:13:05Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Release 1.2.29904373: user reported sign-in still does nothing on an M1 Mac after 1.2.29904303. Sign-in URLs from `on_new_window` now navigate the main window; peek creation is deferred out of the callback; added stderr logging and the Tauri `devtools` feature for diagnosis. Root cause not yet confirmed on hardware.

### 2026-11-09T21:45:00Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Reworked Arch packaging in `release.yml` after repeated failures (container job not starting, tag-only upload steps, PKGBUILD needing a published tag tarball, `arch-release.yml` 403 on published releases). The Linux job now repackages its `.deb` into `.pkg.tar.zst` plus a binary PKGBUILD via `.github/scripts/arch-package.sh` and uploads by release id; a final `publish-release` job publishes the draft when all platforms succeed. Removed `arch-release.yml`. Script tested locally with a synthetic .deb.

### 2026-11-09T21:03:24Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Release 1.2.29904303: fixed macOS sign-in. wry's WKWebView navigation handler fires for all frames and `javascript:` URLs (no main-frame filter, see wry `wkwebview/navigation.rs`), so the 1.2 `on_navigation` allowlist cancelled Google sign-in. Removed `on_navigation` filtering from all webviews (peek keeps only its sentinel handling); link policy now lives in `on_new_window`. Added Google country-domain matching with tests.

### 2026-11-09T19:15:00Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

README: added the v1.2.29902849 release summary with direct download links per platform, and a Releasing section covering tag-triggered and manual (`arch-release.yml`) flows.

### 2026-11-09T18:35:00Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Fixed `release.yml`: the draft-release step passed the GraphQL node id (`gh release view --json id`) as `releaseId`, so once `tagName` was removed in 1.2 `tauri-action` skipped every upload and v1.2.29902849 was published with no assets. The step now looks up the numeric REST id and reuses an existing release, so the workflow can be re-run to attach assets.

### 2026-11-09T12:30:00Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Added `.github/workflows/arch-release.yml`, a manual workflow that builds the Arch package from a ref, publishes the matching draft release (creating its tag), and uploads the `.pkg.tar.zst` plus a checksummed PKGBUILD. Needed because v1.2.29902849 was released via `workflow_dispatch` (tag pushes are blocked from the agent environment), which skips the tag-only Arch upload steps in `release.yml`. Documentation review: README release steps unchanged; this log is the record.

### 2026-11-08T20:49:24Z — claude/session_01CWCdnFCDU7XtU1qsJ45txs

Implemented the review/improvement plan (speed, security, responsive UI, UX, CI) as version `1.2.29902849`. Work was done on the session-designated branch `claude/jolly-dijkstra-mea4sv` because no `development` branch exists.

Files touched: `src-tauri/src/{lib,util,settings}.rs`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/tauri.conf.json`, `package.json`, `package-lock.json`, `PKGBUILD`, `.github/workflows/{ci,release}.yml`, `README.md`, `CHANGELOG.md`, `.agents/context.md`, `.agents/logs.md`.

Verification: `cargo fmt --check`, `cargo clippy --all-targets -D warnings`, `cargo test` (5 tests), and `cargo build --release` on Linux. Not run on macOS/Windows or against a live Google account.

### 2026-09-14T15:45:22+05:00 — codex/root

Corrected the Windows release packaging version after the initial icon-release workflow failed during MSI bundling. WiX rejects an application patch number greater than 65,535; this project uses a deterministic WiX-specific version while retaining the required UTC-minute app version.

Files touched:

- `AGENTS.md`
- `CHANGELOG.md`
- `package.json`
- `package-lock.json`
- `PKGBUILD`
- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`
- `src-tauri/tauri.conf.json`
- `.agents/context.md`
- `.agents/learnings.md`
- `.agents/logs.md`

Decisions made:

- Assigned unique app version `1.1.29823045` at `2026-09-14T10:45:22Z`.
- Configured WiX version `1.1.455.4165`, mapping the UTC-minute identifier into two valid 16-bit fields.
- Kept failed tag `v1.1.29823022` unpublished; the corrective release uses a new version rather than rewriting history.

Next steps:

- Merge the Windows packaging fix through the required release path, build all artifacts, publish the corrective release, and install it on macOS.

### 2026-09-14T15:08:40+05:00 — codex/root

Refreshed the repository agent-context system and added canonical repository instructions. Created `development` from `main`, then created the active feature branch as required by the newly documented workflow.

Files touched:

- `AGENTS.md`
- `CLAUDE.md`
- `.agents/`
- `.archive/`

Decisions made:

- Archived the dated May 2026 active context into immutable category snapshots before condensing active files.
- Redacted the historical local filesystem path instead of copying it into a new archive.
- Kept the project README unchanged after review because the context-system setup is documented by the new repository instructions and active log.

Issues found:

- The required `development` branch was absent; it was created from the current `main` with explicit user authorization.

Next steps:

- Review and merge this feature into `development` through a non-squash pull request.

## Recent Previous Sessions

### 2026-05-22

The original project bootstrap, Tauri navigation work, in-window peek overlay, and macOS icon work were preserved in the dated log archive.

## Archive Summary

Detailed May 2026 work logs were archived during the September 2026 refresh.

## Archive Pointers

- [Archived Logs Index](../.archive/logs/INDEX.md)
- [2026-05-22 initial project history](../.archive/logs/logs-2026-05-22-initial-project-history.md)
