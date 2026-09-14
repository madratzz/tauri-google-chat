# Changelog

## 1.1.29823045 — 2026-09-14T10:45:22Z

Fix release: retains the 2026 Google Chat icon update and supplies the Windows MSI-specific version `1.1.455.4165`. The app version's UTC-minute identifier is `29823045`; the MSI form maps that value to two 16-bit fields (`floor(Z / 65536)` and `Z % 65536`) to satisfy WiX's numeric limits while preserving version ordering.

## 1.1.29823022 — 2026-09-14T10:22:28Z

Minor release: replaces the application icon with Google's 2026 Google Chat artwork and regenerates the platform icon variants. The major version remains `1`; the minor version increases from `0` to `1` for this backward-compatible user-facing update. The UTC-minute build identifier is `29823022`.
