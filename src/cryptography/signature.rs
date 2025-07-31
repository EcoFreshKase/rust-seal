use std::path::PathBuf;
use std::{fs::File, io::Write};

use oqs::sig::{SecretKey as SigSecretKey, Sig, Signature};

use crate::error::RustSealError;

pub fn sign_and_save_file_signature(
    file_path: &PathBuf,
    signature: &Sig,
    secret_key: &SigSecretKey,
) -> Result<(), RustSealError> {
    let signature = get_signature(file_path, signature, secret_key)?;

    // save file
    let signature_file_path = file_path.with_extension("sig");
    let mut file = File::create_new(&signature_file_path)?;

    file.write_all(signature.as_ref())?;

    Ok(())
}

fn get_signature(
    file_path: &PathBuf,
    signature: &Sig,
    secret_key: &SigSecretKey,
) -> Result<Signature, RustSealError> {
    let file_content = std::fs::read(file_path)?;

    signature
        .sign(&file_content, secret_key)
        .map_err(Into::into)
}
