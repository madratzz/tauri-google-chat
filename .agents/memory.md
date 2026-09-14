# Active Memory

Last updated: 2026-09-14

## Stable Project Facts

- The app is a Tauri 2 / Rust desktop wrapper for Google Chat.
- Main product name: `Google Chat Desktop`.
- Rust crate name: `google-chat-desktop`.
- Bundle identifier: `com.madratzz.google-chat-desktop`.
- The primary implementation is `src-tauri/src/lib.rs`; bundle configuration is `src-tauri/tauri.conf.json`.
- The repository has `main` and `development`; implementation begins from `development` and uses feature branches under the policy in `AGENTS.md`.
- The repo uses local `.npmrc` with `registry=https://registry.npmjs.org/`.

## User Preferences

- User prefers the official npm registry for this project.
- User asked for persistent AI-agent context, logs, memory, learnings, and archives.
- User authorized creation of `development` and execution of this context-system refresh.

## Naming Conventions

- Archive filenames use `YYYY-MM-DD` plus an optional lowercase hyphenated slug.
- Active agent files live under `.agents/`.
- Older context material lives under `.archive/`.
- Feature branches use `feature/<short-kebab-case-description>`.

## Important Entities

- Google Chat URL: `https://chat.google.com/`.
- Default agent profile: `.agents/agents/default-agent.md`.
- Canonical repository instructions: `AGENTS.md`.

## Do Not Forget

- Do not copy secrets or credentials into `.agents/` or `.archive/`.
- macOS `.app` icon requires `icon.icns` and `CFBundleIconFile`; PNG alone is not enough.
- Google Chat rejected the default embedded browser, so the Safari-like user agent is intentional.
- Preserve logical commits and use pull requests; do not force-push, squash merge, or alter Git configuration without explicit user direction.
- Context archives are immutable once verified.

## Archive Summary

The historical May 2026 memory snapshot was archived during the September 2026 refresh. It includes redacted legacy local-path information.

## Archive Pointers

- [Archived Memory Index](../.archive/memory/INDEX.md)
- [2026-05-22 initial project history](../.archive/memory/memory-2026-05-22-initial-project-history.md)
