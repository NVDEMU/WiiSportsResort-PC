# Mod Manager Design

## Supported mod families

### Riivolution

The manager should preserve original Riivolution files and directory structures. It should inspect the patch XML, enumerate patch IDs/options, validate referenced paths, and present the user with selectable options.

Important: XML parsing and UI selection do not themselves activate patches. The manager must use the mechanism supported by the selected Dolphin build. If the CLI or existing Dolphin UI cannot select a patch set deterministically, add a small, tested integration feature to the Dolphin fork.

Never rewrite game images to install a Riivolution patch. Keep source patch files and the selected activation configuration separate.

### Dolphin resource packs and custom textures

Support the Dolphin Resource Pack specification:
- ZIP archive with a manifest and textures directory.
- Game-ID directories inside textures.
- Validate required manifest fields.
- Recognize the compressed flag and format version.
- Preserve pack identity, version, authors, and website metadata.
- Offer stable priority ordering and conflict previews.

Also support legacy custom texture folders under the managed Dolphin user directory. Resource packs and manually installed textures should be distinguishable in the UI.

### Gecko and Action Replay codes

Use Dolphin's supported per-game code configuration where available. Show code origin and description, allow explicit enable/disable, and avoid silently downloading or enabling codes.

## Common metadata model

Each imported mod receives an app-side record:

- Internal UUID.
- Display name and description.
- Mod family.
- Version and author metadata, if supplied.
- Source path and content hash.
- Supported game IDs/regions.
- Declared dependencies/conflicts, if available.
- Selected options.
- Enable state per profile.
- Import time and last validation result.

Do not force all formats into a single rewritten archive. Preserve original data and store only app-managed metadata/selection state separately.

## Installation and rollback

- Import into a staging directory.
- Inspect the archive before extraction.
- Reject absolute paths, parent-directory traversal, unsafe symlinks, excessive file counts, and unreasonable uncompressed sizes.
- Validate XML/JSON before activation.
- Compute hashes and detect duplicate content.
- Create a backup or activation manifest before changing managed files.
- Apply changes atomically where possible.
- If activation fails, restore the previous known-good state.
- Never delete files the manager cannot prove it owns.

## Conflict handling

Show:
- Same-path texture collisions.
- Duplicate patch IDs.
- Conflicting patch options.
- Region/game-ID mismatch.
- Dependencies not installed.
- Mod family unsupported by the selected Dolphin version.

Do not silently resolve semantic conflicts. For texture collisions, allow explicit priority ordering and explain which pack wins.

## Distribution

The app may link users to mod authors' official pages. Do not mirror or bundle third-party game modifications unless redistribution permission is explicit. Do not include copyrighted game files or textures in releases.
