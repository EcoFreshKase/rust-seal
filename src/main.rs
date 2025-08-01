use anyhow::{Context, Result};

use rust_seal::cli::get_args;
use rust_seal::cryptography::signature::sign_and_save_file_signature;

fn main() -> Result<()> {
    let args = get_args().context("Failed to parse command line arguments")?;

    let keypair = args
        .signature
        .keypair()
        .context("Failed to generate keypair")?;

    sign_and_save_file_signature(&args.file_path, &args.signature, &keypair.1)
        .context("Failed to sign and save file signature")?;

    println!("{}", args.signature.algorithm().name());

    Ok(())
}
