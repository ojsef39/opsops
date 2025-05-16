# OpsOps

OpsOps is a wrapper around [SOPS (Secrets OPerationS)](https://github.com/mozilla/sops) that uses 1Password for key management, making it easier to handle encrypted files and secrets in your projects.

## Overview

OpsOps simplifies the process of encrypting, decrypting, and managing secrets by leveraging SOPS with 1Password integration. It handles key management automatically, allowing teams to securely share encrypted configuration without manual key distribution.

## Installation

```bash
# Nix
# todo

# Or using homebrew
brew install opsops
```

## Prerequisites

- [SOPS](https://github.com/mozilla/sops) installed on your system
- [1Password CLI](https://1password.com/downloads/command-line/) installed and configured

## Usage

```
opsops <COMMAND>
```

### Commands

- `list-config` - Parse and display the `.sops.yaml` for this project
- `generate-age-key` - Generate an age key pair
- `edit` - Edit a file using sops with a key from 1password
- `encrypt` - Encrypt a file using sops
- `decrypt` - Decrypt a file using sops
- `init` - Initialize opsops
- `help` - Print this message or the help of the given subcommand(s)

## Getting Started 

### 0. Create a .sops.yaml

See [[#Configuration]] on how to create a .sops.yaml. The 1Password item will be chosen in the next step


### 1. Generate an age key

```bash
opsops generate-age-key
```

This will generate a new age key pair and store the private key securely in your 1Password vault.

### 2. Initialize OpsOps in your project

```bash
opsops init
```

This command will:
- Create a `.sops.yaml` configuration file if it doesn't exist
- Set up 1Password integration
- Guide you through selecting the correct 1Password item

### 3. Encrypting a file

```bash
opsops encrypt config.json
```

### 4. Decrypting a file

```bash
opsops decrypt config.enc.json
```

### 5. Editing an encrypted file

```bash
opsops edit config.enc.json
```

This will decrypt the file, open it in your default editor, and re-encrypt it when you save and exit.

## Configuration

OpsOps uses the standard `.sops.yaml` configuration file format with additional options for 1Password integration.

Example `.sops.yaml`:

```yaml
creation_rules:
  - path_regex: .*.ya?ml
    age: age1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    encrypted_suffix: _secret
  - path_regex: .*.json
    age: age1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    encrypted_regex: "^(data|stringData)$"
    
# OpsOps specific configuration
onepassworditem: op://Personal/test/Private Key
```

## Working with Teams

OpsOps simplifies key management for teams by storing encryption keys in 1Password, which can be shared securely with team members through 1Password vaults.

To add a new team member:
1. Add them to the appropriate 1Password vault
2. They can now encrypt/decrypt files using OpsOps without additional configuration

## Environment Variables

- `OPSOPS_OP_VAULT` - Override the 1Password vault name
- `OPSOPS_OP_ITEM` - Override the 1Password item name
- `OPSOPS_AGE_KEY_FIELD` - Override the field name for the age key in 1Password
- `EDITOR` - The editor to use when editing files (defaults to system default)

## How It Works

OpsOps provides a simplified interface to SOPS while handling all the key management through 1Password:

1. When encrypting/decrypting, OpsOps retrieves the appropriate keys from 1Password
2. It temporarily makes the keys available to SOPS
3. SOPS performs the encryption/decryption operation
4. The keys are set as environment variables and never stored on disk

## Troubleshooting

### Common Issues

- **"1Password CLI not found"** - Install the 1Password CLI and make sure it's in your PATH
- **"Unable to access 1Password vault"** - Ensure you're signed in to 1Password CLI (`op signin`)
- **"Key not found in 1Password"** - Check your configuration and make sure the key exists in the specified vault/item

### Debug Mode

Run OpsOps with debug logging:

```bash
RUST_LOG=debug opsops <command>
```

## Development

- Pull this repo
- For ease of use there is a flake.nix included
- Simply run `direnv allow` and the flake should activate
- If you don't have nix installed you will need cargo and rust

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License

## Security

OpsOps is designed with security in mind:
- No keys are stored on disk in plaintext
- All key material is fetched from 1Password just-in-time

