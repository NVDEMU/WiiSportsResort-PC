# Dolphin Integration

## Current approach

Dolphin remains a separate emulator process. The launcher owns configuration, profiles, and launch orchestration. The initial integration deliberately uses Dolphin's documented command-line interface rather than linking the launcher to Dolphin internals.

Dolphin's documented CLI includes:
- --user / -u to select a user folder.
- --exec / -e to load a game image.
- --config / -C to override a configuration value.

Example launch shape (paths are illustrative):

    Dolphin --user "<managed-user-folder>" --exec "<user-game-image>"

The launcher must pass each argument separately through Rust's process API. Never build a shell command string.

## Executable discovery

Support:
- macOS: a selected Dolphin.app bundle or its executable inside Contents/MacOS.
- Windows: Dolphin.exe.
- Linux: Dolphin binary or supported AppImage entry point.

Do not silently prefer an arbitrary PATH binary when the user has selected a specific runtime. Show the resolved version and executable path in emulator settings.

## User-data layout

Keep stable application data separate from the launcher install:

    App data/
      config/
      profiles/
      emulator/
        User/
          Config/
          Wii/
          Load/
          ResourcePacks/
          StateSaves/
          Screenshots/
          Logs/
      backups/

The exact root directory is platform-specific and must be resolved through Tauri's app-data facilities. Do not place mutable saves or mods inside a macOS app bundle, Windows install directory, or AppImage mount.

## Process lifecycle

1. Validate the executable and game image.
2. Create/verify the managed user directory.
3. Resolve the selected profile into supported Dolphin settings.
4. Start Dolphin with a stable user directory and game path.
5. Capture process ID, stdout/stderr where practical, and exit status.
6. Provide graceful stop/restart handling; avoid force-killing except as a last resort.
7. Keep the launcher responsive while Dolphin is running.

The first release launches Dolphin in its normal game window. Full window unification is a later investigation.

## Configuration bridge

The Rust backend should expose typed commands rather than raw arbitrary configuration writes. Each setting has:
- Stable app-side ID.
- Dolphin configuration section/key.
- Type and allowed values.
- Minimum/maximum supported Dolphin version if relevant.
- Default and profile override.
- Whether restart is required.
- Validation and explanatory UI text.

A capability query will distinguish supported options from unsupported ones. Do not assume an option exists because it appears in another Dolphin release.

## Engine-side changes

The existence of the NVDEMU Dolphin fork does not mean launcher-specific hooks are already implemented. Before modifying the engine:
1. Inspect the fork's current branch and commits.
2. Verify existing CLI and configuration functionality.
3. Write a small reproduction/test for the missing capability.
4. Implement the smallest possible change in the engine fork.
5. Document the supported Dolphin version/contract here.
6. Add a launcher compatibility test.

Potential changes include selecting a named Riivolution patch-set from the command line, emitting structured startup diagnostics, or exposing a versioned integration capability. These are proposals, not claims about current functionality.
