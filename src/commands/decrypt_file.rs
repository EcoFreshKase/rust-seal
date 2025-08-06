use std::fs::{read, write};

use anyhow::{Context, Result};
use clap::ArgMatches;
use oqs::kem::{CiphertextRef, Kem, SecretKey as KemSecretKey};

use crate::{
    Config,
    cli::{CIPHER_TEXT_PATH_ID, FILE_PATH_ID},
    cryptography::symmetric::symmetric_decrypt,
    util::{parse_kem_algorithm_arg, parse_path_arg},
};

pub fn decrypt_file_command(sub_matches: &ArgMatches, config: &Config) -> Result<()> {
    let kem = parse_kem_algorithm_arg(sub_matches)?;
    let file_path = parse_path_arg(sub_matches, FILE_PATH_ID)?;
    let ciphertext_path = parse_path_arg(sub_matches, CIPHER_TEXT_PATH_ID)
        .unwrap_or_else(|_| file_path.with_extension("cipher"));

    let (_, secret_key) = config.get_kem_keys(&kem.algorithm()).context(
        "Failed to retrieve KEM secret key from configuration. KEM Algorithm is not initialized.",
    )?;
    let file_content = read(&file_path).context("Failed to read file content for decryption.")?;
    let ciphertext_bytes = read(&ciphertext_path).context("Failed to read ciphertext file.")?;
    let ciphertext = kem
        .ciphertext_from_bytes(&ciphertext_bytes)
        .context("Failed to parse ciphertext from bytes. Ensure the file is a valid ciphertext.")?;

    let decrypted_data = decrypt_file(kem, ciphertext, secret_key, &file_content)
        .context("Failed to decrypt file content with KEM algorithm and provided ciphertext.")?;

    //removes the extension from the path (file should have multiple extensions => test.txt.cipher)
    write(file_path.with_extension(""), decrypted_data)
        .context("Failed to write decrypted data to file.")?;

    println!(
        "Decrypted data written to {}\nDecryption successful",
        &file_path.display()
    );

    Ok(())
}

fn decrypt_file(
    kem: Kem,
    ciphertext: CiphertextRef,
    secret_key: KemSecretKey,
    decrypted_data: &[u8],
) -> Result<Vec<u8>> {
    let shared_secret = kem.decapsulate(&secret_key, ciphertext)
        .context("Failed to decapsulate shared secret using KEM algorithm. Secret Key or Ciphertext might be incorrect or Algorithm might be disabled.")?;
    symmetric_decrypt(decrypted_data, shared_secret.as_ref())
        .context("Failed to decrypt file content with shared secret")
}
