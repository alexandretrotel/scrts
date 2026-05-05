# Changelog

## [0.2.1] - 2026-05-05

### Changed

- Password prompts now show masked characters (dots) while typing instead of nothing
- Rename prompt no longer pre-fills the old name — start typing the new name directly
- Added a blank line before all confirmation messages for easier reading
- Updated `dirs` from v5 to v6

## [0.2.0] - 2026-05-05

### Added

- `rename` command — interactively select a secret and give it a new name
- `set` command — interactively select a secret and overwrite its value

## [0.1.0] - 2026-05-05

### Added

- `add` command — prompt for a name and secret, store in OS native keyring
- `list` command — print all stored secret names
- `delete` command — interactively select and delete one or more secrets
- `copy` command — select a secret and copy it to the clipboard
- Cross-platform support: macOS Keychain, Windows Credential Manager, Linux Secret Service
- Registry of entry names persisted at `~/.scrts.json`
