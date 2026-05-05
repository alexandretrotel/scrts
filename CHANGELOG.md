# Changelog

## [0.1.0] - 2026-05-05

### Added

- `add` command — prompt for a name and secret, store in OS native keyring
- `list` command — print all stored secret names
- `delete` command — interactively select and delete one or more secrets
- `copy` command — select a secret and copy it to the clipboard
- Cross-platform support: macOS Keychain, Windows Credential Manager, Linux Secret Service
- Registry of entry names persisted at `~/.scrts.json`
