# Wii Sports Resort Enhanced

A game-specific launcher and enhancement layer for **Wii Sports Resort**, powered by a customized Dolphin runtime.

> **Development status:** architecture and repository bootstrap. This project is not yet a playable launcher, and planned features below should not be interpreted as implemented.

## Goals

- A dedicated, Wuhu Island-inspired desktop launcher.
- A hybrid integration model: the launcher owns configuration and game management while Dolphin remains the emulation engine.
- Real Wii Remotes and MotionPlus, plus configurable virtual Wii Remotes driven by gamepads and keyboard/mouse.
- Mod profiles and support for Riivolution patches, Dolphin resource packs/custom textures, and Gecko codes.
- High-resolution rendering presets through 8K output targets, subject to hardware and emulator limits.
- Persistent saves, backups, screenshots, recording, and per-profile settings.
- Windows, macOS, and Linux releases.

## Architecture

- **Frontend:** Tauri 2 + React + TypeScript.
- **Desktop/backend layer:** Rust. It validates paths, manages configuration and profiles, and launches Dolphin as a child process without passing commands through a shell.
- **Emulation:** [NVDEMU/dolphin](https://github.com/NVDEMU/dolphin), maintained separately from this repository.
- **User data:** kept outside the install directory so application updates do not overwrite saves, mods, or settings.

The initial integration uses Dolphin's documented command-line interface and a dedicated user directory. Deeper integration will be added only where the existing CLI/configuration interfaces are insufficient.

## Repository layout

- src/ — React application
- src-tauri/ — Rust desktop application and native integration
- docs/ — Architecture, integration notes, and roadmap
- assets/ — Original launcher artwork and UI resources
- .github/ — CI, issue templates, and release automation

## Development status

| Area | Status |
|---|---|
| Product architecture and roadmap | In progress |
| Launcher UI | Planned |
| Dolphin detection and launch | Planned |
| Per-game/per-profile configuration | Planned |
| Riivolution manager | Planned |
| Texture/resource-pack manager | Planned |
| Wii Remote and virtual motion configuration | Planned |
| Graphics presets through 8K | Planned |
| Save management and backups | Planned |
| Cross-platform packaging | Planned |

See [the implementation roadmap](docs/ROADMAP.md) and [architecture](docs/ARCHITECTURE.md).

## Requirements

- A legally obtained Wii Sports Resort disc image or supported game dump. The game is not included.
- A compatible Dolphin build, either selected by the user or provided through a properly licensed distribution package.
- A desktop operating system supported by both Tauri and the selected Dolphin build.

## First-run target

1. Open the launcher.
2. Select the Dolphin executable or application bundle.
3. Select the user's Wii Sports Resort game image.
4. Choose or create a profile.
5. Launch through Dolphin with the app-managed user directory.
6. Preserve saves, settings, and mods independently of application updates.

## Build targets

- macOS: target is a .dmg containing a functional .app.
- Windows: installer or portable .exe distribution.
- Linux: AppImage.

## Licensing and third-party software

Dolphin is licensed under GPL-2.0-or-later. Any distribution containing Dolphin must comply with Dolphin's license, including required source-code and license notices. The launcher and its relationship to bundled third-party components will be documented before releases. No Nintendo game files, firmware, or proprietary game assets will be redistributed.

## Contributing

Read CONTRIBUTING.md before submitting changes. Do not commit game dumps, NAND images, BIOS/firmware, proprietary game assets, copyrighted texture packs, personal save files, or controller credentials.
