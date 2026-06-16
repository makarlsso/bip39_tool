# bip39-tool

![Rust Edition](https://img.shields.io/badge/edition-2024-orange?style=flat-square&logo=rust)
![MSRV](https://img.shields.io/badge/MSRV-1.85%2B-orange?style=flat-square&logo=rust)
![Version](https://img.shields.io/badge/version-1.0.3-blue?style=flat-square)
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

The executable is `target/release/bip39-tool`.

## Run

### Quick generate (default)

Run with no flags. Uses **24 words** and **no passphrase**. No prompts.

```bash
bip39-tool
```

### Interactive generate

Use `--generate` to prompt for any option you omit:

```bash
bip39-tool --generate
```

Prompts for word count (Enter accepts **24**) and passphrase (Enter accepts none).

Provide one flag to prompt only for the other:

```bash
bip39-tool --generate --words 12
bip39-tool --generate --password "my extension passphrase"
```

Provide both flags to skip all prompts:

```bash
bip39-tool --generate --words 12 --password ""
```

### Direct flags (no prompts)

Pass `--words` and/or `--password` without `--generate`. Omitted options use defaults (**24** words, no passphrase). No prompts.

```bash
bip39-tool --words 12
bip39-tool --password "my extension passphrase"
bip39-tool --words 18 --password ""
```

## Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--generate` | | Prompt for `--words` and/or `--password` when omitted |
| `--words` | `-w` | Word count: `12`, `18`, or `24`. Default is `24`. |
| `--password` | `-p` | BIP39 extension passphrase. Default is none. |

**Behavior**

| Invocation | Words | Passphrase | Prompts |
|------------|-------|------------|---------|
| `bip39-tool` | 24 | none | No |
| `bip39-tool --words 12` | 12 | none | No |
| `bip39-tool --generate` | prompted | prompted | Yes |
| `bip39-tool --generate --words 12` | 12 | prompted | Passphrase only |

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
- **Passphrase input is hidden** when prompted with `--generate`.
- **The passphrase is not the mnemonic.** It is an optional BIP39 extension passphrase (sometimes called the "25th word") used only during seed derivation.

## Develop

```bash
cargo test
cargo build --release
```
