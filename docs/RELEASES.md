# Release and Update Policy

## Targets

- macOS: functional .app packaged inside a .dmg.
- Windows: Tauri NSIS/MSI installer assets.
- Linux: AppImage (plus any other Tauri targets produced by the release workflow).

## CI

Pull requests and main-branch pushes run frontend checks, Rust tests, and a native Tauri build on macOS, Windows and Ubuntu. Tagged `v*` builds run through the release workflow and publish GitHub Release assets.

CI must be green before publishing a version. The launcher is not yet considered release-ready until the first successful three-platform workflow and clean-install tests complete.

## Runtime

Dolphin is selected by the user and is not bundled in the first implementation. If a future release bundles Dolphin, it must pin the exact source revision and satisfy GPL-2.0-or-later source and notice obligations.

## User data

Mutable data is stored in the OS application-data directory, never inside the application bundle/install directory. Updating or uninstalling the app must not delete Dolphin profile folders, saves, mods, texture packs, controller profiles or backups by default.
