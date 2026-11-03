# Contributing

## Scope

Contributions should improve Wii Sports Resort Enhanced as a game-specific launcher and Dolphin integration layer. Keep emulator-engine changes in NVDEMU/dolphin unless they are clearly launcher-specific and maintained in this repository.

## Before opening a pull request

- Describe the user-facing problem and expected behavior.
- Include operating system and Dolphin version for integration changes.
- Add tests for parsing, path handling, profile migration, or process management.
- Do not claim a feature is supported until it has been tested against the targeted Dolphin build.
- Update docs/ROADMAP.md when milestone scope or acceptance criteria change.

## Code safety

- Use Rust for privileged filesystem and process operations.
- Do not invoke a shell to launch Dolphin.
- Validate all user-supplied paths and archive contents.
- Do not execute scripts from mods or archives.
- Use atomic configuration writes and preserve existing data on failures.
- Keep logs useful but redact personal paths and other sensitive details from exported diagnostics.

## Never commit

- Wii Sports Resort game images or extracted game files.
- Wii NAND images, firmware, keys, or BIOS files.
- Proprietary Nintendo artwork, music, fonts, or other assets without permission.
- Third-party texture packs or mods without redistribution permission.
- Personal save files, state files, or controller pairing information.
- Build outputs, signing keys, certificates, or secrets.

## Pull request checklist

- [ ] Builds on the affected platform(s).
- [ ] Formatting and lint checks pass.
- [ ] Tests added or updated.
- [ ] No copyrighted or personal files included.
- [ ] User data remains outside the install directory.
- [ ] Documentation and roadmap updated.
