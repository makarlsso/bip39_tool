use anyhow::{Context, Result};
use bip39_tool::{DEFAULT_WORD_COUNT, format_seed_base64, format_seed_hex, generate_mnemonic};
use clap::Parser;
use dialoguer::{Input, Password};

#[derive(Parser)]
#[command(name = "bip39-tool", about = "Generate a BIP39 mnemonic and seed")]
struct Cli {
    /// Prompt for word count and/or passphrase when they are not provided
    #[arg(long)]
    generate: bool,

    /// Number of words in the mnemonic (12, 18, or 24). Default is 24.
    #[arg(long, short)]
    words: Option<u8>,

    /// BIP39 extension passphrase for seed derivation (empty = none)
    #[arg(long, short)]
    password: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let word_count = resolve_word_count(cli.generate, cli.words)?;
    let passphrase = resolve_passphrase(cli.generate, cli.password)?;

    let mnemonic = generate_mnemonic(word_count).context("failed to generate mnemonic")?;
    let seed = mnemonic.to_seed(&passphrase);

    println!("Mnemonic Phrase:\n{mnemonic}");
    println!("\nSeed (HEX):\n{}", format_seed_hex(&seed));
    println!("\nSeed (Base64):\n{}", format_seed_base64(&seed));

    Ok(())
}

fn resolve_word_count(generate: bool, cli_words: Option<u8>) -> Result<u8> {
    match cli_words {
        Some(words) => bip39_tool::validate_word_count(words),
        None if generate => prompt_word_count(),
        None => Ok(DEFAULT_WORD_COUNT),
    }
}

fn resolve_passphrase(generate: bool, cli_password: Option<String>) -> Result<String> {
    match cli_password {
        Some(password) => Ok(password),
        None if generate => prompt_passphrase(),
        None => Ok(String::new()),
    }
}

fn prompt_word_count() -> Result<u8> {
    let count = Input::new()
        .with_prompt("How many words? (12, 18, or 24)")
        .default(DEFAULT_WORD_COUNT)
        .interact_text()
        .context("failed to read word count")?;

    bip39_tool::validate_word_count(count)
}

fn prompt_passphrase() -> Result<String> {
    Password::new()
        .with_prompt("BIP39 passphrase (leave empty for none)")
        .allow_empty_password(true)
        .interact()
        .context("failed to read passphrase")
}
