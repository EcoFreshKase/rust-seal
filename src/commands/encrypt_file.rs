use std::fs::{read, write};

use anyhow::{Context, Result};
use clap::ArgMatches;
use oqs::kem::{Ciphertext, Kem, PublicKeyRef as KemPublicKeyRef};

use crate::{
    cli::{FILE_PATH_ID, PUBLIC_KEY_PATH_ID},
    cryptography::symmetric::symmetric_encrypt,
    util::{parse_kem_algorithm_arg, parse_path_arg},
};

pub fn encrypt_file_command(args: &ArgMatches) -> Result<()> {
    let kem = parse_kem_algorithm_arg(args)?;
    let file_path = parse_path_arg(args, FILE_PATH_ID)?;
    let public_key_path = parse_path_arg(args, PUBLIC_KEY_PATH_ID)
        .unwrap_or_else(|_| file_path.with_extension("pub"));

    let public_key_content =
        read(&public_key_path).context("Failed to read public key content for encryption")?;
    let public_key = kem
        .public_key_from_bytes(&public_key_content)
        .context("Failed to read or parse public key")?;

    let file_content = read(&file_path).context("Failed to read file content for encryption")?;

    let (ciphertext, decrypted_data) = encrypt_file(kem, public_key, &file_content)?;

    let ciphertext_path = file_path.with_extension("cipher");
    write(&ciphertext_path, ciphertext.as_ref()).context("Failed to write ciphertext to file")?;
    println!("Ciphertext written to {}", &ciphertext_path.display());

    let decrypted_data_path = file_path.with_extension("dec");
    write(&decrypted_data_path, decrypted_data)
        .context("Failed to write decrypted data to file")?;
    println!(
        "Decrypted data written to {}",
        &decrypted_data_path.display()
    );

    Ok(())
}

fn encrypt_file(
    kem: Kem,
    public_key: KemPublicKeyRef,
    file_content: &[u8],
) -> Result<(Ciphertext, Vec<u8>)> {
    let (ciphertext, shared_secret) = kem
        .encapsulate(public_key)
        .context("Failed to encapsulate using KEM algorithm. Algorithm might be disabled.")?;

    let decrypted_data = symmetric_encrypt(file_content, shared_secret.as_ref())
        .context("Failed to encrypt file content with shared secret")?;

    Ok((ciphertext, decrypted_data))
}
