use std::path::PathBuf;

use anyhow::{Context, Result, ensure};
use clap::ArgMatches;
use oqs::sig::{Algorithm as SigAlgorithm, PublicKey as SigPublicKey, Sig};

use crate::{
    cryptography::signature::sign_and_save_file_signature, util::create_file_with_content,
};

// pub struct SignFileArgs {
//     pub file_path: PathBuf,
//     pub signature: Sig,
// }

pub fn sign_file_command(args: &ArgMatches) -> Result<()> {
    let file_path = args
        .get_one::<PathBuf>("file_path")
        .context("File path argument is invalid")
        .and_then(|path| {
            ensure!(
                path.is_file(),
                "The specified file does not exist or is not a file"
            );
            Ok(path)
        })?;

    let signature = args
        .get_one::<SigAlgorithm>("signature_algorithm")
        .context("Signature algorithm argument is invalid")
        .and_then(|algorithm| {
            Sig::new(*algorithm).context("Signature algorithm argument is invalid")
        })?;

    sign_file(file_path, &signature)?;

    Ok(())
}

fn sign_file(file_path: &PathBuf, signature: &Sig) -> Result<()> {
    let keypair = signature.keypair().context("Failed to generate keypair")?;

    sign_and_save_file_signature(file_path, signature, &keypair.1)
        .context("Failed to sign and save file signature")?;

    save_pub_key(&file_path.with_extension("pub"), &keypair.0)
        .context("Failed to save public key")?;

    println!("{}", signature.algorithm().name());

    Ok(())
}

fn save_pub_key(file_path: &PathBuf, public_key: &SigPublicKey) -> Result<()> {
    create_file_with_content(file_path, public_key.as_ref())
        .context("Failed to save public key")?;
    Ok(())
}
