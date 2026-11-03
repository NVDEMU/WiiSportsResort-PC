# Mod Manager Design and Current Behavior

## Riivolution

The manager imports ZIP archives containing a Riivolution XML patch definition and its replacement files. It validates archive paths, rejects symlinks, checks for a `wiidisc` XML root, reads game prefix/name metadata and installs into:
`User/Load/Riivolution/<generated UUID>`

Enable/disable moves the app-managed UUID folder between Load/Riivolution and ModStash. Removal deletes only the generated folder owned by the app.

**Activation limitation:** Dolphin's desktop workflow still requires users to right-click the title in the game list, choose **Start with Riivolution Patches**, then choose the XML and options. The launcher's normal `--exec` action does not select the patch set.

## Dolphin Resource Packs and HD textures

The importer extracts the ZIP into a temporary validation directory, requires a root-level `manifest.json` with a `name` field, rejects unsafe paths and symlinks, and enforces file-count/expanded-size limits. It then stores the original ZIP under:
`User/ResourcePacks/<generated UUID>.zip`

The original ZIP is retained because Dolphin loads resource-pack archives from its ResourcePacks folder. Active packs cause the launcher to enable Dolphin's `HiresTextures` and `CacheHiresTextures` options.

## Gecko codes

Codes are validated as lines of two eight-character hexadecimal tokens and written to:
`User/GameSettings/<GAMEID>.ini`
under `[Gecko]`. A comment-delimited managed block allows later saves to replace only the app-owned codes. Profiles with enabled codes launch Dolphin with cheats enabled.

## Controller source

The manager writes `WiimoteSource0` into `[Controls]` in the same GameINI:
- 1 = Emulated Wii Remote
- 2 = Real Wii Remote

Detailed controller mappings and MotionPlus/Nunchuk attachments still require Dolphin's controller UI.

## Archive safety

- Extract to a temporary staging directory.
- Reject traversal/absolute paths and symlinks.
- Limit file count and expanded size.
- Validate XML/JSON before installation.
- Use generated UUIDs for installed paths.
- Never execute archive contents.
- Remove only generated app-managed folders/files.

## Not implemented yet

- Dependency/conflict graph and mod priority UI.
- Direct Riivolution preset selection from the launcher.
- Automatic region/title compatibility verification for every mod.
- Resource-pack format conformance tests for every supported version.
- Automatic MotionPlus/Nunchuk mapping.
