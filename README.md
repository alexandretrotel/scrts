# scrts

A minimal CLI to store secrets using your OS native secret storage (macOS Keychain, Windows Credential Manager, Linux Secret Service).

## Install

```sh
cargo install scrts
```

## Usage

```sh
scrts add              # prompt for name and secret
scrts add --name foo   # skip the name prompt
scrts list             # print stored names
scrts delete           # interactively select and delete
scrts copy             # select a secret and copy it to clipboard
scrts rename           # select a secret and rename it
scrts replace          # select a secret and overwrite its value
```

Secrets are stored under the `scrts` service in your OS keyring. A registry of entry names is kept at `~/.scrts.json`.

## License

GPL-3.0. See [LICENSE](LICENSE).
