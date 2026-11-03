# Wii Sports Resort Enhanced

A game-specific desktop launcher and enhancement layer for **Wii Sports Resort**, powered by a user-selected Dolphin installation.

> **Status:** first implementation committed. Native builds and real-game testing are still pending. This is an emulator wrapper, not a recompilation of Nintendo's game executable.

## Implemented in the first pass

- Tauri 2 + React + TypeScript + Rust desktop application.
- Wuhu Island-inspired launcher dashboard and navigation.
- Dolphin executable detection through `--version`.
- ISO, RVZ, WBFS, WIA, GCZ, GCM and CISO file selection.
- Wii Sports Resort RZT-family ID detection for raw ISO/GCM headers.
- Direct launch through Dolphin's `--user`, `--exec`, and `--config` arguments.
- Separate, persistent Dolphin user folder per profile.
- Profile creation, switching, renaming, deletion (user data retained).
- ZIP import for Riivolution patch folders and Dolphin Resource Packs.
- Safe archive extraction with path, symlink, entry-count and expanded-size checks.
- Enable/disable/remove for app-managed mod folders and resource-pack ZIPs.
- Gecko code writer for Dolphin per-game GameSettings files.
- Per-game real/emulated Wii Remote source selection.
- Internal scale, MSAA, anisotropy, linear filtering, vertex rounding, FPS overlay, and borderless fullscreen preferences.
- Save/state/screenshot backup and restore.
- Cross-platform CI and tagged GitHub Release workflows.

## Important limits

- The initial source has not yet been compiled by the GitHub Actions matrix. Treat it as an implementation awaiting CI verification.
- For Riivolution, the app imports and installs patches but Dolphin's own game-list workflow is still needed to choose **Start with Riivolution Patches** and select the XML/options. The ordinary Play button launches the unpatched game.
- Real and emulated Wii Remote source is selected through the per-game GameINI. Detailed gyro, IR, swing, Nunchuk and MotionPlus extension mappings remain configured in Dolphin's controller UI.
- The MotionPlus/Nunchuk toggles are saved as profile preferences; they do not yet create controller mappings or force an attachment profile.
- Game ID detection currently reads raw ISO/GCM headers. For compressed formats, confirm the six-character ID in Dolphin and enter it in Settings.
- 12× is Dolphin's internal rendering scale ceiling in the current engine. It is not a guarantee of native 8K output; final display resolution and performance depend on Dolphin, the OS, GPU and monitor.
- The Dolphin game window is separate from the launcher.

## Requirements

- A compatible Dolphin installation selected by the user.
- A legally obtained Wii Sports Resort image in a format supported by Dolphin.
- A MotionPlus-compatible controller for original motion behavior, or appropriately configured emulated input.

The game image, Wii NAND, firmware, and proprietary Nintendo artwork/audio are not included.

## Development

Install Node.js 22, Rust stable (1.77+; current Tauri toolchain recommended), and the [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/).

```sh
npm install
npm run tauri -- dev
```

Frontend checks:

```sh
npm run check
npm run build
```

Desktop build:

```sh
npm run tauri -- build
```

See [the roadmap](docs/ROADMAP.md), [architecture](docs/ARCHITECTURE.md), [Dolphin integration](docs/EMULATOR-INTEGRATION.md), and [mod-system details](docs/MOD-SYSTEM.md).

## Dolphin integration

The emulator engine is maintained separately at [NVDEMU/dolphin](https://github.com/NVDEMU/dolphin). The launcher uses its documented CLI and configuration interfaces first, and only requires engine changes when a concrete capability is missing.

## Licensing

Dolphin is GPL-2.0-or-later. This repository's launcher license and third-party dependency notices still need a maintainer decision before redistribution. Any future package containing Dolphin must include the required license notices and corresponding source access. No Nintendo game files or proprietary assets are redistributed.
