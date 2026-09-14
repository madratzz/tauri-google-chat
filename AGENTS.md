# Repository Agent Instructions

This is the authoritative instruction file for AI agents working in this repository. Follow explicit user instructions first, then this file, then any applicable local documentation.

## Project

Google Chat Desktop is a Rust/Tauri 2 wrapper for Google Chat. The main implementation is in `src-tauri/src/lib.rs`; package and bundle metadata live in `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json`.

## Version Control and Pull Requests

- Start implementation work from `development`.
- Create one branch per task as `feature/<short-kebab-case-description>`.
- Make small, cohesive commits that include the relevant documentation and agent-context updates.
- Open a pull request for each merge; merge feature branches into `development` only through that pull request.
- Update `main` only through a release pull request from `development`.
- Preserve logical commits: do not squash merge, rewrite shared history, force-push, change Git configuration, or commit unrelated changes without explicit user direction.
- If `development` is absent or this policy conflicts with enforced repository rules, stop and report the conflict before creating a feature branch or pull request.

## Version Naming

Use `X.Y.Z` versions. Increase `X` for intentional breaking releases and `Y` for backward-compatible feature releases. Set `Z` to `floor(unix_timestamp_seconds / 60)` in UTC; never reuse or manually decrease it. Record the full version, UTC assignment time, and major/minor rationale in relevant release notes or version history.

## Documentation

Every completed task requires a documentation update in the same logical commit. Update the affected README, architecture/usage documentation, changelog, release notes, or agent context. If no project-facing documentation needs substantive change, record a precise documentation-review note in `.agents/logs.md` instead of making cosmetic edits.

## Code and Naming

- Prefer source and configuration changes over generated output.
- Use the existing Rust/Tauri conventions and run proportionate verification (`npm test`, `cargo check`, or `npm run build`) after relevant changes.
- For any Unity C# added in the future, private fields marked `[SerializeField]` use PascalCase, such as `[SerializeField] private float MovementSpeed;`.
- Do not route Google Chat links to the system browser unless the user specifically changes that product behavior.

## Agent Context and Archives

- Read `.agents/README.md`, `.agents/INDEX.md`, and the relevant active files before meaningful work; read `context.md` first.
- Keep current, concise context in `.agents/`; preserve dated historical material in `.archive/`.
- Update `.agents/logs.md` after meaningful work in reverse chronological order, with an ISO 8601 timestamp and agent/session identifier where appropriate.
- Update memory, learnings, and context only for durable, relevant information.
- When a date changes or active material becomes too large, create and verify a unique immutable archive snapshot before trimming active material. Retain a concise active summary and archive pointer.
- Maintain archive indexes in recent-to-oldest order. Never overwrite an existing archive; use `-02`, `-03`, and so on for distinct collisions.
- Before editing shared context or index files, re-read them and merge safely. If a safe merge is not possible, leave the file unchanged and report the conflict.

## Sensitive Data

Never put secrets, API keys, tokens, passwords, private keys, credentials, personally identifiable information, customer content, internal-only URLs/hosts, or unnecessary account numbers in repository context or archives. Redact with clear placeholders such as `<SECRET>`, `<TOKEN>`, `<EMAIL>`, or `<LOCAL_PROJECT_PATH>`.

## Context Maintenance Checklist

- [ ] Updated `.agents/logs.md` for meaningful work.
- [ ] Updated relevant active context, memory, or learnings.
- [ ] Archived old content when required and verified archive links/indexes.
- [ ] Checked new context for sensitive data.
- [ ] Updated affected documentation or recorded the documentation-review note.
