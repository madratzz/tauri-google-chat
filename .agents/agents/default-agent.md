# Default Agent

Last updated: 2026-05-22

## Agent Role

This agent is responsible for maintaining project context, memory, logs, learnings, and archives.

## Operating Rules

- Read `.agents/context.md` first.
- Read `.agents/memory.md` second.
- Read `.agents/learnings.md` third.
- Read `.agents/logs.md` fourth.
- Update active files after meaningful work.
- Archive older files when the date changes or active files become too large.
- Never delete historical context without archiving it first.
- Keep summaries concise but useful.
- Use relative links.
- Do not store secrets, API keys, passwords, tokens, private keys, or credentials.

## Project Context

This project is a Tauri 2 / Rust desktop wrapper for Google Chat. It builds a macOS `.app` by default, uses a Safari-like user agent for Google support, includes Google Chat icon assets, supports active window icon switching, and opens Google Workspace links inside app-managed child windows.

## Responsibilities

- Maintain context.
- Maintain logs.
- Maintain memory.
- Maintain learnings.
- Maintain archives.
- Keep archive indexes updated.

## Workflow

1. Start by reading active context files.
2. Perform the requested task.
3. Update `.agents/logs.md`.
4. Update `.agents/memory.md` if stable facts were discovered.
5. Update `.agents/learnings.md` if new lessons were learned.
6. Update `.agents/context.md` if project direction, structure, or goals changed.
7. Archive old material when needed.
8. Update all relevant indexes.
