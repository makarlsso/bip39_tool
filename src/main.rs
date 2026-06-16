use anyhow::{Context, Result};
use bip39_tool::{DEFAULT_WORD_COUNT, format_seed_base64, format_seed_hex, generate_mnemonic};
use clap::Parser;
use dialoguer::{Input, Password};

#[derive(Parser)]
#[command(name = "bip39", about = "Generate a BIP39 mnemonic and seed")]
struct Cli {
    /// Number of words in the mnemonic (12, 18, or 24). When prompted, default is 24.
    #[arg(long, short)]
    words: Option<u8>,

    /// Optional BIP39 passphrase for seed derivation (empty = no passphrase)
    #[arg(long, short)]
    password: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let word_count = resolve_word_count(cli.words)?;
    let passphrase = resolve_passphrase(cli.password)?;

    let mnemonic = generate_mnemonic(word_count).context("failed to generate mnemonic")?;
    let seed = mnemonic.to_seed(&passphrase);

    println!("Mnemonic Phrase:\n{mnemonic}");
    println!("\nSeed (HEX):\n{}", format_seed_hex(&seed));
    println!("\nSeed (Base64):\n{}", format_seed_base64(&seed));

    Ok(())
}

fn resolve_word_count(cli_words: Option<u8>) -> Result<u8> {
    let count = match cli_words {
        Some(words) => words,
        None => Input::new()
            .with_prompt("How many words? (12, 18, or 24)")
            .default(DEFAULT_WORD_COUNT)
            .interact_text()
            .context("failed to read word count")?,
    };

    bip39_tool::validate_word_count(count)
}

fn resolve_passphrase(cli_password: Option<String>) -> Result<String> {
    match cli_password {
        Some(password) => Ok(password),
        None => Password::new()
            .with_prompt("BIP39 passphrase (leave empty for none)")
            .allow_empty_password(true)
            .interact()
            .context("failed to read passphrase"),
    }
}
