use anyhow::{Context, Result, anyhow, bail};
use base64::{Engine, engine::general_purpose::STANDARD};
use bip39::{Language, Mnemonic};
use getrandom::fill;

pub fn entropy_byte_count(word_count: usize) -> usize {
    (word_count / 3) * 4
}

pub fn validate_word_count(count: u8) -> Result<u8> {
    match count {
        12 | 18 | 24 => Ok(count),
        _ => bail!("word count must be 12, 18, or 24, got {count}"),
    }
}

pub fn generate_mnemonic(word_count: u8) -> Result<Mnemonic> {
    let word_count = validate_word_count(word_count)?;
    let mut entropy = vec![0u8; entropy_byte_count(word_count as usize)];
    fill(&mut entropy).map_err(|err| anyhow!("failed to read entropy from OS: {err:?}"))?;
    Mnemonic::from_entropy_in(Language::English, &entropy).context("failed to generate mnemonic")
}

pub fn format_seed_hex(seed: &[u8; 64]) -> String {
    hex::encode(seed)
}

pub fn format_seed_base64(seed: &[u8; 64]) -> String {
    STANDARD.encode(seed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use bip39::Mnemonic;

    #[test]
    fn validate_word_count_accepts_standard_lengths() {
        for count in [12, 18, 24] {
            assert_eq!(validate_word_count(count).unwrap(), count);
        }
    }

    #[test]
    fn validate_word_count_rejects_invalid_lengths() {
        for count in [0, 11, 15, 25, 255] {
            let err = validate_word_count(count).unwrap_err();
            assert!(
                err.to_string().contains("word count must be 12, 18, or 24"),
                "unexpected error for {count}: {err}"
            );
        }
    }

    #[test]
    fn seed_hex_and_base64_round_trip() {
        let seed = [0xAB; 64];
        let hex = format_seed_hex(&seed);
        assert_eq!(hex::decode(hex).unwrap(), seed.as_slice());

        let b64 = format_seed_base64(&seed);
        assert_eq!(STANDARD.decode(b64).unwrap(), seed.as_slice());
    }

    #[test]
    fn mnemonic_from_entropy_has_expected_word_count() {
        let cases = [(16, 12), (24, 18), (32, 24)];

        for (entropy_len, expected_words) in cases {
            let entropy = vec![0x7f; entropy_len];
            let mnemonic =
                Mnemonic::from_entropy_in(Language::English, &entropy).expect("valid entropy");
            assert_eq!(mnemonic.word_count(), expected_words);
        }
    }

    #[test]
    fn seed_derivation_matches_bip39_test_vector_without_passphrase() {
        let entropy = hex::decode("7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f").unwrap();
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
        assert_eq!(
            mnemonic.to_string(),
            "legal winner thank year wave sausage worth useful legal winner thank yellow"
        );

        let seed = mnemonic.to_seed("");
        assert_eq!(
            format_seed_hex(&seed),
            "878386efb78845b3355bd15ea4d39ef97d179cb712b77d5c12b6be415fffeffe5f377ba02bf3f8544ab800b955e51fbff09828f682052a20faa6addbbddfb096"
        );
    }

    #[test]
    fn seed_derivation_matches_bip39_test_vector_with_passphrase() {
        let entropy = hex::decode("7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f").unwrap();
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();

        let seed = mnemonic.to_seed_normalized("TREZOR");
        assert_eq!(
            format_seed_hex(&seed),
            "2e8905819b8723fe2c1d161860e5ee1830318dbf49a83bd451cfb8440c28bd6fa457fe1296106559a3c80937a1c1069be3a3a5bd381ee6260e8d9739fce1f607"
        );
    }

    #[test]
    fn passphrase_changes_derived_seed() {
        let entropy = hex::decode("7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f").unwrap();
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();

        assert_ne!(mnemonic.to_seed(""), mnemonic.to_seed("TREZOR"));
    }

    #[test]
    fn generated_mnemonic_word_count_matches_request() {
        for count in [12, 18, 24] {
            let mnemonic = generate_mnemonic(count).unwrap();
            assert_eq!(mnemonic.word_count(), count as usize);
        }
    }
}
