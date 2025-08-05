use anyhow::{Context, Result};
use clap::ArgMatches;

use crate::{
    cli::{FILE_PATH_ID, PUBLIC_KEY_PATH_ID, SIGNATURE_PATH_ID},
    cryptography::signature::verify_file_with_signature,
    util::{parse_path_arg, parse_signature_algorithm_arg},
};

pub fn verify_signature_command(args: &ArgMatches) -> Result<()> {
    let signature = parse_signature_algorithm_arg(args)?;
    let file_path = parse_path_arg(args, FILE_PATH_ID)?;
    let signature_path = parse_path_arg(args, SIGNATURE_PATH_ID)?;
    let public_key_path = parse_path_arg(args, PUBLIC_KEY_PATH_ID)?;

    let file_content =
        std::fs::read(file_path).context("Failed to read file content for verification")?;
    let sig_content = std::fs::read(signature_path)
        .context("Failed to read signature content for verification")?;
    let pub_key_content = std::fs::read(public_key_path)
        .context("Failed to read public key content for verification")?;

    let public_key = signature
        .public_key_from_bytes(&pub_key_content)
        .context("Provided public key is not valid")?;

    verify_file_with_signature(&file_content, &sig_content, &signature, public_key)
        .context("\x1b[31m Signature verification failed\x1b[0m")?;

    println!("Signature \x1b[32mverification succeeded\x1b[0m");

    Ok(())
}
