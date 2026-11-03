# Dolphin Integration

## Current launch contract

Dolphin's documented CLI includes:
- `--version`
- `--user <folder>`
- `--exec <game image>`
- `--config=<System>.<Section>.<Key>=<Value>`

The Rust backend passes each argument separately. No shell is used.

## Applied per-run graphics settings

- `GFX.Settings.InternalResolution`
- `GFX.Settings.MaxInternalResolution`
- `GFX.Settings.MSAA`
- `GFX.Enhancements.MaxAnisotropy` (mapped to enum values: 1×=0, 2×=1, 4×=2, 8×=3, 16×=4)
- `GFX.Enhancements.ForceTextureFiltering=2` (Linear)
- `GFX.Hacks.VertexRounding`
- `GFX.Settings.ShowFPS`
- `GFX.Settings.BorderlessFullscreen`
- `GFX.Settings.HiresTextures` and `GFX.Settings.CacheHiresTextures` when a resource pack is active
- `Core.EnableCheats` when the profile has enabled Gecko codes

The key names are verified against Dolphin's current GraphicsSettings.cpp and VideoConfig.h.

## Controller configuration

The launcher writes a per-game GameINI:
- File: `User/GameSettings/<GAMEID>.ini`
- Section: `[Controls]`
- `WiimoteSource0 = 1` for Emulated Wii Remote
- `WiimoteSource0 = 2` for Real Wii Remote

The source values correspond to Dolphin's game-specific controller override. For compressed images without a readable raw header, the user must confirm the six-character Game ID in launcher Settings.

Detailed Wii Remote mapping, MotionPlus/Nunchuk attachment and device binding are left to Dolphin's controller UI. The launcher stores those preferences but does not claim to apply a custom motion mapping yet.

## User folders

Each profile has a stable path:
`AppData/DolphinProfiles/<profile UUID>/User`

That path is passed to Dolphin with `--user`. Saves, cheats, texture packs, GameSettings and Riivolution patches live outside the launcher install.

## Riivolution workflow

Riivolution archives are imported into:
`User/Load/Riivolution/<generated mod UUID>`

The XML is validated and metadata is extracted. To activate a patch, open Dolphin's game list, right-click Wii Sports Resort, choose **Start with Riivolution Patches**, then select the installed XML and patch options. The launcher's standard `--exec` action does not select a Riivolution preset.

## Resource packs

The archive is validated in a temporary staging directory, including a root-level `manifest.json`, then the original ZIP is stored in `User/ResourcePacks/<generated UUID>.zip`. Dolphin's high-resolution texture loading and cache are enabled for profiles with active packs.

## 8K caveat

12× is Dolphin's maximum internal-resolution setting in the current engine. This is not the same as a forced 7680×4320 window/display output mode. Final output and performance depend on display, OS, graphics backend and GPU.
