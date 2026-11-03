# Implementation Roadmap

The roadmap covers the complete requested scope. Milestones are ordered to produce a usable build early without abandoning the advanced features.

## M0 — Repository and engineering foundation

- [x] Create the launcher repository.
- [x] Record the hybrid architecture and feature roadmap.
- [ ] Bootstrap the Tauri 2 + React + TypeScript + Rust application.
- [ ] Add formatting, linting, unit tests, and CI.
- [ ] Define application data paths, config schema, and migration rules.
- [ ] Add legal notices and third-party component inventory.

**Exit criteria:** clean checkout builds on macOS, Windows, and Linux CI; the app opens; version and diagnostic information are visible.

## M1 — First playable launcher

- [ ] Build the Resort-themed home screen and navigation shell.
- [ ] Add first-run setup for Dolphin and the game image.
- [ ] Validate the selected game image and identify its region/title from metadata.
- [ ] Detect Dolphin version and validate executable launch.
- [ ] Launch the game using a stable app-managed Dolphin user directory.
- [ ] Display launch errors, emulator exit status, and logs.
- [ ] Preserve user data when the app updates.

**Exit criteria:** a user can configure an existing Dolphin installation and their own Wii Sports Resort image, press Play, and reach gameplay. Saves remain after closing/reopening and after installing a newer launcher build.

## M2 — Profiles and Dolphin configuration bridge

- [ ] Create, rename, duplicate, export, and import profiles.
- [ ] Persist settings in a versioned schema.
- [ ] Read/write supported per-game Dolphin configuration values.
- [ ] Detect unsupported settings by Dolphin version.
- [ ] Add safe backup/restore of managed configuration.
- [ ] Add a Dolphin fork compatibility/version capability check.

**Exit criteria:** two profiles can use different graphics, controllers, and mods without corrupting each other's configuration or save data.

## M3 — Graphics and display

- [ ] Add Native, 720p, 1080p, 1440p, 4K, 5K, and 8K output presets.
- [ ] Separate internal rendering scale from window/display resolution.
- [ ] Expose renderer, anti-aliasing, anisotropic filtering, texture filtering, and shader options where supported.
- [ ] Add optional Resort compatibility fixes after validating each one.
- [ ] Add fullscreen, borderless, and windowed display modes.
- [ ] Show estimated performance impact and warn about VRAM requirements.

**Exit criteria:** settings persist per profile and are verified in Dolphin logs/configuration; unsupported modes are disabled or clearly marked. 8K is a target preset, not a performance guarantee.

## M4 — Mod manager

- [ ] Import, inspect, enable, disable, and remove Riivolution patch sets.
- [ ] Parse Riivolution XML options and display supported choices.
- [ ] Verify how the current Dolphin fork activates a selected patch set.
- [ ] Implement engine integration only if the existing Dolphin interface cannot select patches reliably.
- [ ] Import Dolphin Resource Pack v1/v2 archives.
- [ ] Manage custom texture folders and resource-pack priority.
- [ ] Manage Gecko/Action Replay codes where Dolphin exposes them.
- [ ] Add conflict preview, per-profile activation, safe rollback, and mod diagnostics.

**Exit criteria:** a mod set can be enabled for one profile, disabled for another, and reverted without modifying the original game image. Invalid or unsafe archives are rejected.

## M5 — Controllers and motion input

- [ ] Expose real Wii Remote connection and passthrough setup.
- [ ] Configure Wii Remote + MotionPlus and Nunchuk extensions.
- [ ] Add virtual Wii Remote profiles for gamepads.
- [ ] Add keyboard/mouse remapping.
- [ ] Separate motion, tilt, swing, shake, pointer/IR, and button mapping.
- [ ] Add calibration and live input visualization.
- [ ] Add multiple controller slots and local multiplayer profiles.

**Exit criteria:** tests cover real Wii Remote passthrough and virtual inputs; each input profile can be restored and changed without editing raw Dolphin configuration files.

## M6 — Saves, capture, and gameplay utilities

- [ ] Save backup/restore with timestamps.
- [ ] Automatic backup before mod or profile changes that affect persistent data.
- [ ] Save-state shortcuts and management when supported by the active Dolphin build.
- [ ] Screenshot and video-recording controls.
- [ ] Frame pacing, speed limit, and slow-motion controls where supported.
- [ ] Disk-space and save-data visibility.

**Exit criteria:** backup/restore is tested against corrupted or missing data; app updates do not alter saves or states.

## M7 — Resort identity and polish

- [ ] Original Wuhu Island-inspired launcher artwork and icon set.
- [ ] Sport-specific panels for all 12 sports.
- [ ] Animated UI transitions and optional ambient audio.
- [ ] Keyboard navigation, accessibility labels, scalable UI, and high-contrast mode.
- [ ] Settings search and controller shortcut hints.
- [ ] First-run guide and support/diagnostics page.

**Exit criteria:** UI remains readable at common window sizes and display scaling; no copyrighted game art/audio is bundled without permission.

## M8 — Distribution and release automation

- [ ] macOS .app inside a .dmg.
- [ ] Windows installer/portable .exe.
- [ ] Linux AppImage.
- [ ] CI build and test on every pull request.
- [ ] Release workflow publishes GitHub Releases after a tagged, verified build.
- [ ] Runtime packaging, version pinning, license notices, and corresponding source are handled correctly.
- [ ] Upgrade test confirms user data survives.
- [ ] Clean-machine install and launch tests on each platform.

**Exit criteria:** release assets install and launch on clean test machines, and update/uninstall procedures preserve user data unless the user explicitly requests removal.

## Non-goals

- Reimplementing the Wii hardware or recompiling Nintendo's game executable.
- Bundling the Wii Sports Resort game image, firmware, NAND, or copyrighted game assets.
- Promising native 8K rendering performance on hardware that cannot sustain it.
- Treating every Riivolution patch as automatically compatible with Dolphin or every game region.
