# Architecture

## Product shape

Wii Sports Resort Enhanced is a game-specific desktop launcher, not a recompilation of the Wii Sports Resort executable. Dolphin remains responsible for Wii hardware emulation and game execution.

## Stack

- Tauri 2 desktop shell.
- React + TypeScript launcher interface.
- Rust backend for file validation, profile persistence, process lifecycle, safe archive extraction, Gecko GameINI editing and backups.
- `NVDEMU/dolphin` as a separately maintained external emulator runtime.

## Process integration

The launcher starts Dolphin as a child process with discrete OS arguments, never a shell command string. The first implementation uses:
- `--version` for runtime detection.
- `--user` for a stable per-profile Dolphin user folder.
- `--exec` for the selected game image.
- `--config=` for validated per-run graphics and cheat settings.

The game runs in a separate Dolphin window. Window embedding is deferred until a stable cross-platform render-window contract exists.

## User data

Mutable data lives in the OS application data directory, outside the app install:
- `config.json`: launcher configuration and profiles.
- `DolphinProfiles/<profile UUID>/User`: Dolphin user folder with title saves, states, cheats, GameSettings, textures, resource packs and Riivolution files.
- `Backups/<profile UUID>/<backup ID>`: save/state/screenshot backups.
- `ModStaging`: temporary import staging.

Removing a profile does not delete its Dolphin user folder. The backup feature covers Dolphin `Wii/title`, `StateSaves` and `Screenshots` directories.

## Mod subsystem

Riivolution archives are staged, path-validated, XML-parsed and copied into `User/Load/Riivolution/<generated UUID>`. Resource packs are validated from their manifest and their original ZIP archive is stored as `User/ResourcePacks/<generated UUID>.zip`. Toggle/disable moves only app-managed generated paths into `User/ModStash`.

Dolphin's normal Riivolution workflow is still required to select the XML/preset. Importing a patch does not activate it. The normal Play button uses `--exec` and launches the unpatched title.

Dolphin Resource Packs are enabled by their presence in the ResourcePacks folder. When a profile contains an enabled pack, the launcher turns on Dolphin's high-resolution texture loading and cache settings.

## Graphics

Per-run configuration includes internal resolution, maximum internal resolution, MSAA, anisotropy, forced linear texture filtering, vertex rounding, FPS metrics and borderless fullscreen. The anisotropy setting is mapped from the UI's 1×/2×/4×/8×/16× choices into Dolphin's `AnisotropicFilteringMode` enum.

12× is the current Dolphin internal-resolution ceiling. It is not a guarantee of native 8K output. The final output is controlled by the Dolphin render window, OS, graphics backend, GPU and monitor.

## Controllers

The launcher writes `WiimoteSource0` into the selected game's `GameSettings/<GAMEID>.ini` under `[Controls]`, selecting 1 (Emulated) or 2 (Real). This is a per-game override and avoids changing all Dolphin titles.

MotionPlus/Nunchuk preferences and device notes are persisted per launcher profile, but automatic attachment selection and detailed bindings are not implemented yet. Gyro, IR, swing, rumble and extension mapping remain configured in Dolphin's controller UI.

## Security and reliability

- Validate all user-selected paths.
- Use process argument vectors, never shell command strings.
- Reject ZIP path traversal, symlinks, excessive entry counts and oversized expanded archives.
- Never execute scripts from mod archives.
- Use generated UUIDs for app-owned mod paths.
- Remove only files/directories the app can identify as its own.
- Use temporary files plus rename for configuration/GameINI changes.
- Back up current data before restore.
- Keep all saves and user profiles outside the install directory.
