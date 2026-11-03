# Architecture

## Product shape

Wii Sports Resort Enhanced is a game-specific desktop application. It is not a recompilation of the Wii Sports Resort executable. Dolphin remains responsible for emulating the Wii hardware and executing the original game image.

The product has two independently maintainable parts:

1. The launcher repository (this repository) owns the interface, user configuration, mod/profile management, and process lifecycle.
2. The Dolphin fork owns emulation and any engine-level changes that cannot be performed reliably through documented settings or command-line arguments.

## Technology

- Tauri 2 for the desktop shell and native window.
- React + TypeScript for the launcher UI.
- Rust for file-system operations, validation, configuration persistence, process management, and platform-specific integration.
- Dolphin as an external native process in the first playable milestone.

This keeps the launcher responsive and comparatively lightweight while avoiding a premature rewrite of Dolphin's Qt frontend.

## Major subsystems

### Application shell
- Startup and first-run setup.
- Navigation, theme, accessibility, and window state.
- User-visible error reporting and diagnostic export.

### Game library
- One primary Wii Sports Resort entry at first.
- Game image selection and validation.
- Read the disc header and title metadata rather than hard-coding a single region ID.
- Record the selected image path without copying the game image into the app install directory.

### Emulator manager
- Detect supported Dolphin installs and allow an explicit executable override.
- Read the Dolphin version and test whether the executable starts.
- Store per-profile Dolphin settings and a managed user-data path.
- Launch using argument arrays, never shell command concatenation.
- Capture exit status and logs; allow graceful shutdown before forced termination.

### Profiles and persistence
- Profiles reference a game image, Dolphin executable, graphics settings, controller layout, and a mod set.
- The app database/configuration lives in the OS application data directory.
- Emulator user data lives in a stable app-managed directory outside the install location.
- Updates must not delete or replace saves, NAND, screenshots, texture data, mods, or user settings.
- Profile import/export must use a versioned schema and migration path.

### Mod manager
- Separate adapters for Riivolution, Dolphin resource packs/custom textures, and Gecko/Action Replay codes.
- A common manifest model for app-managed metadata; preserve each underlying format rather than silently rewriting it.
- Conflict preview, enable/disable, ordering, backup, and validation.
- Do not download or redistribute copyrighted mods without permission.

### Controller manager
- Real Wii Remote discovery/connection remains a Dolphin responsibility.
- Expose Dolphin's emulated Wii Remote and extension configuration where possible.
- Provide gamepad and keyboard/mouse mappings to virtual Wii Remote inputs.
- Treat MotionPlus motion data and pointer/IR data as distinct capabilities.
- Show calibration and input diagnostics.

### Graphics manager
- Map friendly presets to verified Dolphin settings.
- Distinguish internal rendering scale, window/display resolution, and post-processing/upscaling.
- Do not claim that an 8K display output means native 8K game rendering.
- Keep risky compatibility options individually switchable and profile-scoped.

## Process boundary

Initial launch contract:

- Executable: a user-selected, validated Dolphin binary.
- User directory: a stable application-managed directory, supplied with Dolphin's user-directory argument.
- Game: the user's selected supported Wii game image, supplied with Dolphin's exec argument.
- Arguments are passed as discrete operating-system arguments.
- No shell, command prompt, or script interpreter is involved.

The launcher records the exact Dolphin version, arguments (with private paths redacted in exported diagnostics), profile ID, and process result.

The first release uses a separate Dolphin game window. A unified window or embedded render surface is a later engineering investigation, not a blocker for the first playable release.

## Engine integration strategy

Use public Dolphin command-line and configuration interfaces first. Add engine-side changes to NVDEMU/dolphin only when a concrete feature cannot be delivered safely through those interfaces.

Potential engine work, subject to source inspection and tests:

- A documented, versioned launcher integration contract.
- Reliable selection and activation of a named Riivolution patch set.
- Queryable runtime status and structured diagnostics.
- Game-specific graphics defaults and compatibility overrides.
- Controller profile import/export or a stable configuration bridge.

Do not maintain a long-lived patch set just to rename Dolphin or hide unrelated emulator features. Keep changes small, tested, and upstreamable where possible.

## Security and reliability

- Validate every path and file type at the Rust boundary.
- Use allow-listed executable launching and argument arrays.
- Protect archive extraction against path traversal, symlinks, oversized files, and decompression bombs.
- Never execute scripts supplied inside mod archives.
- Back up files before destructive mod changes.
- Use atomic writes for configuration and profile updates.
- Keep user data outside the application bundle.
- Provide clear errors when a feature is unsupported by the detected Dolphin build.

## Cross-platform policy

- macOS: support Apple Silicon first; detect the actual executable inside an app bundle.
- Windows: support x64 first; handle paths with spaces and non-ASCII characters.
- Linux: support common x64 distributions first; detect AppImage/system builds.
- Package Dolphin separately or as a properly licensed runtime payload. Its license and corresponding source obligations must be met.
