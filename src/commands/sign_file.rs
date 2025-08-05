use std::{fs::write, path::PathBuf};

use anyhow::{Context, Result};
use clap::ArgMatches;
use oqs::sig::{PublicKey as SigPublicKey, SecretKey as SigSecretKey, Sig};

use crate::{
    Config,
    cli::FILE_PATH_ID,
    cryptography::signature::get_signature_from_file,
    util::{parse_path_arg, parse_signature_algorithm_arg},
};

pub fn sign_file_command(args: &ArgMatches, config: &mut Config) -> Result<()> {
    let signature = parse_signature_algorithm_arg(args)?;
    let file_path = parse_path_arg(args, FILE_PATH_ID)?;

    let (public_key, secret_key) = match config.get_signature_keys(&signature.algorithm()) {
        Ok(keys) => keys,
        Err(_) => {
            println!(
                "No keys found for signature algorithm '{}'. Generating new keys...",
                signature.algorithm()
            );
            config
                .add_signature_algorithm(&signature)
                .context("Failed to add signature algorithm")?
        }
    };

    sign_file(file_path, &signature, &public_key, &secret_key).context("Failed to sign file")?;

    Ok(())
}

fn sign_file(
    file_path: &PathBuf,
    signature: &Sig,
    public_key: &SigPublicKey,
    secret_key: &SigSecretKey,
) -> Result<()> {
    let file_signature = get_signature_from_file(file_path, signature, secret_key)
        .context("Failed to create signature from file")?;

    write(file_path.with_extension("sig"), file_signature.as_ref())
        .context("Failed to write signature to file")?;
    println!(
        "Signature file created successfully: {}",
        file_path.with_extension("sig").display()
    );

    write(file_path.with_extension("pub"), public_key.as_ref())
        .context("Failed to save public key")?;
    println!(
        "Public key created successfully: {}",
        file_path.with_extension("pub").display()
    );

    Ok(())
}
