# Release and Update Policy

## Targets

- macOS: a functional .app packaged inside a .dmg.
- Windows: a Windows installer or portable .exe distribution.
- Linux: AppImage.

## CI and release flow

- Pull requests run formatting, lint, type-check, and platform build checks.
- Pushes to the default branch run continuous integration.
- Release builds are created from a version tag only after required checks pass.
- GitHub Releases are the canonical distribution location; transient workflow artifacts are for testing, not the only release channel.
- CI must avoid overlapping release jobs and cancel superseded non-release builds safely.

## Runtime handling

Dolphin can be user-provided or included in a release only when licensing and corresponding-source requirements are met. The release process must record the exact Dolphin commit/build version and include required license texts and source access.

Do not auto-update the Dolphin engine independently of the app's compatibility matrix without validation.

## User-data preservation

- App executable and mutable user data are separate.
- Updates never reset saves, NAND, mods, texture packs, controller profiles, or user settings.
- Database/config schema migrations are versioned and tested.
- Uninstall behavior must not delete user data by default.
- Rollback instructions are published for releases with settings or data migrations.
