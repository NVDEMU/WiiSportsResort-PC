# Implementation Roadmap

This roadmap reflects the current first-pass implementation. A checked box means source code or workflow definition exists; it does not imply that the feature has passed a native build or real-game test.

## M0 — Application foundation

- [x] Tauri 2 + React + TypeScript + Rust scaffold.
- [x] Resort-inspired launcher UI with Home, Mods, Graphics, Controls, Saves, Settings pages.
- [x] Native file-picker and typed Rust command bridge.
- [x] JSON configuration and profile model.
- [x] CI and tagged release workflow definitions.
- [ ] CI matrix green on macOS, Windows and Linux.

## M1 — Playable launcher

- [x] Select Dolphin executable and query version using `--version`.
- [x] Select ISO, RVZ, WBFS, WIA, GCZ, GCM and CISO images.
- [x] Read RZT-family Wii Sports Resort IDs from raw ISO/GCM headers.
- [x] Launch using `--user`, `--exec`, and config overrides.
- [x] Isolate Dolphin user data per profile.
- [x] Poll child-process status.
- [ ] Test with real Wii Sports Resort images on all target OSes.
- [ ] Add process log collection and graceful shutdown controls.

## M2 — Profiles and persistence

- [x] Create, rename, switch and delete profiles.
- [x] Store graphics, controller preferences, mods and Gecko codes per profile.
- [x] Keep mutable data outside the app install directory.
- [x] Preserve profile user folders when profiles are removed.
- [x] Back up and restore managed Wii/title, StateSaves and Screenshots directories.
- [ ] Add profile import/export and schema migration tests.
- [ ] Add backup integrity checks and restore progress UI.

## M3 — Graphics

- [x] Expose 1×–12× internal-resolution settings.
- [x] Apply internal/max internal resolution, MSAA, anisotropy, linear texture filtering, vertex rounding, FPS overlay and borderless fullscreen through Dolphin configuration overrides.
- [x] Explain 8K output and hardware limitations.
- [ ] Validate 8K output modes on real monitors and GPUs.
- [ ] Add renderer selection, shader presets and verified Resort-specific profiles.
- [ ] Add frame pacing and performance testing.

## M4 — Mods

- [x] Import Riivolution ZIPs with safe extraction and XML validation.
- [x] Install, enable, disable and remove app-managed Riivolution folders.
- [x] Import Dolphin Resource Pack ZIPs and keep them as ZIP files in ResourcePacks.
- [x] Enable/disable/remove app-managed resource-pack archives.
- [x] Write Gecko codes into Dolphin GameSettings INI files.
- [ ] Add direct Riivolution patch-preset selection through a tested Dolphin engine interface. Until then, use Dolphin's game-list **Start with Riivolution Patches** workflow.
- [ ] Add Resource Pack v1/v2 conformance tests, conflict previews and priority controls.
- [ ] Add dependency graph, metadata editing and mod-profile export.

## M5 — Controllers

- [x] Apply real vs emulated Wii Remote source to per-game `WiimoteSource0`.
- [x] Persist MotionPlus/Nunchuk preferences and device notes.
- [ ] Build native mapping editor for gamepad, keyboard/mouse, gyro and IR.
- [ ] Apply MotionPlus and extension attachment profiles automatically.
- [ ] Add calibration view, live input diagnostics and multiple remote slots.

## M6 — Capture and utilities

- [x] Backup/restore utilities for Wii title saves, state saves and screenshots.
- [ ] Add screenshot and video capture controls.
- [ ] Add save-state management and hotkey customization.
- [ ] Add profile export/import and diagnostics export.

## M7 — Resort UI

- [x] Wuhu Island-inspired color system, island hero art built from CSS shapes, sports shortcut strip and Resort-themed dashboard.
- [ ] Add original sport artwork, transitions, sound themes, keyboard navigation and accessibility review.

## M8 — Distribution

- [x] macOS, Windows and Linux CI build definitions.
- [x] Tagged GitHub Release workflow definition.
- [ ] Verify successful macOS .app/.dmg, Windows installer/EXE and Linux AppImage assets.
- [ ] Add clean-install, upgrade and data-preservation tests.
- [ ] Decide launcher license and automate third-party notices/source bundle for any bundled runtime.

## Known limitations

- CI/native build has not yet been confirmed green.
- The launcher does not embed Dolphin's render window.
- Riivolution activation still requires Dolphin's own patch-selection UI.
- Virtual controller mapping, MotionPlus attachment and Nunchuk profiles are not yet implemented; Dolphin's controller UI handles those details.
- For compressed game images, users must confirm the six-character Game ID in Settings.
- No game image, NAND, firmware or proprietary game assets are bundled.
