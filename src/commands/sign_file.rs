use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::ArgMatches;
use oqs::sig::{PublicKey as SigPublicKey, Sig};

use crate::{
    cli::FILE_PATH_ID,
    cryptography::signature::sign_and_save_file_signature,
    util::{create_file_with_content, parse_path_arg, parse_signature_algorithm_arg},
};

pub fn sign_file_command(args: &ArgMatches) -> Result<()> {
    let signature = parse_signature_algorithm_arg(args)?;
    let file_path = parse_path_arg(args, FILE_PATH_ID)?;

    sign_file(file_path, &signature).context("Failed to sign file")?;

    Ok(())
}

fn sign_file(file_path: &PathBuf, signature: &Sig) -> Result<()> {
    let keypair = signature.keypair().context("Failed to generate keypair")?;

    sign_and_save_file_signature(file_path, signature, &keypair.1)
        .context("Failed to sign and save file signature")?;
    println!(
        "Signature file created successfully: {}",
        file_path.with_extension("sig").display()
    );

    save_pub_key(&file_path.with_extension("pub"), &keypair.0)
        .context("Failed to save public key")?;
    println!(
        "Public key created successfully: {}",
        file_path.with_extension("pub").display()
    );

    Ok(())
}

fn save_pub_key(file_path: &PathBuf, public_key: &SigPublicKey) -> Result<()> {
    create_file_with_content(file_path, public_key.as_ref())
        .context("Failed to save public key")?;
    Ok(())
}
