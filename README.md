# bip39

![Rust Edition](https://img.shields.io/badge/edition-2024-orange?style=flat-square&logo=rust)
![MSRV](https://img.shields.io/badge/MSRV-1.85%2B-orange?style=flat-square&logo=rust)
![Version](https://img.shields.io/badge/version-1.0.0-blue?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)
![BIP39](https://img.shields.io/badge/BIP39-compliant-lightgrey?style=flat-square)

Generate a random BIP39 mnemonic and print the derived seed to stdout.

## What it does

1. Creates a new mnemonic with **12**, **18**, or **24** English words.
2. Derives the BIP39 seed using an optional extension passphrase.
3. Prints the mnemonic, seed (hex), and seed (base64).

Entropy comes from the operating system (`getrandom`). No files are read or written.

## Install

Build the release binary:

```bash
cargo build --release
```

The executable is `target/release/bip39`.

## Run

### Interactive mode

Run without flags. The tool asks for word count and passphrase:

```bash
bip39
```

### Partial flags

Pass word count on the command line. The tool still prompts for the passphrase:

```bash
bip39 --words 24
```

### Non-interactive mode

Pass both flags to skip all prompts:

```bash
bip39 --words 12 --password "my extension passphrase"
```

Use an empty password for no extension passphrase:

```bash
bip39 --words 12 --password ""
```

## Flags

| Flag | Short | Required | Description |
|------|-------|----------|-------------|
| `--words` | `-w` | No | Word count: `12`, `18`, or `24`. Prompted if omitted. |
| `--password` | `-p` | No | BIP39 extension passphrase. Prompted if omitted. Use `""` for none. |

**Prompt rules**

- Omit `--words` → the tool asks for `12`, `18`, or `24`.
- Omit `--password` → the tool asks for a passphrase (hidden input; press Enter for none).
- Provide both flags → no prompts.

## Output

Stdout contains three labeled sections:

```
Mnemonic Phrase:
<word1> <word2> ... <wordN>

Seed (HEX):
<128-character hex string>

Seed (Base64):
<base64 string>
```

The seed is always 64 bytes. Hex and base64 are two encodings of the same value.

## Security

- **Treat output as secret.** Anyone with the mnemonic or seed can control derived wallets.
- **Use a trusted machine.** Do not run on shared or remote systems you do not control.
- **Passphrase input is hidden** when prompted interactively.
- **The passphrase is not the mnemonic.** It is an optional BIP39 extension passphrase (sometimes called the "25th word") used only during seed derivation.

## Develop

```bash
cargo test
cargo build --release
```
